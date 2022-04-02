use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{anyhow, Result};
use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use headers::{Cookie, HeaderMapExt};
use hyper::StatusCode;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::configuration::{Application, Configuration};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JwtClaims {
    pub sub: String,
    pub iss: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: Option<String>,
    pub exp: usize,
}

impl JwtClaims {
    pub fn new(
        sub: String,
        iss: String,
        email: String,
        given_name: String,
        family_name: String,
        picture: Option<String>,
        exp_offset: usize,
    ) -> Self {
        Self {
            sub,
            iss,
            email,
            given_name,
            family_name,
            picture,
            // TODO: Use chrono here, if useful
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize
                + exp_offset,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.given_name, self.family_name)
    }
}

pub async fn jwt_auth_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
    config: Application,
) -> impl IntoResponse {
    match jwt_authentication(&req, &config).await {
        Ok(claims) => {
            tracing::info!("Successfully authenticated JWT with claims: {:?}", claims);
            req.extensions_mut()
                .get_mut::<Option<JwtClaims>>()
                .unwrap_or(&mut None)
                .replace(claims);
        }
        Err(e) => {
            req.extensions_mut()
                .get::<Option<JwtClaims>>()
                .replace(&None);
            tracing::warn!("JWT authentication failed: {}", e);
        }
    }

    Ok::<_, StatusCode>(next.run(req).await)
}

pub async fn require_jwt_authentication<B>(
    req: Request<B>,
    next: Next<B>,
    config: Application,
) -> impl IntoResponse {
    let claims = req.extensions().get::<Option<JwtClaims>>().unwrap_or(&None);
    if claims.is_none() {
        return Err(Redirect::to(config.auth.login_path.as_str()));
    }

    Ok::<_, Redirect>(next.run(req).await)
}

/// Takes an ``axum::Request`` and tries to extract and decode the
/// JWT from a configurable HTTP header.
pub(crate) async fn jwt_authentication<ReqBody>(
    req: &Request<ReqBody>,
    app_config: &Application,
) -> Result<JwtClaims> {
    tracing::debug!(
        "Trying to authenticate JWT. Reading from HTTP header: {}",
        &app_config.auth.jwt_header_name
    );

    let auth_header_result = match req.headers().get(&app_config.auth.jwt_header_name) {
        Some(value) => match value
            .to_str()
            .unwrap_or("")
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .as_slice()
        {
            ["Bearer", token] => {
                tracing::debug!(
                    "Found '{}' header. Trying to find Bearer JWT.",
                    &app_config.auth.jwt_header_name
                );

                let token = verify_token(token, app_config).await?;
                tracing::debug!("JWT decoded successfully. Claims: {:?}", &token.claims);

                Ok(token.claims)
            }
            any => Err(anyhow!(
                "Authentication via '{}' header failed. Unsupported header format: '{}'",
                &app_config.auth.jwt_header_name,
                any.join(" ")
            )),
        },
        None => Err(anyhow!(
            "Authentication via '{}' header failed. Header is missing.",
            &app_config.auth.jwt_header_name
        )),
    };

    if auth_header_result.is_ok() {
        return auth_header_result;
    } else {
        tracing::debug!("{}", auth_header_result.as_ref().unwrap_err().to_string());
    }

    match req.headers().typed_try_get::<Cookie>() {
        Ok(Some(cookie)) => {
            if let Some(token) = cookie.get(&app_config.auth.jwt_cookie_name) {
                tracing::debug!("Found cookie id token header. Trying to decode JWT.");

                let token = verify_token(token, app_config).await?;
                tracing::debug!("JWT decoded successfully. Claims: {:?}", &token.claims);

                Ok(token.claims)
            } else {
                Err(anyhow!(
                    "No id token cookie with name '{}' found.",
                    &app_config.auth.jwt_cookie_name
                ))
            }
        }
        Ok(None) => Err(anyhow!("No Cookie header found.")),
        Err(e) => Err(e.into()),
    }
}

async fn verify_token(token: &str, app_config: &Application) -> Result<TokenData<JwtClaims>> {
    let token = decode::<JwtClaims>(
        token,
        &DecodingKey::from_rsa_pem(app_config.auth.jwtrsapublickey.as_bytes())?,
        &Validation::new(Algorithm::from_str(&app_config.auth.jwt_algorithm)?),
    )?;

    Ok(token)
}

pub(crate) fn create_jwt_from_claims(
    cfg: &Configuration,
    claims: JwtClaims,
    force_rsa: Option<bool>,
) -> Result<String> {
    let rsa = if let Some(force) = force_rsa {
        force
    } else {
        cfg.application.auth.jwt_algorithm == "RS256"
    };

    if rsa {
        let key = EncodingKey::from_rsa_pem(cfg.application.auth.jwtrsaprivatekey.as_bytes())?;
        encode(&Header::new(Algorithm::RS256), &claims, &key).map_err(Into::into)
    } else {
        let key = EncodingKey::from_secret(cfg.application.auth.jwt_secret.as_ref());
        encode(&Header::new(Algorithm::HS256), &claims, &key).map_err(Into::into)
    }
}

pub async fn create_jwt(
    cfg: &Configuration,
    user_id: Option<Uuid>,
    force_rsa: Option<bool>,
) -> Result<String> {
    let sub = user_id.unwrap_or_else(Uuid::new_v4).to_string();

    let claims = JwtClaims::new(
        sub,
        "http://nutrist.local".into(),
        "xelbartusx@gmail.com".into(),
        "Toni".into(),
        "Tester".into(),
        None,
        cfg.application.auth.jwt_expiration_offset_seconds,
    );

    create_jwt_from_claims(cfg, claims, force_rsa)
}
