use aurelia::testing::{client::AureliaTestClient, util::get_tc};

use crate::cfg_fn;

#[tokio::test]
async fn test_oidc_login() {
    let c = get_tc::<_, AureliaTestClient>(cfg_fn).await;
    let oidc_provider = c.configuration.get_oidc_provider("auth0").unwrap();

    let r = c
        .get_oidc_login(&oidc_provider.provider_name)
        .await
        .unwrap();

    assert!(r.headers().get("Location").is_some());
    assert!(match r.headers().get("Location") {
        Some(s) => {
            s.to_str().unwrap().contains(&oidc_provider.issuer_url)
        }
        None => false,
    });
}

#[tokio::test]
async fn test_oidc_login_cb() {
    let c = get_tc::<_, AureliaTestClient>(cfg_fn).await;
    let oidc_provider = c.configuration.get_oidc_provider("auth0").unwrap();

    let r = c
        .get_oidc_login(&oidc_provider.provider_name)
        .await
        .unwrap();

    let mut provider_location = r
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    provider_location = provider_location.replace("3001", &c.port.to_string());

    let provider_response = reqwest::get(provider_location).await.unwrap();

    // TODO: read jwt
    assert!(provider_response.status() == 200);
}
