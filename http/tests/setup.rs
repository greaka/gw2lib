#![cfg(feature = "blocking")]

use std::{net::TcpStream, sync::Arc};

use gw2lib::{self, cache::InMemoryCache, rate_limit::NoopRateLimiter, Client};
use hyper::client::HttpConnector;
use tokio::sync::Mutex;

const API_KEY: &str = "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015";
const HOST: &str = "localhost:52321";

pub fn setup() -> Client<InMemoryCache, NoopRateLimiter, HttpConnector, true> {
    TcpStream::connect(HOST).expect("couldn't connect to the proxy");
    Client::empty()
        .cache(Arc::new(Mutex::new(InMemoryCache::default())))
        .api_key(API_KEY)
        .host_http("http://".to_string() + HOST)
}
