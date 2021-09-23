use axum::{
    handler::{get, post},
    routing::BoxRoute,
    AddExtensionLayer, Router,
};
use tower::layer::layer_fn;

use crate::{
    application::ApplicationState,
    handler::{
        authentication::{claims, oidc_client_login, oidc_client_login_cb},
        create_user, get_tags, root,
    },
    middleware::authentication::{JwtAuthenticationMiddleware, JwtClaims},
};

#[derive(Debug)]
pub struct AureliaRouter {
    router: Router<BoxRoute>,
    state: ApplicationState,
}

impl AureliaRouter {
    /// Router definitions should go here.
    pub(crate) fn configure(state: ApplicationState) -> AureliaRouter {
        Self {
            router: Router::new()
                .route("/", get(root))
                .route("/users", post(create_user))
                .route("/tags", get(get_tags))
                .layer(AddExtensionLayer::new(state.clone()))
                .boxed(),
            state,
        }
    }

    /// Takes existing AureliaRouter and adds authentication routes
    pub(crate) fn with_auth_routes(mut self) -> AureliaRouter {
        let ar: Router<BoxRoute> = Router::new()
            .route("/self", get(claims))
            .route("/oidc_login/:provider_name", get(oidc_client_login))
            .route("/oidc_login_cb/:provider_name", get(oidc_client_login_cb))
            .layer(layer_fn(|inner| JwtAuthenticationMiddleware {
                inner,
                configuration: self.state.configuration.clone(),
            }))
            .layer(AddExtensionLayer::new(None::<JwtClaims>))
            .layer(AddExtensionLayer::new(self.state.clone()))
            .boxed();

        self.router = self
            .router
            .nest(&self.state.configuration.application.auth.path_prefix, ar)
            .boxed();

        self
    }

    /// Optional extra routes given from the outside, mounted in the
    /// routing tree.
    #[allow(dead_code)]
    pub(crate) fn with_extra_routes(
        mut self,
        routes: Router<BoxRoute>,
        path_prefix: String,
    ) -> Self {
        self.router = self.router.nest(&path_prefix, routes).boxed();
        self
    }

    pub fn get_routes(self) -> Router<BoxRoute> {
        self.router.boxed()
    }
}
