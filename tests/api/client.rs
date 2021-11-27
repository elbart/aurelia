use aurelia::{configuration::Configuration, middleware::authentication::create_jwt};

pub struct TestClient {
    pub port: u16,
    http_client: reqwest::Client,
    pub configuration: Configuration,
    client_jwt: Option<String>,
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
            client_jwt: None,
        }
    }

    pub async fn authenticated(mut self) -> Self {
        self.set_jwt(create_jwt(&self.configuration).await);
        self
    }

    pub fn set_jwt(&mut self, jwt: String) {
        self.client_jwt = Some(jwt);
    }

    fn uri(&self, uri_part: &str) -> String {
        format!("http://localhost:{}{}", self.port, uri_part)
    }

    async fn request_get(&self, uri: &str) -> anyhow::Result<reqwest::Response> {
        let mut req = self.http_client.get(uri);
        if let Some(jwt) = &self.client_jwt {
            req = req.header("Authorization", &format!("Bearer {}", jwt));
        }
        Ok(req.send().await?)
    }

    pub async fn get_oidc_login(&self, provider_name: &str) -> anyhow::Result<reqwest::Response> {
        Ok(self
            .request_get(&self.uri(&format!("/auth/oidc_login/{}", provider_name)))
            .await?)
    }

    pub async fn _get_full_uri(&self, uri: String) -> anyhow::Result<reqwest::Response> {
        Ok(self.request_get(&uri).await?)
    }

    pub async fn get_recipes(&self) -> anyhow::Result<reqwest::Response> {
        Ok(self
            .request_get(&self.uri(&format!("/api/recipes")))
            .await?)
    }

    pub async fn get_tags(&self) -> anyhow::Result<reqwest::Response> {
        Ok(self.request_get(&self.uri(&format!("/api/tags"))).await?)
    }

    pub async fn get_ingredients(&self) -> anyhow::Result<reqwest::Response> {
        Ok(self
            .request_get(&self.uri(&format!("/api/ingredients")))
            .await?)
    }
}
