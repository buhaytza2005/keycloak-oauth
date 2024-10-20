use keycloak_oauth::client::{
    AppConfigBuilder, EnvironmentCredential, KeycloakClient, WithOwnerCredentials,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let owner_creds = EnvironmentCredential::resource_owner_password_credential()?;
    let app_config = AppConfigBuilder::new("waves-ui")
        .auth_url("")
        .with_owner_credentials(owner_creds)
        .build()
        .expect("app config");

    let client = KeycloakClient::<WithOwnerCredentials>::from(app_config);

    Ok(())
}
