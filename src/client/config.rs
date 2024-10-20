use std::collections::HashMap;

use dotenv::dotenv;
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
    pub fn from_env() -> Self {
        let names = vec![
            "client_id",
            "client_secret",
            "auth_url",
            "token_url",
            "device_authorization_url",
            "token_cache_path",
            "jwks_url",
            "realm",
            "username",
            "password",
            "scopes",
        ];
        let mut vars = HashMap::new();
        for v in &names {
            vars.insert(*v, format!("KK_{}", v.to_uppercase()));
        }

        dotenv().ok();
        let client_id: Option<String> = std::env::var(vars.get("client_id").expect("work")).ok();
        let client_secret: Option<String> =
            std::env::var(vars.get("client_secret").expect("work")).ok();
        let auth_url: Option<String> = std::env::var(vars.get("auth_url").expect("work")).ok();
        let token_url: Option<String> = std::env::var(vars.get("token_url").expect("work")).ok();
        let device_authorization_url: Option<String> =
            std::env::var(vars.get("device_authorization_url").expect("work")).ok();
        let token_cache_path: Option<String> =
            std::env::var(vars.get("token_cache_path").expect("work")).ok();
        let jwks_url: Option<String> = std::env::var(vars.get("jwks_url").expect("work")).ok();
        let realm: Option<String> = std::env::var(vars.get("realm").expect("work")).ok();
        let username: Option<String> = std::env::var(vars.get("username").expect("work")).ok();
        let password: Option<String> = std::env::var(vars.get("password").expect("work")).ok();

        let scopes = std::env::var("scopes").ok();
        let scopes = match scopes {
            Some(scopes_string) => {
                let parts = scopes_string
                    .split(",")
                    .collect::<Vec<_>>()
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                parts
            }
            None => Vec::new(),
        };

        ClientConfiguration {
            client_id,
            client_secret,
            auth_url,
            token_url,
            device_authorization_url,
            token_cache_path,
            jwks_url,
            realm,
            scopes,
            username,
            password,
        }
    }
}
