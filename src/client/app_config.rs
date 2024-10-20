use std::marker::PhantomData;

use super::{Credential, DeviceCodeCredential, ResourceOwnerPasswordCredential};

// base
pub struct NoCredentials;

// Builder after owner creds are set
pub struct WithOwnerCredentials;

// Builder after device creds are set
pub struct WithDeviceCredentials;

#[derive(Debug)]
pub struct AppConfig<C: Credential> {
    pub client_id: String,
    pub auth_url: String,
    pub token_url: Option<String>, // this is only needed for Device flow
    pub credential: C,
}

impl<C: Credential> AppConfig<C> {
    pub fn new(client_id: impl Into<String>, auth_url: impl Into<String>, credentials: C) -> Self {
        AppConfig {
            client_id: client_id.into(),
            auth_url: auth_url.into(),
            token_url: None,
            credential: credentials,
        }
    }
}

pub struct AppConfigBuilder<State, C: Credential> {
    client_id: Option<String>,
    auth_url: Option<String>,
    token_url: Option<String>,
    credential: Option<C>,
    _marker: PhantomData<State>,
}

impl AppConfigBuilder<NoCredentials, ResourceOwnerPasswordCredential> {
    pub fn new(client_id: impl Into<String>) -> Self {
        AppConfigBuilder {
            client_id: Some(client_id.into()),
            auth_url: None,
            token_url: None,
            credential: None,
            _marker: PhantomData,
        }
    }

    pub fn auth_url(mut self, auth_url: impl Into<String>) -> Self {
        self.auth_url.insert(auth_url.into());
        self
    }

    pub fn with_owner_credentials(
        self,
        credentials: ResourceOwnerPasswordCredential,
    ) -> AppConfigBuilder<WithOwnerCredentials, ResourceOwnerPasswordCredential> {
        AppConfigBuilder {
            client_id: self.client_id,
            auth_url: self.auth_url,
            token_url: self.token_url,
            credential: Some(credentials),
            _marker: PhantomData::<WithOwnerCredentials>,
        }
    }

    pub fn with_device_code_credentials(
        self,
        credentials: DeviceCodeCredential,
    ) -> AppConfigBuilder<WithDeviceCredentials, DeviceCodeCredential> {
        AppConfigBuilder {
            client_id: self.client_id,
            auth_url: self.auth_url,
            token_url: self.token_url,
            credential: Some(credentials),
            _marker: PhantomData::<WithDeviceCredentials>,
        }
    }
}

impl AppConfigBuilder<WithOwnerCredentials, ResourceOwnerPasswordCredential> {
    pub fn build(self) -> Result<AppConfig<ResourceOwnerPasswordCredential>, &'static str> {
        let client_id = self.client_id.ok_or("client_id is not set")?;
        let auth_url = self.auth_url.ok_or("auth_url is not set")?;
        let credential = self.credential.ok_or("Owner credentials not set")?;
        Ok(AppConfig {
            client_id,
            auth_url,
            token_url: None,
            credential,
        })
    }
}

impl AppConfigBuilder<WithDeviceCredentials, DeviceCodeCredential> {
    pub fn token_url(mut self, token_url: impl Into<String>) -> Self {
        self.token_url.insert(token_url.into());
        self
    }

    pub fn build(self) -> Result<AppConfig<DeviceCodeCredential>, &'static str> {
        let client_id = self.client_id.ok_or("client_id is not set")?;
        let auth_url = self.auth_url.ok_or("auth_url is not set")?;
        let token_url = self.token_url.ok_or("token_url is not set")?;
        let credentials = self.credential.ok_or("DeviceCredential not set")?;

        Ok(AppConfig {
            client_id,
            auth_url,
            token_url: Some(token_url),
            credential: credentials,
        })
    }
}
