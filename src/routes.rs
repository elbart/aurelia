use std::sync::Arc;

use axum::{
    handler::{get, post},
    routing::BoxRoute,
    AddExtensionLayer, Router,
};
use tower::layer::layer_fn;

use crate::{
    application::ApplicationState,
    handler::{create_user, get_tags, root},
    middleware::authentication::JwtAuthenticationMiddleware,
};

/// Router definitions should go here.
/// This returns a ``BoxRoute``, because it's otherwise hard to guess the
/// type before.
pub(crate) fn configure(state: Arc<ApplicationState>) -> Router<BoxRoute> {
    Router::new()
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/tags", get(get_tags))
        .layer(layer_fn(|inner| JwtAuthenticationMiddleware {
            inner,
            configuration: state.configuration.clone(),
        }))
        .layer(AddExtensionLayer::new(state))
        .boxed()
}
