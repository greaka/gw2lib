mod requester;
use core::default::Default;
use std::{
    any::{Any, TypeId},
    sync::{Arc, Weak},
};

#[cfg(feature = "blocking")]
pub use blocking::Requester;
#[cfg(not(feature = "blocking"))]
pub use requester::Requester;
#[cfg(feature = "blocking")]
mod blocking;

use chrono::Duration;
use fxhash::FxHashMap;
use gw2api_model::Language;
use hyper::client::{connect::Connect, HttpConnector};
use hyper_rustls::HttpsConnector;
use static_init::dynamic;
use tokio::sync::Mutex;

use crate::{
    cache::CleanupCache, BucketRateLimiter, Cache, InMemoryCache, NoopCache, NoopRateLimiter,
    RateLimiter,
};

pub(crate) type Inflight = Arc<Mutex<FxHashMap<(TypeId, u64), Box<dyn Any + Send>>>>;

pub struct Client<
    C: Cache,
    R: RateLimiter,
    Conn: Connect + Clone + Send + Sync + 'static,
    const AUTHENTICATED: bool,
> {
    pub host: String,
    pub language: Language,
    client: hyper::Client<Conn, hyper::Body>,
    api_key: Option<String>,
    cache: Arc<Mutex<C>>,
    inflight: Inflight,
    rate_limiter: Arc<Mutex<R>>,
}

impl Client<NoopCache, NoopRateLimiter, HttpsConnector<HttpConnector>, false> {
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
            cache: Arc::new(Mutex::new(NoopCache {})),
            inflight: Default::default(),
            rate_limiter,
        }
    }
}

impl Default for Client<InMemoryCache, BucketRateLimiter, HttpsConnector<HttpConnector>, false> {
    fn default() -> Self {
        let client = create_client();
        let rate_limiter = Arc::new(Mutex::new(BucketRateLimiter::default()));
        let cache = Arc::new(Mutex::new(InMemoryCache::default()));
        periodically_cleanup_cache(cache.clone());
        Self {
            host: "https://api.guildwars2.com".to_string(),
            language: Language::En,
            client,
            api_key: None,
            cache,
            inflight: Default::default(),
            rate_limiter,
        }
    }
}

/// constructing client
impl<
        C: Cache,
        R: RateLimiter,
        Conn: Connect + Clone + Send + Sync + 'static,
        const AUTHENTICATED: bool,
    > Client<C, R, Conn, AUTHENTICATED>
{
    /// sets the host name
    ///
    /// default is `https://api.guildwars2.com` (no trailing slash)
    /// for non https hosts use [`Client::host_http`]
    pub fn host(
        self,
        host: impl Into<String>,
    ) -> Client<C, R, HttpsConnector<HttpConnector>, AUTHENTICATED> {
        let client = create_client();
        Client {
            host: host.into(),
            language: self.language,
            client,
            api_key: self.api_key,
            cache: self.cache,
            inflight: self.inflight,
            rate_limiter: self.rate_limiter,
        }
    }

    /// sets the non https host name
    ///
    /// for https hosts use [`Client::host`]
    pub fn host_http(self, host: impl Into<String>) -> Client<C, R, HttpConnector, AUTHENTICATED> {
        let client = hyper::Client::new();
        Client {
            host: host.into(),
            language: self.language,
            client,
            api_key: self.api_key,
            cache: self.cache,
            inflight: self.inflight,
            rate_limiter: self.rate_limiter,
        }
    }

    /// sets the language
    pub fn language(&mut self, language: impl Into<Language>) {
        self.language = language.into();
    }

    /// sets a new api key
    /// ### Warning
    /// this wipes the cache for all authenticated endpoints to prevent leaking
    /// account specific information
    #[cfg(not(feature = "blocking"))]
    pub async fn api_key(self, key: impl Into<String>) -> Client<C, R, Conn, true> {
        set_api_key(self, key).await
    }

    /// sets a new api key
    /// ### Warning
    /// this wipes the cache for all authenticated endpoints to prevent leaking
    /// account specific information
    #[cfg(feature = "blocking")]
    pub fn api_key(self, key: impl Into<String>) -> Client<C, R, Conn, true> {
        crate::block::block(set_api_key(self, key))
    }

    /// sets the cache
    /// ## Example
    /// ```
    /// use gw2api_http::cache::InMemoryCache;
    /// use gw2api_http::Client;
    ///
    /// let client = Client::empty().cache(InMemoryCache::default());
    pub fn cache<NC: Cache + Send + Sync + 'static>(
        self,
        cache: NC,
    ) -> Client<NC, R, Conn, AUTHENTICATED> {
        let cache = Arc::new(Mutex::new(cache));
        periodically_cleanup_cache(cache.clone());
        Client {
            host: self.host,
            language: self.language,
            client: self.client,
            api_key: self.api_key,
            cache,
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
    ) -> Client<C, NR, Conn, AUTHENTICATED> {
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

impl<
        C: Cache + Send,
        R: RateLimiter + Sync,
        Conn: Connect + Clone + Send + Sync + 'static,
        const AUTHENTICATED: bool,
    > requester::Requester<AUTHENTICATED, false> for Client<C, R, Conn, AUTHENTICATED>
{
    type Caching = C;
    type RateLimiting = R;
    type Connector = Conn;

    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, Self::Connector, AUTHENTICATED> {
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
    Conn: Connect + Clone + Send + Sync + 'static,
    const AUTHENTICATED: bool,
    const FORCE: bool,
> {
    client: &'client Client<C, R, Conn, AUTHENTICATED>,
    cache_duration: Duration,
}

impl<
        C: Cache + Send,
        R: RateLimiter + Sync,
        Conn: Connect + Clone + Send + Sync + 'static,
        const AUTHENTICATED: bool,
        const FORCE: bool,
    > requester::Requester<AUTHENTICATED, FORCE>
    for CachedRequest<'_, C, R, Conn, AUTHENTICATED, FORCE>
{
    type Caching = C;
    type RateLimiting = R;
    type Connector = Conn;

    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, Conn, AUTHENTICATED> {
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

async fn set_api_key<
    C: Cache,
    R: RateLimiter,
    Conn: Connect + Clone + Send + Sync + 'static,
    const AUTHENTICATED: bool,
>(
    client: Client<C, R, Conn, AUTHENTICATED>,
    key: impl Into<String>,
) -> Client<C, R, Conn, true> {
    client.cache.lock().await.wipe_authenticated().await;
    Client {
        host: client.host,
        language: client.language,
        client: client.client,
        api_key: Some(key.into()),
        cache: client.cache,
        inflight: client.inflight,
        rate_limiter: client.rate_limiter,
    }
}

fn periodically_cleanup_cache(cache: Arc<Mutex<dyn CleanupCache + Send + Sync + 'static>>) {
    #[dynamic]
    static CACHES: Mutex<Vec<Weak<Mutex<dyn CleanupCache + Send + Sync>>>> =
        Mutex::new(Vec::with_capacity(1));

    let task = async move {
        let mut caches = CACHES.lock().await;
        caches.push(Arc::downgrade(&cache));
        if caches.len() == 1 {
            drop(caches);
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;

                let mut caches = CACHES.lock().await;

                caches.retain(|cache| cache.upgrade().is_some());

                if caches.is_empty() {
                    break;
                }

                for cache in caches.iter() {
                    if let Some(cache) = cache.upgrade() {
                        cache.lock().await.cleanup().await;
                    }
                }
            }
        }
    };

    crate::block::spawn(task);
}
