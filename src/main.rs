use aurelia::application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = application::Application::init(None).await?;

    Ok(app.run().await?)
}
