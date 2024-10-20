use jsonwebtoken::TokenData;
use oauth2::{
    basic::BasicErrorResponseType, reqwest::async_http_client, ClientSecret,
    DeviceAuthorizationResponse, EmptyExtraDeviceAuthorizationFields, RequestTokenError,
    ResourceOwnerPassword, ResourceOwnerUsername, Scope, StandardErrorResponse,
    StandardTokenResponse, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::{fs, marker::PhantomData, path::Path, sync::Arc};
use thiserror::Error;
use tokio::sync::Mutex;

use oauth2::{
    basic::{BasicClient, BasicTokenType},
    AuthUrl, ClientId, DeviceAuthorizationUrl, EmptyExtraTokenFields, RefreshToken,
};

use crate::client::PollDeviceCodeEvent;

use super::{
    config::ClientConfiguration,
    jwks::{KeyCache, SharedKeyCache},
    verify_jwt, AppConfig, Claims, DeviceCodeCredential, ResourceOwnerPasswordCredential,
    VerifyJwtError, WithDeviceCredentials, WithOwnerCredentials,
};

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Token cache error: {0}")]
    TokenCacheError(#[from] serde_json::Error),

    #[error("OAuth2 Request Token error: {0}")]
    OAuth2RequestTokenError(
        #[from]
        RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JWT verification error: {0}")]
    JwtVerificationError(#[from] VerifyJwtError),

    #[error("No valid token available. Please authenticate.")]
    NoValidTokenError,

    #[error("Missing credentials for password grant. Check the KK_USER and KK_PASSWORD in your .env file")]
    NoPresentCredentialsError,
}

type MyStandardTokenResponse = oauth2::StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

pub struct KeycloakClient<C> {
    pub inner: BasicClient,
    pub config: ClientConfiguration,
    pub cache: SharedKeyCache,
    pub _marker: PhantomData<C>,
}
impl From<AppConfig<DeviceCodeCredential>> for KeycloakClient<WithDeviceCredentials> {
    fn from(value: AppConfig<DeviceCodeCredential>) -> Self {
        let config = ClientConfiguration::from_env();

        let token_url = if value.token_url.is_some() {
            value.token_url.clone().expect("token")
        } else {
            config
                .token_url
                .clone()
                .expect("No token_url in the config. Add token_url")
        };

        let inner = BasicClient::new(
            ClientId::new(value.client_id.clone()),
            Some(ClientSecret::new(
                config
                    .client_secret
                    .clone()
                    .expect("should have client secret"),
            )),
            AuthUrl::new(value.auth_url.clone()).expect("Invalid auth endpoint"),
            Some(TokenUrl::new(token_url).expect("Invalid token")),
        );
        let cache = Arc::new(Mutex::new(KeyCache::new()));

        KeycloakClient {
            inner,
            cache,
            config,
            _marker: PhantomData,
        }
    }
}
impl From<AppConfig<ResourceOwnerPasswordCredential>> for KeycloakClient<WithOwnerCredentials> {
    fn from(value: AppConfig<ResourceOwnerPasswordCredential>) -> Self {
        let config = ClientConfiguration::from_env();

        let token_url = if value.token_url.is_some() {
            value.token_url.clone().expect("token")
        } else {
            config
                .token_url
                .clone()
                .expect("No token_url in the config. Add token_url")
        };

        let inner = BasicClient::new(
            ClientId::new(value.client_id.clone()),
            None,
            AuthUrl::new(value.auth_url.clone()).expect("Invalid auth endpoint"),
            Some(TokenUrl::new(token_url).expect("Invalid token")),
        );
        let cache = Arc::new(Mutex::new(KeyCache::new()));

        KeycloakClient {
            inner,
            cache,
            config,
            _marker: PhantomData,
        }
    }
}
impl KeycloakClient<WithDeviceCredentials> {
    pub async fn initiate_device_flow(
        &self,
    ) -> Result<DeviceAuthorizationResponse<EmptyExtraDeviceAuthorizationFields>, ClientError> {
        let scopes = self
            .config
            .scopes
            .iter()
            .map(|s| Scope::new(s.clone()))
            .collect::<Vec<_>>();

        let device_auth_request = self
            .inner
            .clone()
            .set_device_authorization_url(
                DeviceAuthorizationUrl::new(
                    self.config
                        .device_authorization_url
                        .clone()
                        .expect("Device authorization url cannot be found. Check .env!"),
                )
                .expect("Invalid dev endpoint"),
            )
            .exchange_device_code()
            .expect("works?")
            .add_scopes(scopes)
            .request_async(async_http_client)
            .await
            .expect("device auth");

        println!(
            "Open this url {} \nand enter the code: {}",
            **device_auth_request.verification_uri(),
            device_auth_request.user_code().secret()
        );
        Ok(device_auth_request)
    }
    pub async fn poll_for_token(
        &self,
        device_auth_response: &oauth2::DeviceAuthorizationResponse<
            EmptyExtraDeviceAuthorizationFields,
        >,
    ) -> Result<MyStandardTokenResponse, ClientError> {
        let mut attempts = 0;
        let max_attempts = (device_auth_response.expires_in().as_secs()
            / device_auth_response.interval().as_secs()) as usize;

        let mut interval = device_auth_response.interval().as_secs();
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
            attempts += 1;

            match self
                .inner
                .exchange_device_access_token(device_auth_response)
                .request_async(
                    async_http_client,
                    tokio::time::sleep,
                    Some(tokio::time::Duration::from_secs(10)),
                )
                .await
            {
                Ok(token) => {
                    self.cache_token(&token)?;
                    return Ok(token);
                }
                Err(err) => match err {
                    RequestTokenError::ServerResponse(e) => {
                        let poll_event = PollDeviceCodeEvent::from(e);
                        poll_event.as_message();
                        match poll_event {
                            PollDeviceCodeEvent::AuthorizationPending => continue,
                            PollDeviceCodeEvent::AuthorizationDeclined => break,
                            PollDeviceCodeEvent::BadVerificationCode => continue,
                            PollDeviceCodeEvent::ExpiredToken => break,
                            PollDeviceCodeEvent::AccessDenied => break,
                            PollDeviceCodeEvent::SlowDown => {
                                interval += 5_u64;
                                continue;
                            }
                        }
                    }
                    _ => PollDeviceCodeEvent::AccessDenied.as_message(),
                },
            }
            if attempts >= max_attempts {
                eprintln!("Maximum polling attempts reached. Exiting.");
                break;
            }
        }

        Err(ClientError::OAuth2RequestTokenError(
            oauth2::RequestTokenError::Other("Polling timeout".into()),
        ))
    }
    pub async fn authenticate(&self) -> Result<MyStandardTokenResponse, ClientError> {
        let device_auth_response = self.initiate_device_flow().await?;

        let token = self.poll_for_token(&device_auth_response).await?;
        Ok(token)
    }
}

impl KeycloakClient<WithOwnerCredentials> {
    pub async fn initiate_password_flow(
        &self,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, ClientError> {
        if self.config.username.is_none() || self.config.password.is_none() {
            return Err(ClientError::NoPresentCredentialsError);
        };

        let username =
            ResourceOwnerUsername::new(self.config.username.clone().expect("Should have username"));
        let password =
            ResourceOwnerPassword::new(self.config.password.clone().expect("Should have password"));
        let scopes = self
            .config
            .scopes
            .iter()
            .map(|s| Scope::new(s.clone()))
            .collect::<Vec<_>>();
        let owner_credentials = self
            .inner
            .exchange_password(&username, &password)
            .add_scopes(scopes)
            .request_async(async_http_client)
            .await
            .expect("password grant");

        self.cache_token(&owner_credentials)?;

        Ok(owner_credentials)
    }
    pub async fn authenticate(&self) -> Result<MyStandardTokenResponse, ClientError> {
        let token = self.initiate_password_flow().await?;

        Ok(token)
    }
}

impl<C> KeycloakClient<C> {
    pub fn cache_token(
        &self,
        token: &oauth2::StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    ) -> Result<(), ClientError> {
        let expires_in = token
            .expires_in()
            .unwrap_or_else(|| std::time::Duration::from_secs(3600));
        let expires_at = chrono::Utc::now() + expires_in;

        let cached_token = CachedToken {
            access_token: token.access_token().secret().to_string(),
            expires_at,
            refresh_token: token.refresh_token().map(|rt| rt.secret().clone()),
        };

        let serialized = serde_json::to_string_pretty(&cached_token)?;
        std::fs::write(
            &self
                .config
                .token_cache_path
                .clone()
                .expect("Could not find the cache path in .env"),
            serialized,
        )?;
        Ok(())
    }
    pub fn load_cached_token(&self) -> Result<CachedToken, ClientError> {
        if !Path::exists(Path::new(
            &self
                .config
                .token_cache_path
                .clone()
                .expect("Could not find the cache path in .env"),
        )) {
            return Err(ClientError::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "token cache not found",
            )));
        }

        let data = fs::read_to_string(Path::new(
            &self
                .config
                .token_cache_path
                .clone()
                .expect("Could not find the cache path in .env"),
        ))?;
        let cached_token: CachedToken = serde_json::from_str(&data)?;
        Ok(cached_token)
    }

    /// Verifies the passed access token
    pub async fn verify_access_token(&self, token: &str) -> Result<TokenData<Claims>, ClientError> {
        let jwks_url = self
            .config
            .jwks_url
            .clone()
            .expect("Could not find jwks url, check .env");
        let client_id = self
            .config
            .client_id
            .clone()
            .expect("Cannot find client_id, token verification failed. Check .env");
        let realm = self
            .config
            .realm
            .clone()
            .expect("Cannot find realm, token verification failed. Check .env");
        verify_jwt(
            token,
            &jwks_url,
            self.cache.clone(),
            &[&client_id],
            &[&realm],
        )
        .await
        .map_err(ClientError::JwtVerificationError)
    }

    pub async fn verify_and_refresh_access_token(&self) -> Result<String, ClientError> {
        match self.load_cached_token() {
            Ok(cached_token) => {
                if cached_token.expires_at <= chrono::Utc::now() {
                    //token is expired
                    if let Some(refresh_token_str) = cached_token.refresh_token {
                        let new_token = match self
                            .inner
                            .exchange_refresh_token(&RefreshToken::new(refresh_token_str))
                            .request_async(async_http_client)
                            .await
                        {
                            Ok(token) => token,
                            Err(_e) => return Err(ClientError::NoValidTokenError),
                        };
                        self.cache_token(&new_token)?;
                        Ok(new_token.access_token().secret().clone())
                    } else {
                        Err(ClientError::NoValidTokenError)
                    }
                } else {
                    //token is still valid
                    Ok(cached_token.access_token)
                }
            }
            Err(e) => match &e {
                ClientError::IoError(error) => {
                    if error.kind() == std::io::ErrorKind::NotFound {
                        Err(ClientError::NoValidTokenError)
                    } else {
                        Err(e)
                    }
                }
                _ => Err(e),
            },
        }
    }
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct CachedToken {
    pub access_token: String,
    #[serde_as(as = "DisplayFromStr")]
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub refresh_token: Option<String>,
}

pub enum Flow {
    DeviceAuthorization,
    OwnerCredentials,
}
