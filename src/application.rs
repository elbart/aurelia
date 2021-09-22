use std::net::SocketAddr;
use std::sync::Arc;

use rbatis::rbatis::Rbatis;

use crate::configuration::{self, Configuration};
use crate::database::init_connection;

use crate::routes;

pub type ApplicationState = Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub configuration: Configuration,
    pub rbatis: Rbatis,
}

impl AppState {
    pub fn new(cfg: Configuration, rbatis: Rbatis) -> ApplicationState {
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
        let app_state = AppState::new(cfg.clone(), rb);
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
