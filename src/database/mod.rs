use sqlx::postgres::PgPoolOptions;

use crate::{configuration::Configuration, driver::db};

pub mod entity;

// auto generated sea orm models
pub async fn init_connection(configuration: &Configuration) -> db::DB {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&configuration.get_db_url())
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Error connecting to database with URL: {}",
                &configuration.get_db_url()
            )
        });

    pool
}
