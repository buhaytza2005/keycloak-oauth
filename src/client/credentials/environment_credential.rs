use std::{
    env::VarError,
    fmt::{Debug, Formatter},
};

use dotenv::dotenv;

use super::credential_types::ResourceOwnerPasswordCredential;
#[derive(Clone)]
pub struct EnvironmentCredential;

impl Debug for EnvironmentCredential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnvironmentCredential").finish()
    }
}

impl EnvironmentCredential {
    pub fn resource_owner_password_credential() -> Result<ResourceOwnerPasswordCredential, VarError>
    {
        EnvironmentCredential::try_username_password_env()
    }

    fn try_username_password_env() -> Result<ResourceOwnerPasswordCredential, VarError> {
        dotenv().ok();

        let client_id = std::env::var("KK_CLIENT_ID")?;
        let username = std::env::var("KK_USERNAME")?;
        let password = std::env::var("KK_PASSWORD")?;
        let creds = ResourceOwnerPasswordCredential {
            client_id: client_id.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        };

        Ok(creds)
    }
}
