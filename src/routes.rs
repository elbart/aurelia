use axum::{
    extract::Extension,
    middleware::{self},
    routing::{get, get_service},
    Router,
};
use hyper::StatusCode;
use tower_http::services::ServeDir;

use crate::{
    application::ApplicationState,
    driver::db,
    handler::authentication::{claims, oidc_client_login, oidc_client_login_cb},
    middleware::authentication::{jwt_auth_middleware, JwtClaims},
};

#[derive(Debug, Clone)]
pub struct ApplicationRouter {
    router: Router,
    state: ApplicationState,
}
impl ApplicationRouter {
    /// Router definitions should go here.
    pub(crate) fn configure(state: ApplicationState) -> ApplicationRouter {
        Self {
            router: Router::new(),
            state,
        }
    }

    /// Takes existing ApplicationRouter and adds authentication routes
    pub(crate) fn with_auth_routes(mut self) -> ApplicationRouter {
        let cfg_clone = self.state.configuration.application.clone();
        let ar: Router = Router::new()
            .route("/self", get(claims))
            .route("/oidc_login/:provider_name", get(oidc_client_login))
            .route("/oidc_login_cb/:provider_name", get(oidc_client_login_cb))
            .route_layer(middleware::from_fn(move |req, next| {
                jwt_auth_middleware(req, next, cfg_clone.clone())
            }))
            .layer(Extension(None::<JwtClaims>))
            .layer(Extension(self.state.clone()));

        self.router = self
            .router
            .nest(&self.state.configuration.application.auth.path_prefix, ar);

        self
    }

    pub(crate) fn with_static_route(mut self, dir: (String, String)) -> ApplicationRouter {
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

    /// Convenience function to add an Extension to the root routing
    pub(crate) fn with_extension<E>(mut self, ext: Extension<E>) -> Self
    where
        E: Clone + Sync + Send + 'static,
    {
        self.router = self.router.layer(ext);
        self
    }

    /// Optional extra routes given from the outside, mounted in the
    /// routing tree.
    pub(crate) fn with_extra_routes(mut self, routes: Router) -> Self {
        self.router = self.router.merge(routes);
        self
    }

    /// Optionally allow HTTP Request/Response tracing using `tower_http::trace::TraceLayer`
    pub(crate) fn with_trace_layer(mut self) -> Self {
        self.router = self.router.layer(
            tower::ServiceBuilder::new().layer(tower_http::trace::TraceLayer::new_for_http()),
        );
        self
    }

    pub fn finalize(self, db: db::DB) -> Router {
        self.router.layer(Extension(db))
    }
}
