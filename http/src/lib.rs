pub(crate) mod block;
pub mod cache;
mod client;
pub mod rate_limit;
pub use client::*;
pub use gw2api_model as model;
use thiserror::Error;
use tokio::sync::broadcast::error::RecvError;

use crate::{
    cache::{Cache, InMemoryCache, NoopCache},
    rate_limit::{BucketRateLimiter, NoopRateLimiter, RateLimiter},
};

#[derive(Error, Debug)]
pub enum EndpointError {
    #[error("unsupported query type for this endpoint")]
    UnsupportedEndpointQuery,
    #[error("endpoint requires authentication")]
    NotAuthenticated,
    #[error("unexpected rate limiting error")]
    RateLimiterCrashed,
    #[error("connection to gw2 api failed: {0}")]
    RequestFailed(#[from] hyper::Error),
    #[error("gw2 api returned non success status {0}: {1}")]
    ApiError(hyper::StatusCode, String),
    #[error("failed to retrieve item from already running request: {0}")]
    InflightReceiveFailed(#[from] RecvError),
    #[error("invalid json response: {0}")]
    InvalidJsonResponse(#[from] serde_json::Error),
}

type EndpointResult<T> = Result<T, EndpointError>;
