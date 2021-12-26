use anyhow::Result;
use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RequestContext {
    pub uri: String,
}

#[async_trait]
impl<B> FromRequest<B> for RequestContext
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Ok(RequestContext {
            uri: req.uri().path().to_string(),
        })
    }
}
