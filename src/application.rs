use std::net::SocketAddr;
use std::sync::Arc;

use rbatis::rbatis::Rbatis;

use crate::configuration::{self, Configuration};
use crate::database::init_connection;
use crate::routes;

pub struct ApplicationState {
    pub configuration: Configuration,
    pub rbatis: Rbatis,
}

impl ApplicationState {
    pub fn new(cfg: Configuration, rbatis: Rbatis) -> Arc<Self> {
        Arc::new(Self {
            configuration: cfg,
            rbatis,
        })
    }
}

pub struct Application {}

impl Application {
    /// External Entry Point for the application, which usually get's run from main
    pub async fn init() {
        let cfg = configuration::Configuration::new().unwrap();
        tracing::info!("Parsed configuration: {:?}", cfg);

        let rb = init_connection(&cfg).await;

        let app_state = ApplicationState::new(cfg.clone(), rb);

        let router = routes::configure(app_state);

        // run our app with hyper
        let addr = SocketAddr::from((cfg.http.address, cfg.http.port));
        tracing::info!("http server is listening on \"{}\"", addr);
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .unwrap();
    }
}
