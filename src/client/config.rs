use super::KeycloakClient;

pub struct ClientConfiguration {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub device_authorization_url: String,
    pub token_cache_path: String,
    pub jwks_url: String,
    pub realm: String,
    pub scopes: Vec<String>,
}

impl ClientConfiguration {
    pub fn build(self) -> KeycloakClient {
        KeycloakClient::new(self)
    }

    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Self {
            client_id: std::env::var("KK_CLIENT_ID").expect("Missing KK_CLIENT_ID"),
            client_secret: std::env::var("KK_CLIENT_SECRET").expect("Missing KK_CLIENT_SECRET"),
            auth_url: std::env::var("KK_AUTH_URL").expect("Missing KK_AUTH_URL"),
            token_url: std::env::var("KK_TOKEN_URL").expect("Missing KK_TOKEN_URL"),
            device_authorization_url: std::env::var("KK_DEVICE_AUTHORIZATION_URL")
                .expect("Missing KK_DEVICE_AUTHORIZATION_URL"),
            jwks_url: std::env::var("KK_JWKS_URL").expect("Missing KK_JWKS_URL"),
            token_cache_path: std::env::var("KK_TOKEN_CACHE_PATH")
                .expect("Missing KK_TOKEN_CACHE_PATH"),
            realm: std::env::var("KK_REALM").expect("Missing realm"),
            scopes: vec![
                "openid".to_string(),
                "email".to_string(),
                "offline_access".to_string(),
                "profile".to_string(),
                "roles".to_string(),
            ],
        }
    }
}
