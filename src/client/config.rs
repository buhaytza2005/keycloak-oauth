use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ClientConfiguration {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub auth_url: Option<String>,
    pub token_url: Option<String>,
    pub device_authorization_url: Option<String>,
    pub token_cache_path: Option<String>,
    pub jwks_url: Option<String>,
    pub realm: Option<String>,
    pub scopes: Vec<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl ClientConfiguration {
    pub fn from_env() -> Result<Self, envy::Error> {
        match envy::prefixed("KK_").from_env::<ClientConfiguration>() {
            Ok(config) => Ok(config),
            Err(e) => Err(e),
        }
    }
}
