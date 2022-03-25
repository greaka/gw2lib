use std::{fmt::Display, hash::Hash, marker::PhantomData, sync::MutexGuard};

use chrono::{Duration, NaiveDateTime, Utc};
use gw2api_model::{BulkEndpoint, Endpoint, EndpointWithId, FixedEndpoint};
use serde::de::DeserializeOwned;
use ureq::{Request, Response};

use crate::{
    Auth, Cache, CachedRequest, Client, EndpointResult, ErrorNotAuthenticated,
    ErrorUnsupportedEndpointQuery, Force, Forced, RateLimiter,
};

pub trait Requester: Sized {
    type Authenticated: Auth;
    type Caching: Cache;
    type RateLimiting: RateLimiter;
    type ForceRefresh: Force;

    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, Self::Authenticated>;

    fn cache_duration(&self) -> Duration;

    fn cached(
        &self,
        cache_duration: Duration,
    ) -> CachedRequest<Self::Caching, Self::RateLimiting, Self::Authenticated, Self::ForceRefresh>
    {
        CachedRequest {
            client: self.client(),
            cache_duration,
            forced: PhantomData,
        }
    }

    fn forced(
        &self,
    ) -> CachedRequest<Self::Caching, Self::RateLimiting, Self::Authenticated, Forced> {
        CachedRequest {
            client: self.client(),
            cache_duration: Duration::zero(),
            forced: PhantomData,
        }
    }

    /// call the fixed endpoint
    fn get<T: DeserializeOwned + Clone + FixedEndpoint + 'static>(&self) -> EndpointResult<T> {
        get_or_ids::<T, T, Self>(self)
    }

    /// request a single item
    fn single<
        T: DeserializeOwned + Clone + EndpointWithId<I> + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
        id: I,
    ) -> EndpointResult<T> {
        if !Self::ForceRefresh::FORCED {
            let mut cache: MutexGuard<Self::Caching> = self.client().cache.lock().unwrap();
            if let Some(cached) = cache.get(&id) {
                return Ok(cached);
            }
        }

        let url = format!("{}/{}/{}", self.client().host, T::URL, id);
        let request = self.client().agent.get(&url);
        let request = set_common_headers_and_rate_limit::<T, Self>(self, request)?;

        let response = request.call()?;
        let result = cache_response(self, &id, response)?;

        Ok(result)
    }

    /// request all available ids
    fn ids<
        T: DeserializeOwned + EndpointWithId<I> + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<I>> {
        get_or_ids::<T, Vec<I>, Self>(self)
    }

    /// request multiple ids at once
    fn many<
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
        mut ids: Vec<I>,
    ) -> EndpointResult<Vec<T>> {
        let mut result = Vec::with_capacity(ids.len());
        if !Self::ForceRefresh::FORCED {
            ids = extract_many_from_cache(self, ids, &mut result);
            if ids.is_empty() {
                return Ok(result);
            }
        }

        let url = format!("{}/{}", self.client().host, T::URL);
        let chunks = join_ids(&ids)?;
        for rest in chunks {
            let request = self.client().agent.get(&url).query("ids", &rest);
            let request = set_common_headers_and_rate_limit::<T, Self>(self, request)?;

            let response = request.call()?;
            cache_response_many(self, response, &mut result)?;
        }

        Ok(result)
    }

    /// requests a page of items and returns the number of total items across
    /// all pages
    fn page<
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
        page: usize,
        page_size: u8,
        result: &mut Vec<T>,
    ) -> EndpointResult<usize> {
        let url = format!("{}/{}", self.client().host, T::URL);
        let request = self
            .client()
            .agent
            .get(&url)
            .query("page", page.to_string().as_str())
            .query("page_size", page_size.to_string().as_str());
        let request = set_common_headers_and_rate_limit::<T, Self>(self, request)?;

        let response = request.call()?;
        let count = response
            .header("x-result-total")
            .and_then(|x| x.parse().ok())
            .unwrap_or(0);
        cache_response_many(self, response, result)?;

        Ok(count)
    }

    /// requests all items using the most efficient method available
    /// ### Remarks
    /// for most endpoints this means using [`Self::get_all_by_paging`].
    /// You might not want this for endpoints that change rapidly (like
    /// requesting listings on the tp). In that case, fall back to
    /// [`Self::get_all_by_requesting_ids`] or simply call [`Self::many`].
    fn all<
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
    fn get_all_by_ids_all<
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        if !T::ALL {
            return Err(Box::new(ErrorUnsupportedEndpointQuery));
        }

        let url = format!("{}/{}", self.client().host, T::URL);
        let request = self.client().agent.get(&url).query("ids", "all");
        let request = set_common_headers_and_rate_limit::<T, Self>(self, request)?;

        let response = request.call()?;
        let count = response
            .header("x-result-total")
            .and_then(|x| x.parse().ok())
            .unwrap_or(0);
        let mut result = Vec::with_capacity(count);
        cache_response_many(self, response, &mut result)?;

        Ok(result)
    }

    /// Gets all items by querying all pages
    ///
    /// use [`Self::all`] to use the most efficient way to request all items
    fn get_all_by_paging<
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
    fn get_all_by_requesting_ids<
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        let ids = self.ids::<T, I>()?;
        self.many(ids)
    }
}

fn get_or_ids<
    T: DeserializeOwned + Endpoint + 'static,
    K: DeserializeOwned + Clone + 'static,
    Req: Requester,
>(
    req: &Req,
) -> EndpointResult<K> {
    if !Req::ForceRefresh::FORCED {
        let mut cache = req.client().cache.lock().unwrap();
        if let Some(cached) = cache.get(&()) {
            return Ok(cached);
        }
    }

    let url = format!("{}/{}", req.client().host, T::URL);
    let request = req.client().agent.get(&url);
    let request = set_common_headers_and_rate_limit::<T, Req>(req, request)?;

    let response = request.call()?;
    let result = cache_response(req, &(), response)?;

    Ok(result)
}

fn set_common_headers_and_rate_limit<T: Endpoint, R: Requester>(
    req: &R,
    mut request: Request,
) -> EndpointResult<Request> {
    if T::AUTHENTICATED && !R::Authenticated::AUTHENTICATED {
        return Err(Box::new(ErrorNotAuthenticated));
    }
    request = request.set("X-Schema-Version", T::VERSION);
    if T::AUTHENTICATED {
        request = request.set(
            "Authorization",
            &format!("Bearer {}", req.client().api_key.as_ref().unwrap()),
        );
    }
    if T::LOCALE {
        request = request.query("lang", req.client().language.as_str());
    }

    Ok(request)
}

/// returns the remaining ids not found in cache
fn extract_many_from_cache<I: Hash + 'static, K: Clone + 'static, Req: Requester>(
    req: &Req,
    ids: Vec<I>,
    result: &mut Vec<K>,
) -> Vec<I> {
    let mut cache = req.client().cache.lock().unwrap();
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

fn cache_response<I: Hash + 'static, K: DeserializeOwned + Clone + 'static, Req: Requester>(
    req: &Req,
    id: &I,
    response: Response,
) -> Result<K, std::io::Error> {
    let expires = get_cache_expiry(req, &response);
    let result: K = response.into_json()?;
    let res = result.clone();
    {
        let mut cache = req.client().cache.lock().unwrap();
        cache.insert(id, res, expires);
    }
    Ok(result)
}

fn cache_response_many<
    I: Hash + 'static,
    K: DeserializeOwned + EndpointWithId<I> + Clone + 'static,
    Req: Requester,
>(
    req: &Req,
    response: Response,
    result: &mut Vec<K>,
) -> Result<(), std::io::Error> {
    let expires = get_cache_expiry(req, &response);
    let res: Vec<K> = response.into_json()?;
    {
        let mut cache = req.client().cache.lock().unwrap();
        for t in res {
            cache.insert(t.id(), t.clone(), expires);
            result.push(t);
        }
    }
    Ok(())
}

fn get_cache_expiry<Req: Requester>(req: &Req, response: &Response) -> NaiveDateTime {
    let duration = req.cache_duration();
    let expires = if !duration.is_zero() {
        duration
    } else {
        get_expire_from_header(response)
    };
    Utc::now().naive_utc() + expires
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
