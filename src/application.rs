use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::{BoxRoute, IntoMakeService};
use axum::Server;

use hyper::server::conn::AddrIncoming;

use crate::configuration::{self, Configuration};
use crate::database::init_connection;

use crate::routes::{self};

/// Central read-only (except of rbatis) application state struct.
#[derive(Clone, Debug)]
pub struct ApplicationState {
    pub configuration: Arc<Configuration>,
}

impl ApplicationState {
    pub fn new(cfg: Configuration) -> ApplicationState {
        Self {
            configuration: Arc::new(cfg),
        }
    }
}

pub struct Application {
    pub state: ApplicationState,
    server: Server<AddrIncoming, IntoMakeService<BoxRoute>>,
}

impl Application {
    /// External Entry Point for the application, which usually get's run from main
    pub async fn init(cfg: Option<configuration::Configuration>) -> anyhow::Result<Self> {
        let app_state;
        if cfg.is_none() {
            app_state = ApplicationState::new(configuration::Configuration::new()?);
            tracing::info!("Parsed configuration: {:?}", app_state.configuration);
        } else {
            app_state = ApplicationState::new(cfg.unwrap());
            tracing::info!("Got external configuration: {:?}", app_state.configuration);
        }

        let rb = init_connection(&app_state.configuration).await;

        // run our app with hyper
        let addr = SocketAddr::from((
            app_state.configuration.http.address,
            app_state.configuration.http.port,
        ));
        let router = routes::ApplicationRouter::configure(app_state.clone())
            .with_auth_routes()
            .finalize(Arc::new(rb));

        Ok(Self {
            state: app_state,
            server: axum::Server::bind(&addr).serve(router.into_make_service()),
        })
    }

    pub fn get_port(&self) -> u16 {
        self.server.local_addr().port()
    }

    pub async fn run(self) -> Result<(), hyper::Error> {
        tracing::info!(
            "http server is listening on \"{}\"",
            self.server.local_addr()
        );
        self.server.await
    }
}
