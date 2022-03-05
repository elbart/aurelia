use std::collections::HashMap;
use std::str::FromStr;

use axum::{
    extract::{Path, Query},
    http::{StatusCode, Uri},
    response::{Headers, Redirect},
    {extract::Extension, response::IntoResponse, Json},
};

use hyper::header::{HeaderName, SET_COOKIE};
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, RedirectUrl,
};

use crate::{application::ApplicationState, middleware::authentication::JwtClaims};

pub async fn claims(Extension(claims): Extension<Option<JwtClaims>>) -> impl IntoResponse {
    Json(claims)
}

/// Oidc client creation helper
async fn oidc_client(
    provider_name: &String,
    state: &ApplicationState,
) -> Result<CoreClient, StatusCode> {
    let provider = state
        .configuration
        .get_oidc_provider(&provider_name)
        .ok_or_else(|| {
            tracing::warn!("Provider not found: {}", provider_name);
            StatusCode::NOT_FOUND
        })?;

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
    Ok(client)
}

/// Callback handler for openid connect
pub async fn oidc_client_login(
    Path(provider_name): Path<String>,
    Extension(_claims): Extension<Option<JwtClaims>>,
    Extension(state): Extension<ApplicationState>,
) -> Result<Redirect, StatusCode> {
    let client = oidc_client(&provider_name, &state).await?;
    let (auth_url, _csrf_token, _nonce) = client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .url();

    Ok(Redirect::to(Uri::from_str(auth_url.as_str()).map_err(
        |e| {
            tracing::error!("Invalid Uri error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        },
    )?))
}

pub async fn oidc_client_login_cb(
    Query(query): Query<HashMap<String, String>>,
    Path(provider_name): Path<String>,
    Extension(_claims): Extension<Option<JwtClaims>>,
    Extension(state): Extension<ApplicationState>,
) -> Result<(Headers<Vec<(HeaderName, std::string::String)>>, StatusCode), StatusCode> {
    tracing::info!("{:?}", query);
    let client = oidc_client(&provider_name, &state).await?;
    let code = query.get("code").ok_or_else(|| {
        tracing::error!("Missing request query parameter 'code'");
        StatusCode::BAD_REQUEST
    })?;

    let token_response = client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(async_http_client)
        .await
        .map_err(|e| {
            tracing::error!("Request Token Error received: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let h = Headers(vec![(
        SET_COOKIE,
        format!(
            "{}={}; path=/; HttpOnly",
            &state.configuration.application.auth.jwt_cookie_name,
            token_response
                .extra_fields()
                .id_token()
                .cloned()
                .unwrap()
                .to_string()
        ),
    )]);

    tracing::info!("Got valid Token Response: {:?}", token_response);

    Ok((h, StatusCode::OK))
}
