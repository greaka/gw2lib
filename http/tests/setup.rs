#![cfg(feature = "blocking")]
#![cfg(feature = "redis")]

use std::sync::Arc;

use gw2lib::{self, cache::InMemoryCache, rate_limit::RedisRateLimiter, Client};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

const API_KEY: &str = "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015";

pub fn setup(
) -> Client<InMemoryCache, RedisRateLimiter<Arc<redis::Client>>, HttpsConnector<HttpConnector>, true>
{
    let client = Arc::new(redis::Client::open("redis://localhost").unwrap());
    let rate_limiter = RedisRateLimiter::new(client).unwrap();
    Client::default()
        .api_key(API_KEY)
        .rate_limiter(rate_limiter)
}
