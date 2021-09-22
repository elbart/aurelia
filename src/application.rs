use std::net::SocketAddr;
use std::sync::Arc;

use rbatis::rbatis::Rbatis;

use crate::configuration::{self, Configuration};
use crate::database::init_connection;

use crate::routes;

/// Central read-only (except of rbatis) application state struct.
#[derive(Clone, Debug)]
pub struct ApplicationState {
    pub configuration: Arc<Configuration>,
    pub rbatis: Arc<Rbatis>,
}

impl ApplicationState {
    pub fn new(cfg: Configuration, rbatis: Rbatis) -> ApplicationState {
        Self {
            configuration: Arc::new(cfg),
            rbatis: Arc::new(rbatis),
        }
    }
}

pub struct Application {}

impl Application {
    /// External Entry Point for the application, which usually get's run from main
    pub async fn init() {
        // initialize tracing
        tracing_subscriber::fmt::init();
        let cfg = configuration::Configuration::new().unwrap();
        tracing::info!("Parsed configuration: {:?}", cfg);

        let rb = init_connection(&cfg).await;
        let app_state = ApplicationState::new(cfg.clone(), rb);
        let router = routes::AureliaRouter::configure(app_state).with_auth_routes();

        // run our app with hyper
        let addr = SocketAddr::from((cfg.http.address, cfg.http.port));
        tracing::info!("http server is listening on \"{}\"", addr);
        axum::Server::bind(&addr)
            .serve(router.get_routes().into_make_service())
            .await
            .unwrap();
    }
}
