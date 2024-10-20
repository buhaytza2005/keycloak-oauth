use keycloak_oauth::client::{
    AppConfigBuilder, ClientConfiguration, ClientError, EnvironmentCredential, Flow,
    KeycloakClient, WithOwnerCredentials,
};
use oauth2::TokenResponse;

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let owner_credentials =
        EnvironmentCredential::resource_owner_password_credential().expect("creds");
    let app_config = AppConfigBuilder::new("waves-ui")
        .with_owner_credentials(owner_credentials)
        .build()
        .expect("app config to build");

    let keycloak_client = KeycloakClient::<WithOwnerCredentials>::from(app_config);

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
