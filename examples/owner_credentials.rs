use keycloak_oauth::client::{ClientConfiguration, ClientError, Flow, KeycloakClient};
use oauth2::TokenResponse;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let config = ClientConfiguration::from_env();

    let keycloak_client = KeycloakClient::new(config);

    let _token = match keycloak_client.verify_and_refresh_access_token().await {
        Ok(token) => token,
        Err(_) => keycloak_client
            .authenticate(Flow::OwnerCredentials)
            .await?
            .access_token()
            .secret()
            .clone(),
    };

    Ok(())
}
