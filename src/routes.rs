

use axum::{
    handler::{get, post},
    routing::BoxRoute,
    AddExtensionLayer, Router,
};
use tower::layer::layer_fn;

use crate::{
    application::{ApplicationState},
    handler::{
        authentication::{claims, oidc_client_login},
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
    /// This returns a ``BoxRoute``, because it's otherwise hard to guess the
    /// type before.
    pub(crate) fn configure(state: ApplicationState) -> AureliaRouter {
        Self {
            router: Router::new()
                .route("/", get(root))
                .route("/users", post(create_user))
                .route("/tags", get(get_tags))
                .layer(layer_fn(|inner| JwtAuthenticationMiddleware {
                    inner,
                    configuration: state.configuration.clone(),
                }))
                .layer(AddExtensionLayer::new(state.clone()))
                .boxed(),
            state,
        }
    }

    /// Takes existing AureliaRouter and adds authentication routes
    /// TODO: configure /auth mountpoint
    pub(crate) fn with_auth_routes(mut self) -> AureliaRouter {
        let ar: Router<BoxRoute> = Router::new()
            .route("/self", get(claims))
            .route("/oidc_login", get(oidc_client_login))
            .layer(layer_fn(|inner| JwtAuthenticationMiddleware {
                inner,
                configuration: self.state.configuration.clone(),
            }))
            .layer(AddExtensionLayer::new(None::<JwtClaims>))
            .boxed();

        self.router = self.router.nest("/auth", ar).boxed();

        self
    }

    pub fn get_routes(self) -> Router<BoxRoute> {
        self.router.boxed()
    }
}
