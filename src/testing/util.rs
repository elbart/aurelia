use crate::{
    application::{Application, AureliaAppHandler},
    configuration,
    telemetry::init_subscriber,
};
use once_cell::sync::Lazy;

use crate::testing::client::TestClient;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    init_subscriber();
});

pub async fn spawn_test_application<F, T>(cfg_fn: F) -> anyhow::Result<T>
where
    F: FnOnce(Application) -> Application,
    T: TestClient,
{
    Lazy::force(&TRACING);
    let mut test_cfg = configuration::Configuration::new(None)?;
    test_cfg.http.port = 0;

    let app =
        Application::init(Some(test_cfg.clone()), Some(Box::new(AureliaAppHandler {}))).await?;
    let prepared_app = cfg_fn(app).prepare().await.unwrap();

    let client = T::new(prepared_app.get_port(), test_cfg);

    tracing::debug!(
        "Test http application runs on port: {}",
        prepared_app.get_port()
    );

    tokio::spawn(async move { prepared_app.run().await });
    Ok(client)
}

pub async fn get_tc<F, T>(cfg_fn: F) -> T
where
    F: FnOnce(Application) -> Application,
    T: TestClient,
{
    spawn_test_application(cfg_fn)
        .await
        .expect("Unable to create test application")
}
