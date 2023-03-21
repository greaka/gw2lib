#![cfg(feature = "blocking")]

#[cfg(not(feature = "redis"))]
use gw2lib::rate_limit::NoopRateLimiter;
#[cfg(feature = "redis")]
use gw2lib::rate_limit::RedisRateLimiter;
use gw2lib::{self, cache::InMemoryCache, Client};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

const API_KEY: &str = "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015";

#[cfg(not(feature = "redis"))]
pub fn setup() -> Client<InMemoryCache, NoopRateLimiter, HttpsConnector<HttpConnector>, true> {
    let rate_limiter = NoopRateLimiter {};
    Client::default()
        .api_key(API_KEY)
        .rate_limiter(rate_limiter)
}

#[cfg(feature = "redis")]
pub fn setup() -> Client<InMemoryCache, RedisRateLimiter, HttpsConnector<HttpConnector>, true> {
    let client = redis::Client::open("redis://localhost").unwrap();
    let rate_limiter = RedisRateLimiter::new(client).unwrap();
    Client::default()
        .api_key(API_KEY)
        .rate_limiter(rate_limiter)
}
