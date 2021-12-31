use crate::{application::Application, configuration, telemetry::init_subscriber};
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::testing::client::TestClient;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    init_subscriber();
});

pub async fn spawn_test_application<'a, F, T, CC>(cfg_fn: F) -> anyhow::Result<T>
where
    F: FnOnce(Application<CC>) -> Application<CC>,
    T: TestClient<CC>,
    CC: std::fmt::Debug + Clone + Deserialize<'a> + Sync + Send + 'static,
{
    Lazy::force(&TRACING);
    let mut test_cfg = configuration::Configuration::new()?;
    test_cfg.http.port = 0;

    let app = Application::<CC>::init(Some(test_cfg.clone())).await?;
    let prepared_app = cfg_fn(app).prepare().await.unwrap();

    let client = T::new(prepared_app.get_port(), test_cfg);

    tracing::debug!(
        "Test http application runs on port: {}",
        prepared_app.get_port()
    );

    tokio::spawn(async move { prepared_app.run().await });
    Ok(client)
}

pub async fn get_tc<'a, F, T, CC>(cfg_fn: F) -> T
where
    F: FnOnce(Application<CC>) -> Application<CC>,
    T: TestClient<CC>,
    CC: std::fmt::Debug + Clone + Deserialize<'a> + Sync + Send + 'static,
{
    spawn_test_application(cfg_fn)
        .await
        .expect("Unable to create test application")
}
