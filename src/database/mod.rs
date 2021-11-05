use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::configuration::Configuration;

pub type DbPool = Pool<Postgres>;

// auto generated sea orm models
pub async fn init_connection(configuration: &Configuration) -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&configuration.get_db_url())
        .await
        .expect(&format!(
            "Error connecting to database with URL: {}",
            &configuration.get_db_url()
        ));

    pool
}
