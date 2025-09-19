mod requester;
use core::default::Default;
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    sync::{Arc, Once, Weak},
};

#[cfg(feature = "blocking")]
pub use blocking::Requester;
#[cfg(not(feature = "blocking"))]
pub use requester::Requester;
#[cfg(feature = "blocking")]
mod blocking;

use chrono::Duration;
use dashmap::DashMap;
use gw2lib_model::{Authenticated, Authentication, Language, NoAuthentication};
use reqwest::Client as ReqwestClient;
use static_init::dynamic;
use tokio::sync::Mutex;
use url::{ParseError, Url};

use crate::{
    BucketRateLimiter, Cache, NoopCache, NoopRateLimiter, RateLimiter,
    cache::{CleanupCache, InMemoryCache},
};

pub(crate) type Inflight = Arc<DashMap<(TypeId, u64), Box<dyn Any + Send + Sync>>>;

#[derive(Clone)]
#[must_use]
pub struct Client<
    C: Cache + Send + Sync + 'static,
    R: RateLimiter + Send + Sync + 'static,
    Auth: Authentication,
> {
    pub host: Arc<str>,
    pub language: Language,
    client: ReqwestClient,
    api_key: Option<Arc<str>>,
    identifier: Option<Arc<str>>,
    cache: Arc<C>,
    inflight: Inflight,
    rate_limiter: Arc<R>,
    _phantom: PhantomData<Auth>,
}

impl Client<NoopCache, NoopRateLimiter, NoAuthentication> {
    /// creates a new gw2 api client
    /// ### Remarks
    /// This is different from [`Client::default`]!
    /// This initializes a client without any caching or rate limiting.
    /// If you want to use a default cache and rate limiter, use
    /// [`Client::default`].
    pub fn empty() -> Self {
        let client = create_client();
        Self {
            host: "https://api.guildwars2.com".into(),
            language: Language::En,
            client,
            api_key: None,
            identifier: None,
            cache: Arc::new(NoopCache {}),
            inflight: Default::default(),
            rate_limiter: Arc::new(NoopRateLimiter),
            _phantom: PhantomData,
        }
    }
}

impl Default for Client<InMemoryCache, BucketRateLimiter, NoAuthentication> {
    fn default() -> Self {
        let client = create_client();
        let rate_limiter = Arc::new(BucketRateLimiter::default());
        let cache = Arc::new(InMemoryCache::default());
        periodically_cleanup_cache(cache.clone());
        Self {
            host: "https://api.guildwars2.com".into(),
            language: Language::En,
            client,
            api_key: None,
            identifier: None,
            cache,
            inflight: Default::default(),
            rate_limiter,
            _phantom: PhantomData,
        }
    }
}

/// constructing client
impl<C: Cache + Send + Sync + 'static, R: RateLimiter + Send + Sync + 'static, Auth: Authentication>
    Client<C, R, Auth>
{
    /// sets the host name
    ///
    /// default is `https://api.guildwars2.com` (no trailing slash)
    pub fn host(self, host: impl Into<Arc<str>>) -> Result<Self, ParseError> {
        let host = host.into();
        Url::parse(&host)?;
        Ok(Client { host, ..self })
    }

    /// sets the language
    pub fn language(self, language: impl Into<Language>) -> Self {
        Client {
            language: language.into(),
            ..self
        }
    }

    /// sets a new api key
    pub fn api_key(self, key: impl Into<Arc<str>>) -> Client<C, R, Authenticated> {
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
            _phantom: PhantomData,
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
    /// let characters: Vec<CharacterId> = client.ids::<Character>().unwrap();
    ///
    /// // new api key
    /// let client = client.api_key("<different subtoken>");
    /// let client = client.identifier(account.id);
    ///
    /// // cache hit
    /// let characters: Vec<CharacterId> = client.ids::<Character>().unwrap();
    /// ```
    pub fn identifier(self, id: impl Into<Arc<str>>) -> Self {
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
    pub fn cache<NC: Cache + Send + Sync + 'static>(self, cache: Arc<NC>) -> Client<NC, R, Auth> {
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
            _phantom: PhantomData,
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
        rate_limiter: Arc<NR>,
    ) -> Client<C, NR, Auth> {
        Client {
            host: self.host,
            language: self.language,
            client: self.client,
            api_key: self.api_key,
            identifier: self.identifier,
            cache: self.cache,
            inflight: self.inflight,
            rate_limiter,
            _phantom: PhantomData,
        }
    }
}

impl<C: Cache + Send + Sync + 'static, R: RateLimiter + Send + Sync + 'static, Auth: Authentication>
    Requester for Client<C, R, Auth>
{
    type Authenticated = Auth;
    type Caching = C;
    type Force = NotForced;
    type RateLimiting = R;

    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, Auth> {
        self
    }

    fn cache_duration(&self) -> Duration {
        Duration::zero()
    }
}

#[expect(private_bounds)]
#[must_use]
pub struct CachedRequest<
    'client,
    C: Cache + Send + Sync + 'static,
    R: RateLimiter + Send + Sync + 'static,
    Auth: Authentication,
    Forced: Force,
> {
    client: &'client Client<C, R, Auth>,
    cache_duration: Duration,
    _phantom: PhantomData<Forced>,
}

impl<
    Cacher: Cache + Send + Sync + 'static,
    RateLimit: RateLimiter + Send + Sync + 'static,
    Auth: Authentication,
    Forced: Force,
> Requester for CachedRequest<'_, Cacher, RateLimit, Auth, Forced>
{
    type Authenticated = Auth;
    type Caching = Cacher;
    type Force = Forced;
    type RateLimiting = RateLimit;

    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, Auth> {
        self.client
    }

    fn cache_duration(&self) -> Duration {
        self.cache_duration
    }
}

fn create_client() -> ReqwestClient {
    #[cfg(feature = "rustls")]
    {
        static INIT_CRYPTO: Once = Once::new();
        INIT_CRYPTO.call_once(|| {
            rustls::crypto::ring::default_provider()
                .install_default()
                .ok();
        });
    }
    ReqwestClient::builder()
        .gzip(true)
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        ))
        .build()
        .unwrap()
}

fn periodically_cleanup_cache(cache: Arc<dyn CleanupCache + Send + Sync + 'static>) {
    #[dynamic]
    static CACHES: Mutex<Vec<Weak<dyn CleanupCache + Send + Sync>>> =
        Mutex::new(Vec::with_capacity(1));

    let task = async move {
        let count = {
            let mut caches = CACHES.lock().await;
            caches.push(Arc::downgrade(&cache));
            caches.len()
        };
        if count == 1 {
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

pub struct Forced;
pub struct NotForced;
trait Force: Sized + Sync + 'static {
    const FORCE: bool;
}
impl Force for Forced {
    const FORCE: bool = true;
}
impl Force for NotForced {
    const FORCE: bool = false;
}

trait AllowsClient<ClientAuth> {}
impl<ClientAuth> AllowsClient<ClientAuth> for ClientAuth {}
impl AllowsClient<Authenticated> for NoAuthentication {}
