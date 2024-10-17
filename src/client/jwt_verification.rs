use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};

use super::jwks::{fetch_and_cache_jwks, FetchError, SharedKeyCache};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
    iss: String,
    aud: Vec<String>,
}

#[derive(Debug)]
pub enum VerifyJwtError {
    FetchJwksError(FetchError),
    JwtDecodeError(jsonwebtoken::errors::Error),
    InvalidKeyFormatError(String),
}
impl std::fmt::Display for VerifyJwtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for VerifyJwtError {}

impl From<FetchError> for VerifyJwtError {
    fn from(error: FetchError) -> Self {
        VerifyJwtError::FetchJwksError(error)
    }
}
impl From<jsonwebtoken::errors::Error> for VerifyJwtError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        VerifyJwtError::JwtDecodeError(error)
    }
}

impl From<&str> for VerifyJwtError {
    fn from(error: &str) -> Self {
        VerifyJwtError::InvalidKeyFormatError(error.to_string())
    }
}

pub async fn verify_jwt(
    token: &str,
    jwks_url: &str,
    cache: SharedKeyCache,
    audience: &[&str],
    issuer: &[&str],
) -> Result<TokenData<Claims>, VerifyJwtError> {
    // Fetch JWKS from cache or update it if needed
    let keys = fetch_and_cache_jwks(jwks_url, cache).await?;

    // Decode the token header to get the key ID (kid)
    let header = decode_header(token)?;
    let kid = header.kid.ok_or("Missing kid in JWT header")?;

    // Get the corresponding public key from the JWKS cache
    let key_data = keys.get(&kid).ok_or("No matching public key found")?;
    let (n, e) = key_data.split_once('|').ok_or("Invalid key format")?;

    // Convert the public key components (n, e) to a decoding key
    let decoding_key = DecodingKey::from_rsa_components(n, e)?;

    // Validate and decode the JWT
    let mut validation = Validation::new(Algorithm::RS256);

    validation.set_audience(audience);
    validation.set_issuer(issuer);

    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(token_data) => Ok(token_data),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(VerifyJwtError::InvalidKeyFormatError(e.to_string()))
        }
    }

    // Token is valid, return the claims
}
