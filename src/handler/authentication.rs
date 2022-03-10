use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;
use axum::{
    extract::{Path, Query},
    http::{StatusCode, Uri},
    response::{Headers, Redirect},
    {extract::Extension, response::IntoResponse, Json},
};

use hyper::header::{HeaderName, SET_COOKIE};
use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata, CoreTokenResponse},
    reqwest::async_http_client,
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndUserEmail, EndUserFamilyName,
    EndUserGivenName, EndUserPictureUrl, IssuerUrl, LocalizedClaim, Nonce, NonceVerifier,
    RedirectUrl, Scope,
};

use crate::{
    application::ApplicationState,
    middleware::authentication::{create_jwt_from_claims, JwtClaims},
};

pub async fn claims(Extension(claims): Extension<Option<JwtClaims>>) -> impl IntoResponse {
    Json(claims)
}

/// Oidc client creation helper
async fn oidc_client(
    provider_name: &str,
    state: &ApplicationState,
) -> Result<CoreClient, StatusCode> {
    let provider = state
        .configuration
        .get_oidc_provider(provider_name)
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
        // TODO: use config client_scopes value
        .add_scope(Scope::new("email".into()))
        .add_scope(Scope::new("profile".into()))
        .url();

    Ok(Redirect::to(Uri::from_str(auth_url.as_str()).map_err(
        |e| {
            tracing::error!("Invalid Uri error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        },
    )?))
}

// TODO: Actually store this nonce somewhere
pub struct NoNonce();

impl NonceVerifier for &NoNonce {
    fn verify(self, _nonce: Option<&Nonce>) -> Result<(), String> {
        Ok(())
    }
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

    let token_response: CoreTokenResponse = client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(async_http_client)
        .await
        .map_err(|e| {
            tracing::error!(
                "Request Token Error received for code: {code}. Error was: {:?}",
                e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    tracing::info!("Got valid Token Response: {:?}", token_response);

    let id_token = token_response
        .extra_fields()
        .id_token()
        .ok_or(anyhow!(
            "No ID token provided from provider: {provider_name}"
        ))
        .unwrap();

    let provider_claims = id_token
        .claims(&client.id_token_verifier(), &NoNonce())
        .unwrap()
        .clone();

    let claims = JwtClaims::new(
        provider_claims.subject().to_string(),
        state.configuration.http.full_base_url(),
        provider_claims
            .email()
            .unwrap_or(&EndUserEmail::new("".to_string()))
            .to_string(),
        provider_claims
            .given_name()
            .unwrap_or(&LocalizedClaim::<EndUserGivenName>::default())
            .get(None)
            .ok_or_else(|| {
                tracing::error!("Unable to extract given_name from id token.");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .to_string(),
        provider_claims
            .family_name()
            .unwrap_or(&LocalizedClaim::<EndUserFamilyName>::default())
            .get(None)
            .ok_or_else(|| {
                tracing::error!("Unable to extract given_name from id token.");
                StatusCode::INTERNAL_SERVER_ERROR
            })?
            .to_string(),
        provider_claims
            .picture()
            .unwrap_or(&LocalizedClaim::<EndUserPictureUrl>::default())
            .get(None)
            .map(|p| p.to_string()),
        state
            .configuration
            .application
            .auth
            .jwt_expiration_offset_seconds,
    );

    let h = Headers(vec![(
        SET_COOKIE,
        format!(
            "{}={}; path=/; HttpOnly",
            &state.configuration.application.auth.jwt_cookie_name,
            create_jwt_from_claims(&state.configuration, claims, None).unwrap()
        ),
    )]);

    Ok((h, StatusCode::OK))
}
