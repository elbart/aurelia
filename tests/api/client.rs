use aurelia::configuration::Configuration;

pub struct TestClient {
    pub port: u16,
    http_client: reqwest::Client,
    pub configuration: Configuration,
}

impl TestClient {
    pub fn new(port: u16, configuration: Configuration) -> Self {
        Self {
            port,
            http_client: reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .unwrap(),
            configuration,
        }
    }

    fn uri(&self, uri_part: &str) -> String {
        format!("http://localhost:{}{}", self.port, uri_part)
    }

    async fn request_get(&self, uri: &str) -> anyhow::Result<reqwest::Response> {
        Ok(self.http_client.get(uri).send().await?)
    }

    pub async fn get_oidc_login(&self, provider_name: &str) -> anyhow::Result<reqwest::Response> {
        Ok(self
            .request_get(&self.uri(&format!("/auth/oidc_login/{}", provider_name)))
            .await?)
    }

    pub async fn get_full_uri(&self, uri: String) -> anyhow::Result<reqwest::Response> {
        Ok(self.request_get(&uri).await?)
    }
}
