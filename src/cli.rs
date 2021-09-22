use anyhow::Result;
use aurelia::{configuration, middleware::authentication::JwtClaims};
use jsonwebtoken::{encode, EncodingKey, Header};
use refinery::config::{Config, ConfigDbType};
use structopt::StructOpt;
use uuid::Uuid;

mod migrations;

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

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Cli::from_args();
    tracing::info!("Given command / subcommand + args are: {:?}", opt);
    let cfg = configuration::Configuration::new()?;

    match opt.cmd {
        Command::Migrate => {
            let mut conn = Config::new(ConfigDbType::Postgres)
                .set_db_host(&cfg.database.host)
                .set_db_port(&cfg.database.port.to_string())
                .set_db_name(&cfg.database.database_name)
                .set_db_user(&cfg.database.username)
                .set_db_pass(&cfg.database.password);
            migrations::runner().run_async(&mut conn).await?;
        }
        Command::CreateJWT => {
            let claims = JwtClaims::new(
                Uuid::new_v4().to_string(),
                cfg.application.auth.jwt_expiration_offset_seconds,
            );
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(cfg.application.auth.jwt_secret.as_ref()),
            )?;
            println!("{}", &token);
        }
    }

    Ok(())
}
