use aurelia::{application, telemetry::init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_subscriber();
    let app = application::Application::init(None).await?;

    Ok(app.run().await?)
}
