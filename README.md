# keycloak-oauth
**keycloak-oauth** is a basic Rust library designed to facilitate seamless integration with Keycloak's OAuth2 authentication flows. 


Currently only supports device authentication flow.
> [!CAUTION]
> The flow currently temporarily caches a token on disk in `.temp_files/token.json`. This will be upgraded to libsecret in the future

## Getting started

```rust
use keycloak_oauth::{ClientConfiguration, KeycloakClient, ClientError};

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    // Load configuration from environment variables or another source
    let config = ClientConfiguration::from_env();

    // Initialize the KeycloakClient
    let keycloak_client = KeycloakClient::new(config);

    // Authenticate and obtain the access token
    let access_token = keycloak_client.verify_and_refresh_token().await?;

    // Use the access token as needed
    println!("Obtained Access Token: {}", access_token);

    Ok(())
}
```

# License
This project is licensed under the MIT License.
