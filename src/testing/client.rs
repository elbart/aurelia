use async_trait::async_trait;

use crate::{configuration::Configuration, middleware::authentication::create_jwt};

pub struct AureliaTestClient {
    pub port: u16,
    http_client: reqwest::Client,
    pub configuration: Configuration,
    pub client_jwt: Option<String>,
}

#[async_trait]
pub trait TestClient {
    fn new(port: u16, configuration: Configuration) -> Self;
    async fn authenticated(&mut self);
    fn set_jwt(&mut self, jwt: String);
}

#[async_trait]
impl TestClient for AureliaTestClient {
    fn new(port: u16, configuration: Configuration) -> Self {
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

    async fn authenticated(&mut self) {
        self.set_jwt(create_jwt(&self.configuration).await);
    }

    fn set_jwt(&mut self, jwt: String) {
        self.client_jwt = Some(jwt)
    }
}

impl AureliaTestClient {
    pub fn uri(&self, uri_part: &str) -> String {
        format!("http://localhost:{}{}", self.port, uri_part)
    }

    pub async fn request_get(&self, uri: &str) -> anyhow::Result<reqwest::Response> {
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
}
