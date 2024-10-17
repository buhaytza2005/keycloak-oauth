use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use super::VerifyJwtError;

//build key cache - this will be refreshed based on the implementation
#[derive(Debug)]
pub struct KeyCache {
    keys: HashMap<String, String>,
    last_updated: Instant,
    first: bool,
}
impl Default for KeyCache {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyCache {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            last_updated: Instant::now(),
            first: true,
        }
    }

    fn is_expired(&self) -> bool {
        self.last_updated.elapsed() > Duration::from_secs(24 * 60 * 60)
    }
}
pub type SharedKeyCache = Arc<tokio::sync::Mutex<KeyCache>>;

#[derive(Debug)]
pub enum FetchError {
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
    Lock(tokio::sync::TryLockError),
}
impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for FetchError {}

pub async fn fetch_and_cache_jwks(
    jwks_url: &str,
    cache: SharedKeyCache,
) -> Result<HashMap<String, String>, VerifyJwtError> {
    let mut cache_guard = cache.try_lock().map_err(FetchError::Lock)?;
    println!("cache guard before update: {:#?}", cache_guard);

    if cache_guard.first || cache_guard.is_expired() {
        let client = reqwest::Client::new();
        let resp = client
            .get(jwks_url)
            .send()
            .await
            .map_err(FetchError::Reqwest)?;
        let resp_json = resp
            .json::<serde_json::Value>()
            .await
            .map_err(FetchError::Reqwest)?;

        cache_guard.keys.clear();

        if let Some(keys) = resp_json.get("keys") {
            if let Some(keys_array) = keys.as_array() {
                for key in keys_array {
                    if let (Some(kid), Some(n), Some(e)) = (
                        key.get("kid").and_then(|k| k.as_str()),
                        key.get("n").and_then(|n| n.as_str()),
                        key.get("e").and_then(|e| e.as_str()),
                    ) {
                        cache_guard
                            .keys
                            .insert(kid.to_string(), format!("{}|{}", n, e));
                    }
                }
            }
        }
        println!("Fetched jwks: {:#?}", cache_guard.keys);
        cache_guard.last_updated = Instant::now();
        if cache_guard.first {
            cache_guard.first = false
        }
        Ok(cache_guard.keys.clone())
    } else {
        Ok(cache_guard.keys.clone())
    }
}
