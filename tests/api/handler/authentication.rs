use crate::util::spawn_test_application;

#[tokio::test]
async fn test_oidc_login() -> anyhow::Result<()> {
    let c = spawn_test_application()
        .await
        .expect("Test Application creation failed");

    let r = c.get_oidc_login("sh_stage").await?;
    assert!(r.headers().get("Location").is_some());

    Ok(())
}
