use super::ResourceOwnerPasswordCredential;
use std::fmt::Debug;
use std::marker::PhantomData;

///We can use structs to show state on the main AppCOnfig and AppConfigBuilder
///To get the common functions running we can try a trait?
///
// declare all states
#[derive(Default, Debug)]
pub struct WithTokenUrl(String);
#[derive(Default)]
pub struct NoTokenUrl;

#[derive(Default, Debug)]
pub struct WithAuthUrl(String);
#[derive(Default)]
pub struct NoAuthUrl;
#[derive(Debug)]
pub struct Device;
#[derive(Debug)]
pub struct Owner(String);

#[derive(Debug)]
pub struct AppConfig<A, T, C> {
    //all flows will require a client_id
    pub client_id: String,
    pub auth_url: Option<A>,
    pub token_url: Option<T>,
    pub _credential_type: PhantomData<C>,
}

impl<A, T, C> AppConfig<A, T, C> {
    pub fn new(client_id: impl Into<String>) -> Self {
        AppConfig {
            client_id: client_id.into(),
            auth_url: None,
            token_url: None,
            _credential_type: PhantomData,
        }
    }
}

pub struct AppConfigBuilder<A, T, C> {
    //all flows will require a client_id
    pub client_id: String,
    pub auth_url: Option<A>,
    pub token_url: Option<T>,
    pub _credential_type: PhantomData<C>,
}
// can implement the builder pattern with type state? and the type state can just be the flow. this
// can them give access to the properties
//
//
impl<A: Default, T: Default, C> AppConfigBuilder<A, T, C> {
    pub fn new(client_id: impl Into<String>) -> Self {
        AppConfigBuilder {
            client_id: client_id.into(),
            auth_url: None,
            token_url: None,
            _credential_type: PhantomData,
        }
    }
}

// Owner

impl AppConfigBuilder<NoAuthUrl, NoTokenUrl, Owner> {
    pub fn token_url(
        self,
        token_url: impl Into<String>,
    ) -> AppConfigBuilder<NoAuthUrl, WithTokenUrl, Owner> {
        // insert reuses the existing Option, instead of creating a new one
        AppConfigBuilder {
            client_id: self.client_id,
            auth_url: self.auth_url,
            token_url: Some(WithTokenUrl(token_url.into())),
            _credential_type: PhantomData,
        }
    }
}

impl<T> AppConfigBuilder<NoAuthUrl, T, Owner> {
    pub fn auth_url(self, auth_url: impl Into<String>) -> AppConfigBuilder<WithAuthUrl, T, Owner> {
        AppConfigBuilder {
            client_id: self.client_id,
            auth_url: Some(WithAuthUrl(auth_url.into())),
            token_url: self.token_url,
            _credential_type: PhantomData::<Owner>,
        }
    }
}
impl AppConfigBuilder<WithAuthUrl, WithTokenUrl, Owner> {
    pub fn build(self) -> AppConfig<WithAuthUrl, WithTokenUrl, Owner> {
        AppConfig {
            client_id: self.client_id,
            auth_url: self.auth_url,
            token_url: self.token_url,
            _credential_type: PhantomData::<Owner>,
        }
    }
}

impl<WithAuthUrl, WithTokenUrl, C> AppConfigBuilder<WithAuthUrl, WithTokenUrl, C> {
    pub fn with_owner_credentials(
        self,
        owner_credentials: ResourceOwnerPasswordCredential,
    ) -> AppConfigBuilder<WithAuthUrl, WithTokenUrl, Owner> {
        AppConfigBuilder {
            client_id: self.client_id,
            auth_url: self.auth_url,
            token_url: self.token_url,
            _credential_type: PhantomData,
        }
    }
}
