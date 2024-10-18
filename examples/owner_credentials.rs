use keycloak_oauth::client::{ClientConfiguration, ClientError, KeycloakClient};
use oauth2::TokenResponse;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let config = ClientConfiguration::from_env();

    let keycloak_client = KeycloakClient::new(config);

    keycloak_client.initiate_password_flow().await?;

    Ok(())
}
