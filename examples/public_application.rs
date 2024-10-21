use keycloak_oauth::client::{
    AppConfigBuilder, ClientConfiguration, EnvironmentCredential, KeycloakClient,
    WithDeviceCredentials, WithOwnerCredentials,
};
use oauth2::TokenResponse;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let device_credential = EnvironmentCredential::device_credential().expect("creds");
    let config = ClientConfiguration::from_env();
    let app_config = AppConfigBuilder::new("waves-ui")
        .auth_url(config.auth_url.clone().expect("should have auth_url"))
        .with_device_code_credentials(device_credential)
        .token_url(config.token_url.clone().expect("should have token"))
        .build()
        .expect("app config");

    let keycloak_client = KeycloakClient::<WithDeviceCredentials>::from(app_config);
    let token = match keycloak_client.verify_and_refresh_access_token().await {
        Ok(token) => token,
        Err(_) => keycloak_client
            .authenticate()
            .await?
            .access_token()
            .secret()
            .clone(),
    };
    println!("{:#?}", token);

    Ok(())
}
