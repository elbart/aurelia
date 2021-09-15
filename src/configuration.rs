use std::net::Ipv4Addr;

// use anyhow::Result;
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Deserializer};

/// Database configuration
#[derive(Debug, Deserialize, Clone)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub username: String,
    pub password: String,
}

/// Http configuration
#[derive(Debug, Deserialize, Clone)]
pub struct Http {
    #[serde(deserialize_with = "ip_string_to_octets")]
    pub address: Ipv4Addr,
    pub port: u16,
}

/// Application configuration
#[derive(Debug, Deserialize, Clone)]
pub struct Application {
    pub jwt_secret: String,
    pub jwt_expiration_offset_seconds: usize,
    pub jwt_header_name: String,
}

/// Central configuration object which reads:
#[derive(Debug, Deserialize, Clone)]
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
    /// 1. File ``etc/aurelia.toml``,
    /// 2. Env with prefix ``AURELIA_``.
    pub fn new() -> Result<Self, ConfigError> {
        let mut c = Config::default();
        c.merge(File::with_name("etc/aurelia.toml"))?;
        c.merge(Environment::with_prefix("AURELIA"))?;

        c.try_into()
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
}
