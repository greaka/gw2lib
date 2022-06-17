mod requester;
use core::default::Default;
use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use chrono::Duration;
use fxhash::FxHashMap;
use gw2api_model::Language;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
pub use requester::Requester;
use tokio::sync::Mutex;

use crate::{BucketRateLimiter, Cache, InMemoryCache, NoopCache, NoopRateLimiter, RateLimiter};

pub(crate) type Inflight = Arc<Mutex<FxHashMap<(TypeId, u64), Box<dyn Any + Send>>>>;

pub struct Client<C: Cache, R: RateLimiter, const AUTHENTICATED: bool> {
    pub host: String,
    pub language: Language,
    client: hyper::Client<HttpsConnector<HttpConnector>, hyper::Body>,
    api_key: Option<String>,
    cache: Mutex<C>,
    inflight: Inflight,
    rate_limiter: Arc<Mutex<R>>,
}

impl Client<NoopCache, NoopRateLimiter, false> {
    /// creates a new gw2 api client
    /// ### Warning
    /// this is not the same as [`Client::default`]!
    /// This initializes a client without any caching or rate limiting.
    /// If you want to use a default cache and rate limiter, use
    /// [`Client::default`].
    pub fn empty() -> Self {
        let client = create_client();
        let rate_limiter = Arc::new(Mutex::new(NoopRateLimiter {}));
        Self {
            host: "https://api.guildwars2.com".to_string(),
            language: Language::En,
            client,
            api_key: None,
            cache: Mutex::new(NoopCache {}),
            inflight: Default::default(),
            rate_limiter,
        }
    }
}

impl Default for Client<InMemoryCache, BucketRateLimiter, false> {
    fn default() -> Self {
        let client = create_client();
        let rate_limiter = Arc::new(Mutex::new(BucketRateLimiter::default()));
        Self {
            host: "https://api.guildwars2.com".to_string(),
            language: Language::En,
            client,
            api_key: None,
            cache: Mutex::new(InMemoryCache::default()),
            inflight: Default::default(),
            rate_limiter,
        }
    }
}

/// constructing client
impl<C: Cache, R: RateLimiter, const AUTHENTICATED: bool> Client<C, R, AUTHENTICATED> {
    /// evicts all expired items in the cache
    pub async fn cleanup_cache(&self) {
        self.cache.lock().await.cleanup().await;
    }

    /// sets the host name
    ///
    /// default is `https://api.guildwars2.com` (no trailing slash)
    pub fn host(&mut self, host: impl Into<String>) {
        self.host = host.into();
    }

    /// sets the language
    pub fn language(&mut self, language: impl Into<Language>) {
        self.language = language.into();
    }

    /// sets a new api key
    /// ### Warning
    /// this wipes the cache for all authenticated endpoints to prevent leaking
    /// account specific information
    pub async fn api_key(self, key: impl Into<String>) -> Client<C, R, AUTHENTICATED> {
        self.cache.lock().await.wipe_authenticated().await;
        Client {
            host: self.host,
            language: self.language,
            client: self.client,
            api_key: Some(key.into()),
            cache: self.cache,
            inflight: self.inflight,
            rate_limiter: self.rate_limiter,
        }
    }

    /// sets the cache
    /// ## Example
    /// ```
    /// use gw2api_http::cache::InMemoryCache;
    /// use gw2api_http::Client;
    ///
    /// let client = Client::empty().cache(InMemoryCache::default());
    pub fn cache<NC: Cache>(self, cache: NC) -> Client<NC, R, AUTHENTICATED> {
        Client {
            host: self.host,
            language: self.language,
            client: self.client,
            api_key: self.api_key,
            cache: Mutex::new(cache),
            inflight: self.inflight,
            rate_limiter: self.rate_limiter,
        }
    }

    /// allows you to set the rate limiter, for example for sharing it between
    /// multiple clients ## Example
    /// ```
    /// use std::sync::Arc;
    /// use tokio::sync::Mutex;
    /// use gw2api_http::cache::InMemoryCache;
    /// use gw2api_http::Client;
    /// use gw2api_http::rate_limit::BucketRateLimiter;
    ///
    /// let client = Client::empty().cache(InMemoryCache::default());
    /// let rate_limiter = Arc::new(Mutex::new(BucketRateLimiter::default()));
    /// let client = client.rate_limiter(rate_limiter.clone());
    /// let new_client = Client::default().rate_limiter(rate_limiter.clone());
    pub fn rate_limiter<NR: RateLimiter + 'static>(
        self,
        rate_limiter: Arc<Mutex<NR>>,
    ) -> Client<C, NR, AUTHENTICATED> {
        Client {
            host: self.host,
            language: self.language,
            client: self.client,
            api_key: self.api_key,
            cache: self.cache,
            inflight: self.inflight,
            rate_limiter,
        }
    }
}

impl<C: Cache + Send, R: RateLimiter + Sync, const AUTHENTICATED: bool>
    Requester<AUTHENTICATED, false> for Client<C, R, AUTHENTICATED>
{
    type Caching = C;
    type RateLimiting = R;

    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, AUTHENTICATED> {
        self
    }

    fn cache_duration(&self) -> Duration {
        Duration::zero()
    }
}

pub struct CachedRequest<
    'client,
    C: Cache,
    R: RateLimiter,
    const AUTHENTICATED: bool,
    const FORCE: bool,
> {
    client: &'client Client<C, R, AUTHENTICATED>,
    cache_duration: Duration,
}

impl<C: Cache + Send, R: RateLimiter + Sync, const AUTHENTICATED: bool, const FORCE: bool>
    Requester<AUTHENTICATED, FORCE> for CachedRequest<'_, C, R, AUTHENTICATED, FORCE>
{
    type Caching = C;
    type RateLimiting = R;

    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, AUTHENTICATED> {
        self.client
    }

    fn cache_duration(&self) -> Duration {
        self.cache_duration
    }
}

fn create_client() -> hyper::Client<HttpsConnector<HttpConnector>, hyper::Body> {
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();
    hyper::Client::builder().build(https)
}
