mod requester;
use core::default::Default;
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    sync::Arc,
};

use chrono::Duration;
use fxhash::FxHashMap;
use gw2api_model::Language;
use parking_lot::Mutex;
pub use requester::Requester;
use ureq::Agent;

use crate::{
    rate_limit::UreqRateLimit, Auth, Authenticated, BucketRateLimiter, Cache, Force, InMemoryCache,
    NoopCache, NoopRateLimiter, NotAuthenticated, NotForced, RateLimiter,
};

pub(crate) type Inflight = Mutex<FxHashMap<(TypeId, u64), Box<dyn Any + Send>>>;

pub struct Client<C: Cache, R: RateLimiter, A: Auth> {
    pub host: String,
    pub language: Language,
    agent: Agent,
    api_key: Option<String>,
    cache: Mutex<C>,
    inflight: Inflight,
    authenticated: PhantomData<A>,
    rate_limiter: PhantomData<R>,
}

impl Client<NoopCache, NoopRateLimiter, NotAuthenticated> {
    /// creates a new gw2 api client
    /// ### Warning
    /// this is not the same as [`Client::default`]!
    /// This initializes a client without any caching or rate limiting.
    /// If you want to use a default cache and rate limiter, use
    /// [`Client::default`].
    pub fn empty() -> Self {
        let rate_limiter = UreqRateLimit::new(Arc::new(Mutex::new(NoopRateLimiter {})));
        let agent = ureq::builder().middleware(rate_limiter).build();
        Self {
            host: "https://api.guildwars2.com".to_string(),
            language: Language::En,
            agent,
            api_key: None,
            cache: Mutex::new(NoopCache {}),
            inflight: Default::default(),
            authenticated: PhantomData,
            rate_limiter: PhantomData,
        }
    }
}

impl Default for Client<InMemoryCache, BucketRateLimiter, NotAuthenticated> {
    fn default() -> Self {
        let rate_limiter = UreqRateLimit::new(Arc::new(Mutex::new(BucketRateLimiter::default())));
        let agent = ureq::builder().middleware(rate_limiter).build();
        Self {
            host: "https://api.guildwars2.com".to_string(),
            language: Language::En,
            agent,
            api_key: None,
            cache: Mutex::new(InMemoryCache::default()),
            inflight: Default::default(),
            authenticated: PhantomData,
            rate_limiter: PhantomData,
        }
    }
}

/// constructing client
impl<C: Cache, R: RateLimiter, A: Auth> Client<C, R, A> {
    /// evicts all expired items in the cache
    pub fn cleanup_cache(&self) {
        self.cache.lock().cleanup();
    }

    /// sets the host name
    ///
    /// default is https://api.guildwars2.com (no trailing slash)
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
    pub fn api_key(self, key: impl Into<String>) -> Client<C, R, Authenticated> {
        self.cache.lock().wipe_authenticated();
        Client {
            host: self.host,
            language: self.language,
            agent: self.agent,
            api_key: Some(key.into()),
            cache: self.cache,
            inflight: self.inflight,
            authenticated: PhantomData,
            rate_limiter: PhantomData,
        }
    }

    /// sets the cache
    /// ## Example
    /// ```
    /// use gw2api_http::cache::InMemoryCache;
    /// use gw2api_http::Client;
    ///
    /// let client = Client::empty().cache(InMemoryCache::default());
    pub fn cache<NC: Cache>(self, cache: NC) -> Client<NC, R, A> {
        Client {
            host: self.host,
            language: self.language,
            agent: self.agent,
            api_key: self.api_key,
            cache: Mutex::new(cache),
            inflight: self.inflight,
            authenticated: PhantomData,
            rate_limiter: PhantomData,
        }
    }

    /// allows you to set the rate limiter, for example for sharing it between
    /// multiple clients ## Example
    /// ```
    /// use std::sync::{Arc, Mutex};
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
    ) -> Client<C, NR, A> {
        let rate_limiter = UreqRateLimit::new(rate_limiter);
        let agent = ureq::builder().middleware(rate_limiter).build();
        Client {
            host: self.host,
            language: self.language,
            agent,
            api_key: self.api_key,
            cache: self.cache,
            inflight: self.inflight,
            authenticated: PhantomData,
            rate_limiter: PhantomData,
        }
    }
}

impl<C: Cache + Send, R: RateLimiter + Sync, A: Auth + Sync> Requester for Client<C, R, A> {
    type Authenticated = A;
    type Caching = C;
    type ForceRefresh = NotForced;
    type RateLimiting = R;

    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, Self::Authenticated> {
        self
    }

    fn cache_duration(&self) -> Duration {
        Duration::zero()
    }
}

pub struct CachedRequest<'client, C: Cache, R: RateLimiter, A: Auth, F: Force + Sync> {
    client: &'client Client<C, R, A>,
    cache_duration: Duration,
    forced: PhantomData<F>,
}

impl<C: Cache + Send, R: RateLimiter + Sync, A: Auth + Sync, F: Force + Sync> Requester
    for CachedRequest<'_, C, R, A, F>
{
    type Authenticated = A;
    type Caching = C;
    type ForceRefresh = F;
    type RateLimiting = R;

    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, Self::Authenticated> {
        self.client
    }

    fn cache_duration(&self) -> Duration {
        self.cache_duration
    }
}
