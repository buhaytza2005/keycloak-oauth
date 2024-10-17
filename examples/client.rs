use keycloak_oauth::client::{ClientConfiguration, ClientError, KeycloakClient};
use oauth2::TokenResponse;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let config = ClientConfiguration::from_env();

    let keycloak_client = KeycloakClient::new(config);
    let token = match keycloak_client.verify_and_refresh_access_token().await {
        Ok(token) => token,
        Err(_) => keycloak_client
            .authenticate()
            .await?
            .access_token()
            .secret()
            .clone(),
    };

    //make requests with the token
    println!("{}", token);

    Ok(())
}
