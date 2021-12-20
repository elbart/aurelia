use aurelia::{application::Application, configuration, telemetry::init_subscriber};
use once_cell::sync::Lazy;

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