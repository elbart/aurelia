use crate::{
    configuration::Configuration, database_migrations, middleware::authentication::create_jwt,
};
use anyhow::Result;
use refinery::config::{Config, ConfigDbType};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "CLI client", about = "Command line interface")]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Migrate,
    CreateJWT,
}

pub async fn cli_migrate_database(cfg: &Configuration) -> Result<()> {
    let mut conn = Config::new(ConfigDbType::Postgres)
        .set_db_host(&cfg.database.host)
        .set_db_port(&cfg.database.port.to_string())
        .set_db_name(&cfg.database.database_name)
        .set_db_user(&cfg.database.username)
        .set_db_pass(&cfg.database.password);
    database_migrations::migrations::runner()
        .run_async(&mut conn)
        .await?;

    Ok(())
}

pub async fn cli_create_jwt(cfg: &Configuration) -> Result<()> {
    println!("{}", &create_jwt(cfg).await);
    Ok(())
}
