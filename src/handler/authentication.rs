use axum::{extract::Extension, response::IntoResponse, Json};

use crate::middleware::authentication::JwtClaims;

pub async fn claims(Extension(claims): Extension<Option<JwtClaims>>) -> impl IntoResponse {
    Json(claims.clone())
}

pub async fn oidc_client_login() -> impl IntoResponse {
    Json(None::<String>)
}
