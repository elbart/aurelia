use aurelia::{application::Application, configuration};

use crate::client::TestClient;

pub async fn spawn_test_application() -> anyhow::Result<TestClient> {
    let mut test_cfg = configuration::Configuration::new()?;
    test_cfg.http.port = 0;

    let app = Application::init(Some(test_cfg)).await?;
    let client = TestClient::new(app.get_port());

    tracing::debug!("Test http application runs on port: {}", app.get_port());

    tokio::spawn(async move { app.run().await });
    Ok(client)
}
