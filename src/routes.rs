use std::sync::Arc;

use axum::{
    routing::{get, get_service},
    AddExtensionLayer, Router,
};
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use tower::layer::layer_fn;
use tower_http::services::ServeDir;

use crate::{
    application::ApplicationState,
    handler::authentication::{claims, oidc_client_login, oidc_client_login_cb},
    middleware::authentication::{JwtAuthenticationMiddleware, JwtClaims},
};

#[derive(Debug, Clone)]
pub struct ApplicationRouter<CC> {
    router: Router,
    state: ApplicationState<CC>,
}

impl<'a, CC> ApplicationRouter<CC>
where
    CC: Sync + Send + Clone + 'static,
{
    /// Router definitions should go here.
    pub(crate) fn configure(state: ApplicationState<CC>) -> ApplicationRouter<CC> {
        Self {
            router: Router::new(),
            state,
        }
    }

    /// Takes existing ApplicationRouter and adds authentication routes
    pub(crate) fn with_auth_routes(mut self) -> ApplicationRouter<CC>
    where
        CC: Deserialize<'a>,
    {
        let ar: Router = Router::new()
            .route("/self", get(claims))
            .route("/oidc_login/:provider_name", get(oidc_client_login::<CC>))
            .route(
                "/oidc_login_cb/:provider_name",
                get(oidc_client_login_cb::<CC>),
            )
            .layer(layer_fn(|inner| JwtAuthenticationMiddleware {
                inner,
                configuration: self.state.configuration.clone(),
            }))
            .layer(AddExtensionLayer::new(None::<JwtClaims>))
            .layer(AddExtensionLayer::new(self.state.clone()));

        self.router = self
            .router
            .nest(&self.state.configuration.application.auth.path_prefix, ar);

        self
    }

    pub(crate) fn with_static_route(mut self, dir: (String, String)) -> ApplicationRouter<CC> {
        self.router = self.router.nest(
            dir.0.as_str(),
            get_service(ServeDir::new(dir.1.as_str())).handle_error(
                |error: std::io::Error| async move {
                    tracing::error!("Unhandled internal error: {}", error);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        );

        self
    }

    /// Optional extra routes given from the outside, mounted in the
    /// routing tree.
    pub(crate) fn with_extra_routes(mut self, routes: Router) -> Self {
        self.router = self.router.merge(routes);
        self
    }

    pub fn finalize(self, db: Arc<Pool<Postgres>>) -> Router {
        self.router.layer(AddExtensionLayer::new(db))
    }
}
