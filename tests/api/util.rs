use aurelia::{
    application::Application,
    configuration::{self, Configuration},
    middleware::authentication::JwtClaims,
    telemetry::init_subscriber,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use once_cell::sync::Lazy;
use uuid::Uuid;

use crate::client::TestClient;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    init_subscriber();
});

pub async fn spawn_test_application() -> anyhow::Result<TestClient> {
    Lazy::force(&TRACING);
    let mut test_cfg = configuration::Configuration::new()?;
    test_cfg.http.port = 0;

    let app = Application::init(Some(test_cfg.clone())).await?;
    let client = TestClient::new(app.get_port(), test_cfg);

    tracing::debug!("Test http application runs on port: {}", app.get_port());

    tokio::spawn(async move { app.run().await });
    Ok(client)
}

pub async fn get_tc() -> TestClient {
    spawn_test_application()
        .await
        .expect("Unable to create test application")
}

pub async fn create_jwt(cfg: &Configuration) -> String {
    let claims = JwtClaims::new(
        Uuid::new_v4().to_string(),
        cfg.application.auth.jwt_expiration_offset_seconds,
    );

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.application.auth.jwt_secret.as_ref()),
    )
    .unwrap()
}
