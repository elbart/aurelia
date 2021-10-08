use crate::configuration::Configuration;
use rbatis::rbatis::Rbatis;

pub mod model;

pub async fn init_connection(configuration: &Configuration) -> Rbatis {
    let rb = Rbatis::new();
    rb.link(&configuration.get_db_url()).await.expect(&format!(
        "Error connecting to database with URL: {}",
        &configuration.get_db_url()
    ));

    rb
}
