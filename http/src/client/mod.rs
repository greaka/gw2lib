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
use dashmap::DashMap;
use gw2lib_model::Language;
use hyper::client::{connect::Connect, HttpConnector};
use hyper_rustls::HttpsConnector;
use static_init::dynamic;
use tokio::sync::Mutex;

use crate::{
    cache::{CleanupCache, InMemoryCache},
    BucketRateLimiter, Cache, NoopCache, NoopRateLimiter, RateLimiter,
};

pub(crate) type Inflight = Arc<DashMap<(TypeId, u64), Box<dyn Any + Send + Sync>>>;

#[must_use]
pub struct Client<
    C: Cache + Send + Sync + 'static,
    R: RateLimiter + Send + Sync + 'static,
    Conn: Connect + Clone + Send + Sync + 'static,
    const AUTHENTICATED: bool,
> {
    pub host: String,
    pub language: Language,
    client: hyper::Client<Conn, hyper::Body>,
    api_key: Option<String>,
    identifier: Option<String>,
    cache: Arc<C>,
    inflight: Inflight,
    rate_limiter: R,
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
        let rate_limiter = NoopRateLimiter {};
        Self {
            host: "https://api.guildwars2.com".to_string(),
            language: Language::En,
            client,
            api_key: None,
            identifier: None,
            cache: Arc::new(NoopCache {}),
            inflight: Default::default(),
            rate_limiter,
        }
    }
}

impl Default for Client<InMemoryCache, BucketRateLimiter, HttpsConnector<HttpConnector>, false> {
    fn default() -> Self {
        let client = create_client();
        let rate_limiter = BucketRateLimiter::default();
        let cache = Arc::new(InMemoryCache::default());
        periodically_cleanup_cache(cache.clone());
        Self {
            host: "https://api.guildwars2.com".to_string(),
            language: Language::En,
            client,
            api_key: None,
            identifier: None,
            cache,
            inflight: Default::default(),
            rate_limiter,
        }
    }
}

/// constructing client
impl<
        C: Cache + Send + Sync + 'static,
        R: RateLimiter + Send + Sync + 'static,
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
            identifier: self.identifier,
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
            identifier: self.identifier,
            cache: self.cache,
            inflight: self.inflight,
            rate_limiter: self.rate_limiter,
        }
    }

    /// sets the language
    pub fn language(self, language: impl Into<Language>) -> Self {
        Client {
            language: language.into(),
            ..self
        }
    }

    /// sets a new api key
    pub fn api_key(self, key: impl Into<String>) -> Client<C, R, Conn, true> {
        let key = key.into();
        Client {
            host: self.host,
            language: self.language,
            client: self.client,
            api_key: Some(key.clone()),
            identifier: Some(key),
            cache: self.cache,
            inflight: self.inflight,
            rate_limiter: self.rate_limiter,
        }
    }

    /// sets a new identifier
    ///
    /// Identifiers are used to identify authentications in the cache.
    /// Defaults to the api key
    ///
    /// ## Example
    /// ```no_run
    /// use gw2lib::{Client, Requester};
    /// use gw2lib_model::authenticated::{
    ///     account::Account,
    ///     characters::{Character, CharacterId},
    /// };
    ///
    /// let client = Client::default().api_key("<subtoken>");
    /// let account: Account = client.get().unwrap();
    /// let client = client.identifier(&account.id);
    ///
    /// // make a request
    /// let characters: Vec<CharacterId> = client.ids::<Character, CharacterId>().unwrap();
    ///
    /// let client = Client::default().api_key("<different subtoken>");
    /// let client = client.identifier(account.id);
    ///
    /// // cache hit
    /// let characters: Vec<CharacterId> = client.ids::<Character, CharacterId>().unwrap();
    /// ```
    pub fn identifier(self, id: impl Into<String>) -> Self {
        Client {
            identifier: Some(id.into()),
            ..self
        }
    }

    /// sets the cache
    /// ## Example
    /// ```
    /// use gw2lib::cache::InMemoryCache;
    /// use gw2lib::Client;
    /// use std::sync::{Arc, Mutex};
    ///
    /// let cache = Arc::new(InMemoryCache::default());
    /// let client = Client::empty().cache(cache);
    pub fn cache<NC: Cache + Send + Sync + 'static>(
        self,
        cache: Arc<NC>,
    ) -> Client<NC, R, Conn, AUTHENTICATED> {
        periodically_cleanup_cache(cache.clone());
        Client {
            host: self.host,
            language: self.language,
            client: self.client,
            api_key: self.api_key,
            identifier: self.identifier,
            cache,
            inflight: self.inflight,
            rate_limiter: self.rate_limiter,
        }
    }

    /// allows you to set the rate limiter, for example for sharing it between
    /// multiple clients ## Example
    /// ```
    /// use std::sync::Arc;
    /// use gw2lib::cache::InMemoryCache;
    /// use gw2lib::Client;
    /// use gw2lib::rate_limit::BucketRateLimiter;
    ///
    /// let client = Client::empty().cache(Arc::new(InMemoryCache::default()));
    /// let rate_limiter = Arc::new(BucketRateLimiter::default());
    /// let client = client.rate_limiter(rate_limiter.clone());
    /// let new_client = Client::default().rate_limiter(rate_limiter.clone());
    pub fn rate_limiter<NR: RateLimiter + Send + Sync + 'static>(
        self,
        rate_limiter: NR,
    ) -> Client<C, NR, Conn, AUTHENTICATED> {
        Client {
            host: self.host,
            language: self.language,
            client: self.client,
            api_key: self.api_key,
            identifier: self.identifier,
            cache: self.cache,
            inflight: self.inflight,
            rate_limiter,
        }
    }
}

impl<
        C: Cache + Send + Sync + 'static,
        R: RateLimiter + Send + Sync + 'static,
        Conn: Connect + Clone + Send + Sync + 'static,
        const AUTHENTICATED: bool,
    > requester::Requester<AUTHENTICATED, false> for Client<C, R, Conn, AUTHENTICATED>
{
    type Caching = C;
    type Connector = Conn;
    type RateLimiting = R;

    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, Self::Connector, AUTHENTICATED> {
        self
    }

    fn cache_duration(&self) -> Duration {
        Duration::zero()
    }
}

impl<
        C: Cache + Send + Sync + 'static,
        R: RateLimiter + Clone + Send + Sync + 'static,
        Conn: Connect + Clone + Send + Sync + 'static,
        const AUTHENTICATED: bool,
    > Clone for Client<C, R, Conn, AUTHENTICATED>
{
    fn clone(&self) -> Self {
        Self {
            host: self.host.clone(),
            language: self.language,
            client: self.client.clone(),
            api_key: self.api_key.clone(),
            identifier: self.identifier.clone(),
            cache: self.cache.clone(),
            inflight: self.inflight.clone(),
            rate_limiter: self.rate_limiter.clone(),
        }
    }
}

#[must_use]
pub struct CachedRequest<
    'client,
    C: Cache + Send + Sync + 'static,
    R: RateLimiter + Send + Sync + 'static,
    Conn: Connect + Clone + Send + Sync + 'static,
    const AUTHENTICATED: bool,
    const FORCE: bool,
> {
    client: &'client Client<C, R, Conn, AUTHENTICATED>,
    cache_duration: Duration,
}

impl<
        C: Cache + Send + Sync + 'static,
        R: RateLimiter + Send + Sync + 'static,
        Conn: Connect + Clone + Send + Sync + 'static,
        const AUTHENTICATED: bool,
        const FORCE: bool,
    > requester::Requester<AUTHENTICATED, FORCE>
    for CachedRequest<'_, C, R, Conn, AUTHENTICATED, FORCE>
{
    type Caching = C;
    type Connector = Conn;
    type RateLimiting = R;

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

fn periodically_cleanup_cache(cache: Arc<dyn CleanupCache + Send + Sync + 'static>) {
    #[dynamic]
    static CACHES: Mutex<Vec<Weak<dyn CleanupCache + Send + Sync>>> =
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
                        cache.cleanup().await;
                    }
                }
            }
        }
    };

    crate::block::spawn(task);
}
