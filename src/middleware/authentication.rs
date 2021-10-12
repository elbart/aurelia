use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{anyhow, Result};
use axum::http::{Request, Response};
use futures::future::BoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tower::Service;

use crate::configuration::{Application, Configuration};

#[derive(Clone)]
pub struct JwtAuthenticationMiddleware<S> {
    pub(crate) inner: S,
    pub(crate) configuration: Arc<Configuration>,
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
fn jwt_authentication<ReqBody>(
    req: &Request<ReqBody>,
    configuration: &Application,
) -> Result<JwtClaims> {
    tracing::debug!(
        "Trying to authenticate JWT. Reading from HTTP header: {}",
        &configuration.auth.jwt_header_name
    );
    match req.headers().get(&configuration.auth.jwt_header_name) {
        Some(value) => match value
            .to_str()
            .unwrap_or("")
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .as_slice()
        {
            ["Bearer", token] => {
                tracing::debug!("Found authorization header. Trying to find Bearer JWT.");

                let token = decode::<JwtClaims>(
                    token,
                    &DecodingKey::from_secret(configuration.auth.jwt_secret.as_ref()),
                    &Validation::default(),
                )?;

                tracing::debug!("JWT decoded successfully. Claims: {:?}", &token.claims);

                Ok(token.claims)
            }
            any => Err(anyhow!(
                "Unsupported authorization header format: '{}'",
                any.join(" ")
            )),
        },
        None => Err(anyhow!("Authorization header is missing.")),
    }
}
