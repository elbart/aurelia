use aurelia::configuration;
use refinery::config::{Config, ConfigDbType};
mod migrations;

// mod embedded {
//     use refinery::embed_migrations;
//     embed_migrations!("src/database/migrations");
// }

fn main() {
    let cfg = configuration::Configuration::new().unwrap();
    // let rb = database::init_connection(&cfg).await;
    let mut conn = Config::new(ConfigDbType::Postgres)
        .set_db_host(&cfg.database.host)
        .set_db_port(&cfg.database.port.to_string())
        .set_db_name(&cfg.database.database_name)
        .set_db_user(&cfg.database.username)
        .set_db_pass(&cfg.database.password);
    migrations::runner().run(&mut conn).unwrap();
}
