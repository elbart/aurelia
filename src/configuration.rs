use std::{collections::HashMap, fmt, net::Ipv4Addr};

use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Deserializer, Serialize};

/// Database configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub username: String,
    pub password: String,
}

/// Http configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Http {
    #[serde(deserialize_with = "ip_string_to_octets")]
    pub address: Ipv4Addr,
    pub port: u16,
    pub base_url: String,
}

impl Http {
    pub fn full_base_url(&self) -> String {
        if (self.port == 80 && self.base_url.starts_with("http://"))
            || (self.port == 443 && self.base_url.starts_with("https://"))
        {
            return self.base_url.clone();
        }

        format!("{}:{}", self.base_url, self.port)
    }
}

/// Application configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    pub auth: Auth,
    pub debug: bool,
    pub custom: toml::Value,
}

impl Application {
    /// Try to decode custom part into given type
    pub fn custom<'a, T: Deserialize<'a>>(&self) -> Result<T, toml::de::Error> {
        self.custom.clone().try_into()
    }
}

/// Authentication / Authorization configuration
#[derive(Serialize, Deserialize, Clone)]
pub struct Auth {
    pub jwt_secret: String,
    pub jwt_expiration_offset_seconds: usize,
    pub jwt_header_name: String,
    pub jwt_cookie_name: String,
    pub jwt_algorithm: String,
    pub jwt_rsa_private_key: String,
    pub jwt_rsa_public_key: String,
    pub path_prefix: String,
    pub oidc: HashMap<String, Oidc>,
    pub redirect_on_login_success: String,
    pub redirect_on_login_error: String,
    pub login_path: String,
}

impl std::fmt::Debug for Auth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Auth")
            .field("jwt_secret", &"***")
            .field(
                "jwt_expiration_offset_seconds",
                &self.jwt_expiration_offset_seconds,
            )
            .field("jwt_header_name", &self.jwt_header_name)
            .field("jwt_cookie_name", &self.jwt_cookie_name)
            .field("jwt_algorithm", &self.jwt_algorithm)
            .field("jwt_rsa_private_key", &"***")
            .field("jwt_rsa_public_key", &"***")
            .field("path_prefix", &self.path_prefix)
            .field("oidc", &self.oidc)
            .finish()
    }
}

// OpenID client configuration
#[derive(Serialize, Deserialize, Clone)]
pub struct Oidc {
    pub provider_name: String,
    pub client_name: String,
    pub client_id: String,
    pub client_secret: String,
    pub client_scopes: Vec<String>,
    pub client_role: String,
    pub issuer_url: String,
    pub redirect_url: String,
}

impl std::fmt::Debug for Oidc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Oidc")
            .field("provider_name", &self.provider_name)
            .field("client_name", &self.client_name)
            .field("client_id", &self.client_id)
            .field("client_secret", &"***")
            .field("client_scopes", &self.client_scopes)
            .field("client_role", &self.client_role)
            .field("issuer_url", &self.issuer_url)
            .field("redirect_url", &self.redirect_url)
            .finish()
    }
}

/// Central configuration object which reads
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub database: Database,
    pub http: Http,
    pub application: Application,
}

fn ip_string_to_octets<'de, D>(deserializer: D) -> Result<Ipv4Addr, D::Error>
where
    D: Deserializer<'de>,
{
    Ipv4Addr::deserialize(deserializer)
}

impl Configuration {
    /// 1. File ``etc/aurelia.toml`` (optional),
    /// 2. Env with prefix ``AURELIA_``.
    pub fn new() -> Result<Self, ConfigError> {
        let d = Config::builder()
            .add_source(File::with_name("etc/aurelia.toml").required(false))
            .add_source(Environment::with_prefix("AURELIA").separator("__"))
            .build()?;

        d.try_deserialize()
    }

    pub fn get_db_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.database.username,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.database_name,
        )
    }

    pub fn get_oidc_provider(&self, name: &str) -> Option<Oidc> {
        self.application.auth.oidc.get(name).cloned()
    }
}
