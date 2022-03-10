use crate::{configuration::Configuration, middleware::authentication::create_jwt};
use anyhow::Result;
use openssl::rsa::Rsa;
use refinery::{
    config::{Config, ConfigDbType},
    Runner,
};
use structopt::StructOpt;
use uuid::Uuid;

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

pub async fn cli_migrate_database<A>(cfg: &Configuration, runner: A) -> Result<()>
where
    A: Fn() -> Runner,
{
    let mut conn = Config::new(ConfigDbType::Postgres)
        .set_db_host(&cfg.database.host)
        .set_db_port(&cfg.database.port.to_string())
        .set_db_name(&cfg.database.database_name)
        .set_db_user(&cfg.database.username)
        .set_db_pass(&cfg.database.password);
    runner().run_async(&mut conn).await?;

    Ok(())
}

pub async fn cli_create_jwt(cfg: &Configuration, user_id: Option<Uuid>, rsa: bool) -> Result<()> {
    println!("{}", &create_jwt(cfg, user_id, Some(rsa)).await?);
    Ok(())
}

pub fn create_rsa_keypair() -> Result<()> {
    let rsa = Rsa::generate(4096).unwrap();
    println!(
        "{}\n\n\n{}",
        String::from_utf8(rsa.private_key_to_pem()?)?,
        String::from_utf8(rsa.public_key_to_pem()?)?
    );
    Ok(())
}
