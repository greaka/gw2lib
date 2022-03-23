use std::{fmt::Display, hash::Hash, marker::PhantomData, sync::Mutex};

use chrono::{Duration, Utc};
use gw2api_model::{BulkEndpoint, Endpoint, EndpointWithId, FixedEndpoint, Language};
use serde::de::DeserializeOwned;
use ureq::{Agent, Request, Response};

use crate::{
    rate_limit::UreqRateLimit, Auth, Authenticated, BucketRateLimiter, Cache, EndpointResult,
    ErrorNotAuthenticated, ErrorUnsupportedEndpointQuery, InMemoryCache, NoopCache,
    NoopRateLimiter, NotAuthenticated, RateLimiter,
};

pub struct Client<C: Cache, R: RateLimiter, A: Auth> {
    pub host:      String,
    pub language:  Language,
    agent:         Agent,
    api_key:       Option<String>,
    cache:         Mutex<C>,
    authenticated: PhantomData<A>,
    rate_limiter:  PhantomData<R>,
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
            host:          self.host,
            language:      self.language,
            agent:         self.agent,
            api_key:       Some(key.into()),
            cache:         self.cache,
            authenticated: PhantomData,
            rate_limiter:  PhantomData,
        }
    }

    pub fn cache<NC: Cache>(self, cache: NC) -> Client<NC, R, A> {
        Client {
            host:          self.host,
            language:      self.language,
            agent:         self.agent,
            api_key:       self.api_key,
            cache:         Mutex::new(cache),
            authenticated: PhantomData,
            rate_limiter:  PhantomData,
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

/// usage once constructed
impl<C: Cache, R: RateLimiter, A: Auth> Client<C, R, A> {
    /// call the fixed endpoint
    pub fn get<T: DeserializeOwned + Clone + FixedEndpoint + 'static>(&self) -> EndpointResult<T> {
        self.get_or_ids::<T, T>()
    }

    /// request a single item
    pub fn single<
        T: DeserializeOwned + Clone + EndpointWithId<I> + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
        id: I,
    ) -> EndpointResult<T> {
        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(cached) = cache.get(&id) {
                return Ok(cached);
            }
        }

        let url = format!("{}/{}/{}", self.host, T::URL, id);
        let request = self.agent.get(&url);
        let request = self.set_common_headers_and_rate_limit::<T>(request)?;

        let response = request.call()?;
        let result = self.cache_response(&id, response)?;

        Ok(result)
    }

    /// request all available ids
    pub fn ids<
        T: DeserializeOwned + EndpointWithId<I> + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<I>> {
        self.get_or_ids::<T, Vec<I>>()
    }

    /// request multiple ids at once
    pub fn many<
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
        mut ids: Vec<I>,
    ) -> EndpointResult<Vec<T>> {
        let mut result = Vec::with_capacity(ids.len());
        ids = self.extract_many_from_cache(ids, &mut result);
        if ids.is_empty() {
            return Ok(result);
        }

        let url = format!("{}/{}", self.host, T::URL);
        let chunks = join_ids(&ids)?;
        for rest in chunks {
            let request = self.agent.get(&url).query("ids", &rest);
            let request = self.set_common_headers_and_rate_limit::<T>(request)?;

            let response = request.call()?;
            self.cache_response_many(response, &mut result)?;
        }

        Ok(result)
    }

    /// requests a page of items and returns the number of total items across
    /// all pages
    pub fn page<
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
        page: usize,
        page_size: u8,
        result: &mut Vec<T>,
    ) -> EndpointResult<usize> {
        let url = format!("{}/{}", self.host, T::URL);
        let request = self
            .agent
            .get(&url)
            .query("page", page.to_string().as_str())
            .query("page_size", page_size.to_string().as_str());
        let request = self.set_common_headers_and_rate_limit::<T>(request)?;

        let response = request.call()?;
        let count = response
            .header("x-result-total")
            .and_then(|x| x.parse().ok())
            .unwrap_or(0);
        self.cache_response_many(response, result)?;

        Ok(count)
    }

    /// requests all items using the most efficient method available
    /// ### Remarks
    /// for most endpoints this means using [`Self::get_all_by_paging`].
    /// You might not want this for endpoints that change rapidly (like
    /// requesting listings on the tp). In that case, fall back to
    /// [`Self::get_all_by_requesting_ids`] or simply call [`Self::many`].
    pub fn all<
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        if T::ALL {
            self.get_all_by_ids_all()
        } else if T::PAGING {
            self.get_all_by_paging()
        } else {
            self.get_all_by_requesting_ids()
        }
    }

    /// Gets all items by querying ids=all
    ///
    /// use [`Self::all`] to use the most efficient way to request all items
    pub fn get_all_by_ids_all<
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        if !T::ALL {
            return Err(Box::new(ErrorUnsupportedEndpointQuery));
        }

        let url = format!("{}/{}", self.host, T::URL);
        let request = self.agent.get(&url).query("ids", "all");
        let request = self.set_common_headers_and_rate_limit::<T>(request)?;

        let response = request.call()?;
        let count = response
            .header("x-result-total")
            .and_then(|x| x.parse().ok())
            .unwrap_or(0);
        let mut result = Vec::with_capacity(count);
        self.cache_response_many(response, &mut result)?;

        Ok(result)
    }

    /// Gets all items by querying all pages
    ///
    /// use [`Self::all`] to use the most efficient way to request all items
    pub fn get_all_by_paging<
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        if !T::PAGING {
            return Err(Box::new(ErrorUnsupportedEndpointQuery));
        }

        let mut result = Vec::with_capacity(200);
        let max_items = self.page(0, 200, &mut result)?;
        let remaining = max_items.saturating_sub(200);
        result.reserve_exact(remaining);

        let pages = ((remaining as f64) / 200_f64).ceil() as usize;
        for page in 0..pages {
            self.page(page + 1, 200, &mut result)?;
        }

        Ok(result)
    }

    /// Gets all items by querying all ids
    ///
    /// use [`Self::all`] to use the most efficient way to request all items
    pub fn get_all_by_requesting_ids<
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        let ids = self.ids::<T, I>()?;
        self.many(ids)
    }
}

/// private impls
impl<C: Cache, R: RateLimiter, A: Auth> Client<C, R, A> {
    fn get_or_ids<T: DeserializeOwned + Endpoint + 'static, K: DeserializeOwned + Clone + 'static>(&self) -> EndpointResult<K> {
        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(cached) = cache.get(&()) {
                return Ok(cached);
            }
        }

        let url = format!("{}/{}", self.host, T::URL);
        let request = self.agent.get(&url);
        let request = self.set_common_headers_and_rate_limit::<T>(request)?;

        let response = request.call()?;
        let result = self.cache_response(&(), response)?;

        Ok(result)
    }

    fn set_common_headers_and_rate_limit<T: Endpoint>(
        &self,
        mut request: Request,
    ) -> EndpointResult<Request> {
        if T::AUTHENTICATED && !A::AUTHENTICATED {
            return Err(Box::new(ErrorNotAuthenticated));
        }
        request = request.set("X-Schema-Version", T::VERSION);
        if T::AUTHENTICATED {
            request = request.set(
                "Authorization",
                &format!("Bearer {}", self.api_key.as_ref().unwrap()),
            );
        }
        if T::LOCALE {
            request = request.query("lang", self.language.as_str());
        }

        Ok(request)
    }

    /// returns the remaining ids not found in cache
    fn extract_many_from_cache<I: Hash + 'static, K: Clone + 'static>(
        &self,
        ids: Vec<I>,
        result: &mut Vec<K>,
    ) -> Vec<I> {
        let mut cache = self.cache.lock().unwrap();
        ids.into_iter()
            .filter(|i| {
                if let Some(cached) = cache.get(i) {
                    result.push(cached);
                    false
                } else {
                    true
                }
            })
            .collect()
    }

    fn cache_response<I: Hash + 'static, K: DeserializeOwned + Clone + 'static>(
        &self,
        id: &I,
        response: Response,
    ) -> Result<K, std::io::Error> {
        let expires = get_expire_from_header(&response);
        let result: K = response.into_json()?;
        let res = result.clone();
        let expires = Utc::now().naive_utc() + expires;
        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(id, res, expires);
        }
        Ok(result)
    }

    fn cache_response_many<
        I: Hash + 'static,
        K: DeserializeOwned + EndpointWithId<I> + Clone + 'static,
    >(
        &self,
        response: Response,
        result: &mut Vec<K>,
    ) -> Result<(), std::io::Error> {
        let expires = get_expire_from_header(&response);
        let res: Vec<K> = response.into_json()?;
        let expires = Utc::now().naive_utc() + expires;
        {
            let mut cache = self.cache.lock().unwrap();
            for t in res {
                cache.insert(t.id(), t.clone(), expires);
                result.push(t);
            }
        }
        Ok(())
    }
}

/// concatenates ids, separated by comma: 1,2,3,4
/// chunked in 200 per batch
///
/// panics when `ids.len() == 0`
fn join_ids<I: Display + 'static>(ids: &[I]) -> Result<Vec<String>, std::fmt::Error> {
    use std::fmt::Write;
    let modulo = ids.len() % 200 != 0;
    let ceil = ids.len() / 200 + (modulo as usize);
    let mut result = Vec::with_capacity(ceil);
    for ids in ids.chunks(200) {
        let mut query_string = String::with_capacity(6 * ids.len()); // arbitrary. most ids are 5 digits + comma
        write!(&mut query_string, "{}", ids[0])?;
        for i in ids.iter().skip(1) {
            write!(&mut query_string, ",{}", i)?;
        }
        result.push(query_string);
    }
    Ok(result)
}

fn get_expire_from_header(response: &Response) -> Duration {
    let exp = response
        .header("cache-control")
        .and_then(|c| c.split('=').rev().next())
        .and_then(|d| d.parse::<i64>().ok())
        .unwrap_or(300);
    Duration::seconds(exp)
}
