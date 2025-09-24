#![cfg(feature = "blocking")]

#[cfg(not(feature = "redis"))]
use gw2lib::rate_limit::BucketRateLimiter;
#[cfg(feature = "redis")]
use gw2lib::rate_limit::RedisRateLimiter;
use gw2lib::{self, Client, cache::InMemoryCache};
use gw2lib_model::Authenticated;

const API_KEY: &str = "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015";

#[cfg(not(feature = "redis"))]
pub fn setup() -> Client<InMemoryCache, BucketRateLimiter, Authenticated> {
    let apikey = api_key();
    Client::default().api_key(apikey)
}

#[cfg(feature = "redis")]
pub fn setup() -> Client<InMemoryCache, RedisRateLimiter, Authenticated> {
    let apikey = api_key();
    let client = redis::Client::open("redis://localhost/?protocol=resp3").unwrap();
    let rate_limiter = RedisRateLimiter::new(client).unwrap();
    Client::default()
        .api_key(apikey)
        .rate_limiter(rate_limiter.into())
}

fn api_key() -> String {
    std::env::var("GW2_API_KEY")
        .ok()
        .and_then(|x| (!x.is_empty()).then_some(x))
        .unwrap_or(API_KEY.into())
}

pub fn character_name() -> String {
    std::env::var("GW2_TESTING_CHAR")
        .ok()
        .and_then(|x| (!x.is_empty()).then_some(x))
        .unwrap_or("Eff Testing Ele".to_string())
}
