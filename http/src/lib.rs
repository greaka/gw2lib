pub mod cache;
mod client;
pub mod rate_limit;
use std::fmt::Display;

pub use client::Client;
pub use gw2api_model;

use crate::{
    cache::{Cache, InMemoryCache, NoopCache},
    rate_limit::{BucketRateLimiter, NoopRateLimiter, RateLimiter},
};

pub struct NotAuthenticated;
pub struct Authenticated;
pub trait Auth {
    const AUTHENTICATED: bool;
}

impl Auth for NotAuthenticated {
    const AUTHENTICATED: bool = false;
}

impl Auth for Authenticated {
    const AUTHENTICATED: bool = true;
}

struct ErrorUnsupportedEndpointQuery;

impl Display for ErrorUnsupportedEndpointQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "This endpoint does not support this operation")
    }
}

impl std::fmt::Debug for ErrorUnsupportedEndpointQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Display::fmt(&self, f)
    }
}

impl std::error::Error for ErrorUnsupportedEndpointQuery {}

struct ErrorNotAuthenticated;

impl Display for ErrorNotAuthenticated {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "This endpoint requires you to be authenticated")
    }
}

impl std::fmt::Debug for ErrorNotAuthenticated {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Display::fmt(&self, f)
    }
}

impl std::error::Error for ErrorNotAuthenticated {}

type EndpointResult<T> = Result<T, Box<dyn std::error::Error>>;
