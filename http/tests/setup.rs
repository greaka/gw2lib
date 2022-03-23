use std::net::TcpStream;
use gw2api_http;
use gw2api_http::cache::InMemoryCache;
use gw2api_http::{Authenticated, Client};
use gw2api_http::rate_limit::NoopRateLimiter;

const API_KEY: &str = "564F181A-F0FC-114A-A55D-3C1DCD45F3767AF3848F-AB29-4EBF-9594-F91E6A75E015";
const HOST: &str = "localhost:52321";

pub fn setup() -> Client<InMemoryCache, NoopRateLimiter, Authenticated> {
    TcpStream::connect(HOST).expect("couldn't connect to the proxy");
    let client = Client::empty();
    let client = client.cache(InMemoryCache::default());
    let mut client = client.api_key(API_KEY);
    client.host("http://".to_string() + HOST);
    client
}
