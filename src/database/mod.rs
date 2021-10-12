use sea_orm::{Database, DatabaseConnection};

use crate::configuration::Configuration;

// auto generated sea orm models
pub mod entity;

pub async fn init_connection(configuration: &Configuration) -> DatabaseConnection {
    let db: DatabaseConnection =
        Database::connect(&configuration.get_db_url())
            .await
            .expect(&format!(
                "Error connecting to database with URL: {}",
                &configuration.get_db_url()
            ));

    db
}
