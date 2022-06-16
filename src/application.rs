use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use axum::routing::IntoMakeService;
use axum::{Extension, Router, Server};
use dyn_clonable::clonable;
use hyper::server::conn::AddrIncoming;

use crate::configuration::{self, Configuration};
use crate::database::init_connection;

use crate::driver::db;
use crate::middleware::authentication::JwtClaims;
use crate::routes::{self, ApplicationRouter};

#[async_trait]
#[clonable]
pub trait AppHandler: std::fmt::Debug + Clone + Send + Sync {
    async fn on_login_callback(
        &self,
        claims: &mut JwtClaims,
        config: &Configuration,
        db: db::DB,
    ) -> anyhow::Result<()>;
}

#[derive(Debug, Clone)]
pub struct AureliaAppHandler {}

#[async_trait]
impl AppHandler for AureliaAppHandler {
    async fn on_login_callback(
        &self,
        claims: &mut JwtClaims,
        _config: &Configuration,
        _db: db::DB,
    ) -> anyhow::Result<()> {
        tracing::info!("This is the On Login Callback for the claims: {:?}", claims);
        Ok(())
    }
}

/// Central read-only application state struct.
#[derive(Clone, Debug)]
pub struct ApplicationState {
    pub configuration: Arc<Configuration>,
    pub handlers: Box<dyn AppHandler>,
}

impl ApplicationState {
    pub fn new(cfg: Configuration, handlers: Box<dyn AppHandler>) -> ApplicationState {
        Self {
            configuration: Arc::new(cfg),
            handlers,
        }
    }
}

pub struct Application {
    pub state: ApplicationState,
    router: ApplicationRouter,
}

impl Application {
    /// External Entry Point for the application, which usually get's run from main
    pub async fn init(
        cfg: Option<configuration::Configuration>,
        handlers: Option<Box<dyn AppHandler>>,
    ) -> anyhow::Result<Self> {
        let handlers = if handlers.is_some() {
            handlers.unwrap()
        } else {
            Box::new(AureliaAppHandler {})
        };

        let app_state;
        if cfg.is_none() {
            app_state = ApplicationState::new(configuration::Configuration::new(None)?, handlers);
            tracing::info!("Parsed configuration: {:?}", app_state.configuration);
        } else {
            app_state = ApplicationState::new(cfg.unwrap(), handlers);
            tracing::info!("Got external configuration: {:?}", app_state.configuration);
        }

        // Prepare router for configuration
        let router = routes::ApplicationRouter::configure(app_state.clone());

        Ok(Self {
            state: app_state,
            router,
        })
    }

    pub fn with_auth_routes(mut self) -> Self {
        self.router = self.router.with_auth_routes();
        self
    }

    pub fn with_extra_routes(mut self, routes: Router) -> Self {
        self.router = self.router.with_extra_routes(routes);
        self
    }

    pub fn with_static_routes(mut self, dir: (String, String)) -> Self {
        self.router = self.router.with_static_route(dir);
        self
    }

    pub fn with_extension<E>(mut self, ext: Extension<E>) -> Self
    where
        E: Clone + Sync + Send + 'static,
    {
        self.router = self.router.with_extension(ext);
        self
    }

    pub async fn prepare(self) -> Result<PreparedApplication, hyper::Error> {
        let db = init_connection(&self.state.configuration).await;

        let addr = SocketAddr::from((
            self.state.configuration.http.address,
            self.state.configuration.http.port,
        ));

        let server = axum::Server::bind(&addr).serve(self.router.finalize(db).into_make_service());
        tracing::info!("http server is listening on \"{}\"", server.local_addr());

        Ok(PreparedApplication { server })
    }
}

pub struct PreparedApplication {
    server: Server<AddrIncoming, IntoMakeService<Router>>,
}

impl PreparedApplication {
    pub fn get_port(&self) -> u16 {
        self.server.local_addr().port()
    }

    pub async fn run(self) -> Result<(), hyper::Error> {
        self.server.await
    }
}
