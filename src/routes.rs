use std::sync::Arc;

use axum::{
    routing::{get, post},
    AddExtensionLayer, Router,
};
use sqlx::{Pool, Postgres};
use tower::layer::layer_fn;

use crate::{
    application::ApplicationState,
    handler::{
        authentication::{claims, oidc_client_login, oidc_client_login_cb},
        create_user, get_recipes, get_tags, root,
    },
    middleware::authentication::{JwtAuthenticationMiddleware, JwtClaims},
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
            router: Router::new()
                .route("/", get(root))
                .route("/users", post(create_user))
                .route("/tags", get(get_tags))
                .route("/recipes", get(get_recipes))
                .layer(AddExtensionLayer::new(state.clone()))
                .layer(layer_fn(|inner| JwtAuthenticationMiddleware {
                    inner,
                    configuration: state.configuration.clone(),
                }))
                .layer(AddExtensionLayer::new(None::<JwtClaims>)),
            state,
        }
    }

    /// Takes existing ApplicationRouter and adds authentication routes
    pub(crate) fn with_auth_routes(mut self) -> ApplicationRouter {
        let ar: Router = Router::new()
            .route("/self", get(claims))
            .route("/oidc_login/:provider_name", get(oidc_client_login))
            .route("/oidc_login_cb/:provider_name", get(oidc_client_login_cb))
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

    /// Optional extra routes given from the outside, mounted in the
    /// routing tree.
    #[allow(dead_code)]
    pub(crate) fn with_extra_routes(mut self, routes: Router, path_prefix: String) -> Self {
        self.router = self.router.nest(&path_prefix, routes);
        self
    }

    pub fn finalize(self, db: Arc<Pool<Postgres>>) -> Router {
        self.router.layer(AddExtensionLayer::new(db))
    }
}
