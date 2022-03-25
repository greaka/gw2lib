mod requester;
use std::{marker::PhantomData, sync::Mutex};

use chrono::Duration;
use gw2api_model::Language;
pub use requester::Requester;
use ureq::Agent;

use crate::{
    rate_limit::UreqRateLimit, Auth, Authenticated, BucketRateLimiter, Cache, Force, InMemoryCache,
    NoopCache, NoopRateLimiter, NotAuthenticated, NotForced, RateLimiter,
};

pub struct Client<C: Cache, R: RateLimiter, A: Auth> {
    pub host: String,
    pub language: Language,
    agent: Agent,
    api_key: Option<String>,
    cache: Mutex<C>,
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
        let rate_limiter = UreqRateLimit::new(NoopRateLimiter {});
        let agent = ureq::builder().middleware(rate_limiter).build();
        Self {
            host: "https://api.guildwars2.com".to_string(),
            language: Language::En,
            agent,
            api_key: None,
            cache: Mutex::new(NoopCache {}),
            authenticated: PhantomData,
            rate_limiter: PhantomData,
        }
    }
}

impl Default for Client<InMemoryCache, BucketRateLimiter, NotAuthenticated> {
    fn default() -> Self {
        let rate_limiter = UreqRateLimit::new(BucketRateLimiter::default());
        let agent = ureq::builder().middleware(rate_limiter).build();
        Self {
            host: "https://api.guildwars2.com".to_string(),
            language: Language::En,
            agent,
            api_key: None,
            cache: Mutex::new(InMemoryCache::default()),
            authenticated: PhantomData,
            rate_limiter: PhantomData,
        }
    }
}

/// constructing client
impl<C: Cache, R: RateLimiter, A: Auth> Client<C, R, A> {
    pub fn host(&mut self, host: impl Into<String>) {
        self.host = host.into();
    }

    pub fn language(&mut self, language: impl Into<Language>) {
        self.language = language.into();
    }

    pub fn api_key(self, key: impl Into<String>) -> Client<C, R, Authenticated> {
        self.cache.lock().unwrap().wipe();
        Client {
            host: self.host,
            language: self.language,
            agent: self.agent,
            api_key: Some(key.into()),
            cache: self.cache,
            authenticated: PhantomData,
            rate_limiter: PhantomData,
        }
    }

    pub fn cache<NC: Cache>(self, cache: NC) -> Client<NC, R, A> {
        Client {
            host: self.host,
            language: self.language,
            agent: self.agent,
            api_key: self.api_key,
            cache: Mutex::new(cache),
            authenticated: PhantomData,
            rate_limiter: PhantomData,
        }
    }

    pub fn rate_limiter<NR: RateLimiter + 'static>(self, rate_limiter: NR) -> Client<C, NR, A> {
        let rate_limiter = UreqRateLimit::new(rate_limiter);
        let agent = ureq::builder().middleware(rate_limiter).build();
        Client {
            host: self.host,
            language: self.language,
            agent,
            api_key: self.api_key,
            cache: self.cache,
            authenticated: PhantomData,
            rate_limiter: PhantomData,
        }
    }
}

impl<C: Cache, R: RateLimiter, A: Auth> Requester for Client<C, R, A> {
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

pub struct CachedRequest<'client, C: Cache, R: RateLimiter, A: Auth, F: Force> {
    client: &'client Client<C, R, A>,
    cache_duration: Duration,
    forced: PhantomData<F>,
}

impl<C: Cache, R: RateLimiter, A: Auth, F: Force> Requester for CachedRequest<'_, C, R, A, F> {
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
