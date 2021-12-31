use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;

use crate::configuration::Configuration;

pub mod entity;

pub type DbPool = Arc<Pool<Postgres>>;

// auto generated sea orm models
pub async fn init_connection<'a, CC>(configuration: &Configuration<CC>) -> Pool<Postgres>
where
    CC: Deserialize<'a>,
{
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
