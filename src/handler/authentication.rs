use std::str::FromStr;

use axum::extract::Path;

use axum::http::{StatusCode, Uri};
use axum::response::Redirect;
use axum::{extract::Extension, response::IntoResponse, Json};
use openidconnect::core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata};
use openidconnect::{ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, RedirectUrl};

use openidconnect::reqwest::async_http_client;

use crate::application::ApplicationState;
use crate::middleware::authentication::JwtClaims;

pub async fn claims(Extension(claims): Extension<Option<JwtClaims>>) -> impl IntoResponse {
    Json(claims.clone())
}

pub async fn oidc_client_login(
    Path(provider_name): Path<String>,
    Extension(_claims): Extension<Option<JwtClaims>>,
    Extension(state): Extension<ApplicationState>,
) -> Result<Redirect, StatusCode> {
    let provider = state
        .configuration
        .application
        .auth
        .oidc
        .get(&provider_name)
        .ok_or_else(|| StatusCode::NOT_FOUND)?;

    // Use OpenID Connect Discovery to fetch the provider metadata.
    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new(provider.issuer_url.clone()).map_err(|e| {
            tracing::error!("Parsing error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?,
        async_http_client,
    )
    .await
    .map_err(|e| {
        tracing::error!("Discovery Error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::debug!("Oidc Provider Information: {:?}", provider);

    // Create an OpenID Connect client by specifying the client ID, client secret, authorization URL
    // and token URL.
    let client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(provider.client_id.to_string()),
        Some(ClientSecret::new(provider.client_secret.to_string())),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(
        RedirectUrl::new(provider.redirect_url.clone()).map_err(|e| {
            tracing::error!("Parsing error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?,
    );
    tracing::debug!("Oidc Client Information: {:?}", client);

    let (auth_url, _csrf_token, _nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .url();

    Ok(Redirect::found(Uri::from_str(auth_url.as_str()).map_err(
        |e| {
            tracing::error!("Invalid Uri error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        },
    )?))
}
