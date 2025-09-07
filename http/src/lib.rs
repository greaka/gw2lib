pub(crate) mod block;
pub mod cache;
mod client;
pub mod rate_limit;
// todo: multi account ux
pub use client::*;
pub use gw2lib_model as model;
use thiserror::Error;
use tokio::sync::broadcast::error::RecvError;

use crate::{
    cache::{Cache, NoopCache},
    rate_limit::{BucketRateLimiter, NoopRateLimiter, RateLimiter},
};

#[derive(Error, Debug)]
pub enum EndpointError {
    #[error("unsupported query type for this endpoint")]
    UnsupportedEndpointQuery,
    #[error("endpoint requires authentication")]
    NotAuthenticated,
    #[error("unexpected rate limiting error: {0}")]
    RateLimiterCrashed(String),
    #[error("requested too many tokens at once")]
    RateLimiterBucketExceeded,
    #[error("connection to gw2 api failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("gw2 api returned non success status: {0}")]
    ApiError(ApiError),
    #[error("failed to retrieve item from already running request: {0}")]
    InflightReceiveFailed(#[from] RecvError),
    #[error("invalid json response: {0}")]
    InvalidJsonResponse(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("invalid key")]
    Unauthorized,
    #[error("account does not have game access")]
    MissingGameAccess,
    #[error("too many requests")]
    RateLimited,
    #[error("{0}: {1}")]
    Other(reqwest::StatusCode, String),
}

type EndpointResult<T> = Result<T, EndpointError>;
