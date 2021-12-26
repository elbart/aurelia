use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{anyhow, Result};
use axum::http::{Request, Response};
use futures::future::BoxFuture;
use headers::{Cookie, HeaderMapExt};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tower::Service;
use uuid::Uuid;

use crate::configuration::{Application, Configuration};

#[derive(Clone)]
pub struct JwtAuthenticationMiddleware<S> {
    pub(crate) inner: S,
    pub(crate) configuration: Arc<Configuration>,
}

impl<S> JwtAuthenticationMiddleware<S> {
    pub fn new(inner: S, configuration: Arc<Configuration>) -> Self {
        Self {
            inner,
            configuration,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JwtClaims {
    sub: String,
    exp: usize,
}

impl JwtClaims {
    pub fn new(sub: String, exp_offset: usize) -> Self {
        Self {
            sub,
            // TODO: Use chrono here, if useful
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize
                + exp_offset,
        }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for JwtAuthenticationMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        // best practice is to clone the inner service like this
        // see https://github.com/tower-rs/tower/issues/547 for details
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        let app_cfg = self.configuration.application.clone();

        Box::pin(async move {
            tracing::trace!("JwtAuthenticationMiddleware before request processing...");
            match jwt_authentication(&req, &app_cfg) {
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
            let res: Response<ResBody> = inner.call(req).await?;
            tracing::trace!("JwtAuthenticationMiddleware after request processing...");

            Ok(res)
        })
    }
}

/// Takes an ``axum::Request`` and tries to extract and decode the
/// JWT from a configurable HTTP header.
pub(crate) fn jwt_authentication<ReqBody>(
    req: &Request<ReqBody>,
    configuration: &Application,
) -> Result<JwtClaims> {
    tracing::debug!(
        "Trying to authenticate JWT. Reading from HTTP header: {}",
        &configuration.auth.jwt_header_name
    );

    let auth_header_result = match req.headers().get(&configuration.auth.jwt_header_name) {
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
                    &configuration.auth.jwt_header_name
                );

                let token = decode::<JwtClaims>(
                    token,
                    &DecodingKey::from_secret(configuration.auth.jwt_secret.as_ref()),
                    &Validation::default(),
                )?;

                tracing::debug!("JWT decoded successfully. Claims: {:?}", &token.claims);

                Ok(token.claims)
            }
            any => Err(anyhow!(
                "Authentication via '{}' header failed. Unsupported header format: '{}'",
                &configuration.auth.jwt_header_name,
                any.join(" ")
            )),
        },
        None => Err(anyhow!(
            "Authentication via '{}' header failed. Header is missing.",
            &configuration.auth.jwt_header_name
        )),
    };

    if auth_header_result.is_ok() {
        return auth_header_result;
    } else {
        tracing::debug!("{}", auth_header_result.as_ref().unwrap_err().to_string());
    }

    match req.headers().typed_try_get::<Cookie>() {
        Ok(Some(cookie)) => {
            if let Some(token) = cookie.get(&configuration.auth.jwt_cookie_name) {
                tracing::debug!("Found cookie id token header. Trying to decode JWT.");

                let token = decode::<JwtClaims>(
                    token,
                    &DecodingKey::from_secret(configuration.auth.jwt_secret.as_ref()),
                    &Validation::default(),
                )?;

                tracing::debug!("JWT decoded successfully. Claims: {:?}", &token.claims);

                Ok(token.claims)
            } else {
                Err(anyhow!(
                    "No id token cookie with name '{}' found.",
                    &configuration.auth.jwt_cookie_name
                ))
            }
        }
        Ok(None) => Err(anyhow!("No Cookie header found.")),
        Err(e) => Err(e.into()),
    }
}

pub async fn create_jwt(cfg: &Configuration) -> String {
    let claims = JwtClaims::new(
        Uuid::new_v4().to_string(),
        cfg.application.auth.jwt_expiration_offset_seconds,
    );

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.application.auth.jwt_secret.as_ref()),
    )
    .unwrap()
}
