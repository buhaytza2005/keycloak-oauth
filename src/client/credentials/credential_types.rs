use std::fmt::Debug;

pub trait Credential: Debug + Send + Sync {}
#[derive(Debug, Clone)]
pub struct ResourceOwnerPasswordCredential {
    pub client_id: String,
    pub username: String,
    pub password: String,
}

impl Credential for ResourceOwnerPasswordCredential {}

impl ResourceOwnerPasswordCredential {
    pub fn new(
        username: impl Into<String>,
        password: impl Into<String>,
        client_id: impl Into<String>,
    ) -> ResourceOwnerPasswordCredential {
        ResourceOwnerPasswordCredential {
            client_id: client_id.into(),
            username: username.into(),
            password: password.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceCodeCredential {
    pub client_id: String,
    pub device_code: String,
    pub token_url: String,
}

impl Credential for DeviceCodeCredential {}
