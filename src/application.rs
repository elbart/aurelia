use std::net::SocketAddr;
use std::sync::Arc;

use axum::handler::{get, post};
use axum::routing::BoxRoute;
use axum::AddExtensionLayer;
use axum::Router;
use rbatis::rbatis::Rbatis;

use crate::configuration::{self, Configuration};
use crate::database::init_connection;
use crate::handler::{create_user, get_tags, root};

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
    /// Router definitions should go here.
    /// This returns a ``BoxRoute``, because it's otherwise hard to guess the
    /// type before.
    fn routes(state: Arc<ApplicationState>) -> Router<BoxRoute> {
        Router::new()
            .route("/", get(root))
            // `POST /users` goes to `create_user`
            .route("/users", post(create_user))
            .route("/tags", get(get_tags))
            .layer(AddExtensionLayer::new(state))
            .boxed()
    }

    /// External Entry Point for the application, which usually get's run from main
    pub async fn init() {
        let cfg = configuration::Configuration::new().unwrap();
        tracing::info!("Parsed configuration: {:?}", cfg);

        let rb = init_connection(&cfg).await;

        let app_state = ApplicationState::new(cfg.clone(), rb);

        let app = Application::routes(app_state);

        // run our app with hyper
        let addr = SocketAddr::from((cfg.http.address, cfg.http.port));
        tracing::info!("http server is listening on \"{}\"", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
