pub struct TestClient {
    pub port: u16,
    http_client: reqwest::Client,
}

impl TestClient {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            http_client: reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .unwrap(),
        }
    }

    async fn request_get(&self, uri_part: &str) -> anyhow::Result<reqwest::Response> {
        Ok(self
            .http_client
            .get(format!("http://localhost:{}{}", self.port, uri_part))
            .send()
            .await?)
    }

    pub async fn get_oidc_login(&self, provider_name: &str) -> anyhow::Result<reqwest::Response> {
        Ok(self
            .request_get(&format!("/auth/oidc_login/{}", provider_name))
            .await?)
    }
}
