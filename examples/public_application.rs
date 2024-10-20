use keycloak_oauth::client::{
    AppConfigBuilder, EnvironmentCredential, KeycloakClient, PublicApplicationBuilder,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let owner_creds = EnvironmentCredential::resource_owner_password_credential()?;
    let app_config = AppConfigBuilder::new("waves-ui")
        .token_url("")
        .auth_url("")
        .with_owner_credentials(owner_creds)
        .build();

    let client = KeycloakClient::from(app_config);

    Ok(())
}
