use std::{
    any::TypeId,
    collections::hash_map::Entry,
    fmt::Display,
    hash::Hash,
    marker::PhantomData,
    ops::Deref,
    sync::{Arc, Weak},
};

use bus::{Bus as Sender, BusReader as Receiver};
use chrono::{Duration, NaiveDateTime, Utc};
use either::Either;
use fxhash::FxHashMap;
use gw2api_model::{BulkEndpoint, Endpoint, EndpointWithId, FixedEndpoint, Language};
use parking_lot::Mutex;
use rayon::prelude::*;
use serde::de::DeserializeOwned;
use ureq::{Request, Response};

use crate::{
    cache::hash, Auth, Cache, CachedRequest, Client, EndpointError, EndpointResult, Force, Forced,
    Inflight, RateLimiter,
};

pub trait Requester: Sized + Sync {
    type Authenticated: Auth;
    type Caching: Cache + Send;
    type ForceRefresh: Force + Sync;
    type RateLimiting: RateLimiter + Sync;

    #[doc(hidden)]
    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, Self::Authenticated>;

    #[doc(hidden)]
    fn cache_duration(&self) -> Duration;

    /// overwrites the cache duration for all requests returned from this
    /// function ## Example
    /// ```
    /// use chrono::Duration;
    /// use gw2api_http::{Client, Requester};
    /// use gw2api_http::gw2api_model::items::Item;
    /// use gw2api_http::gw2api_model::misc::build::Build;
    ///
    /// let client = Client::default();
    /// let cache_client = client.cached(Duration::seconds(5));
    /// // these requests get cached for 5s
    /// let build_id: Build = cache_client.get().unwrap();
    /// let item: Item = cache_client.single(19993).unwrap();
    /// // normal caching
    /// let other_item: Item = client.single(19721).unwrap();
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

    /// forces a fresh copy from the api
    /// ## Example
    /// ```
    /// use gw2api_http::{Client, Requester};
    /// use gw2api_http::gw2api_model::misc::build::Build;
    ///
    /// let client = Client::default();
    /// let build_id: Build = client.get().unwrap();
    /// // this is cached and won't hit the api
    /// let build_id: Build = client.get().unwrap();
    /// // this will not check the cache and ask the api directly
    /// let build_id: Build = client.forced().get().unwrap();
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
    fn get<T: DeserializeOwned + Clone + Send + Sync + FixedEndpoint + 'static>(
        &self,
    ) -> EndpointResult<T> {
        get_or_ids::<T, T, Self>(self)
    }

    /// request a single item
    fn single<
        T: DeserializeOwned + Clone + Send + Sync + EndpointWithId<I> + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
        id: I,
    ) -> EndpointResult<T> {
        let lang = self.client().language;
        if let Some(c) = self.try_get(&id) {
            return Ok(c);
        }

        let tx = loop {
            let either = check_inflight::<T, I, T>(&self.client().inflight, &id, lang);
            match either {
                Some(Either::Left(mut rx)) => {
                    return rx.recv().map_err(EndpointError::InflightReceiveFailed)
                }
                Some(Either::Right(tx)) => break tx,
                None => {
                    if let Some(c) = self.try_get(&id) {
                        return Ok(c);
                    }
                }
            }
        };

        let url = T::format_url(&self.client().host, &id);
        let request = self.client().agent.get(&url);
        let request = set_common_headers::<T, Self>(self, request)?;

        let response = request.call().map_err(EndpointError::RequestFailed)?;
        let result = cache_response::<I, T, T, Self>(self, &id, response)?;
        tx.lock().broadcast(result.clone());

        Ok(result)
    }

    /// retrieves an item from cache
    /// ```
    /// use gw2api_http::{gw2api_model::items::Item, Client, Requester};
    ///
    /// let client = Client::default();
    /// let from_cache: Option<Item> = client.try_get(&19721);
    /// ```
    fn try_get<
        T: DeserializeOwned + Clone + Endpoint + Send + Sync + 'static,
        I: DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
        id: &I,
    ) -> Option<T> {
        check_cache::<T, I, T, Self>(self, id)
    }

    /// request all available ids
    fn ids<
        T: DeserializeOwned + EndpointWithId<I> + Clone + Send + Sync + 'static,
        I: Display + DeserializeOwned + Hash + Clone + Send + Sync + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<I>> {
        get_or_ids::<T, Vec<I>, Self>(self)
    }

    /// request multiple ids at once
    fn many<
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + Send + Sync + 'static,
        I: Display + DeserializeOwned + Hash + Clone + Eq + Send + Sync + 'static,
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

        let mut txs = FxHashMap::with_capacity_and_hasher(ids.len(), Default::default());
        let mut rxs = Vec::with_capacity(ids.len());
        ids = ids
            .into_iter()
            .filter(|id| loop {
                let either =
                    check_inflight::<T, I, T>(&self.client().inflight, id, self.client().language);
                match either {
                    Some(Either::Left(rx)) => {
                        rxs.push(rx);
                        break false;
                    }
                    Some(Either::Right(tx)) => {
                        txs.insert(id.clone(), tx);
                        break true;
                    }
                    None => {
                        if let Some(c) = check_cache::<T, I, T, Self>(self, id) {
                            result.push(c);
                            break false;
                        }
                    }
                }
            })
            .collect();

        let result = Mutex::new(result);
        let txs = Mutex::new(txs);
        let url = format!("{}/{}", self.client().host, T::URL);
        let chunks = join_ids(&ids);
        chunks
            .into_par_iter()
            .map(|rest| {
                let request = self.client().agent.get(&url).query("ids", &rest);
                let request = set_common_headers::<T, Self>(self, request)?;

                let response = request.call().map_err(EndpointError::RequestFailed)?;
                let mut result = result.lock();
                let index = result.len();
                cache_response_many(self, response, &mut result)?;

                let mut txs = txs.lock();
                for x in result.iter().skip(index) {
                    let tx = txs
                        .remove(x.id())
                        .expect("received unexpected entry from api");
                    tx.lock().broadcast(x.clone());
                }
                Ok(())
            })
            .collect::<EndpointResult<()>>()?;

        let mut result = result.into_inner();
        for mut rx in rxs {
            result.push(rx.recv().map_err(EndpointError::InflightReceiveFailed)?);
        }

        Ok(result)
    }

    /// requests a page of items and returns the number of total items across
    /// all pages
    fn page<
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + Send + 'static,
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
        let request = set_common_headers::<T, Self>(self, request)?;

        let response = request.call().map_err(EndpointError::RequestFailed)?;
        let count = response
            .header("x-result-total")
            .and_then(|x| x.parse().ok())
            .unwrap_or(0);
        cache_response_many(self, response, result)?;

        Ok(count)
    }

    /// requests all items using the most efficient method available
    /// ### Remarks
    /// for most endpoints this means using [`Self::get_all_by_requesting_ids`].
    /// Compared to [`Self::get_all_by_paging`]
    /// this needs to perform an additional request to get all ids, but is much
    /// more cache friendly, being able to utilize the cache and inflight
    /// mechanisms.
    fn all<
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + Send + Sync + 'static,
        I: Display + DeserializeOwned + Hash + Clone + Send + Sync + Eq + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        if T::ALL {
            self.get_all_by_ids_all()
        // paging cannot utilize the cache, so we won't use it by default
        // } else if T::PAGING {
        //     self.get_all_by_paging()
        } else {
            self.get_all_by_requesting_ids()
        }
    }

    /// Gets all items by querying ids=all
    ///
    /// use [`Self::all`] to use the most efficient way to request all items
    fn get_all_by_ids_all<
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + Send + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        if !T::ALL {
            return Err(EndpointError::UnsupportedEndpointQuery);
        }

        let url = format!("{}/{}", self.client().host, T::URL);
        let request = self.client().agent.get(&url).query("ids", "all");
        let request = set_common_headers::<T, Self>(self, request)?;

        let response = request.call().map_err(EndpointError::RequestFailed)?;
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
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + Send + 'static,
        I: Display + DeserializeOwned + Hash + Clone + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        if !T::PAGING {
            return Err(EndpointError::UnsupportedEndpointQuery);
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
        T: DeserializeOwned + EndpointWithId<I> + BulkEndpoint + Clone + Send + Sync + 'static,
        I: Display + DeserializeOwned + Hash + Clone + Send + Sync + Eq + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        let ids = self.ids::<T, I>()?;
        self.many(ids)
    }
}

struct SenderGuard<'client, T> {
    sender: Arc<Mutex<Sender<T>>>,
    inflight: &'client Inflight,
    hash: (TypeId, u64),
}

impl<T> Deref for SenderGuard<'_, T> {
    type Target = Mutex<Sender<T>>;

    fn deref(&self) -> &Self::Target {
        &self.sender
    }
}

impl<T> Drop for SenderGuard<'_, T> {
    fn drop(&mut self) {
        self.inflight.lock().remove(&self.hash);
    }
}

fn check_inflight<'client, H: Send + 'static, I: 'static + Hash, T: Endpoint + Send + 'static>(
    inflight: &'client Inflight,
    id: &I,
    lang: Language,
) -> Option<Either<Receiver<H>, SenderGuard<'client, H>>> {
    let hash = hash::<H, I>(id, T::LOCALE.then(|| lang));
    let mut locked = inflight.lock();
    Some(match locked.entry(hash) {
        Entry::Occupied(mut e) => {
            let r = e
                .get_mut()
                .downcast_mut::<Weak<Mutex<Sender<H>>>>()
                .unwrap();
            let r = r.upgrade()?;
            let mut r = r.lock();
            Either::Left(r.add_rx())
        }
        Entry::Vacant(e) => {
            let tx = Arc::new(Mutex::new(Sender::new(1)));
            e.insert(Box::new(Arc::downgrade(&tx)));
            let tx = SenderGuard {
                sender: tx,
                inflight,
                hash,
            };
            Either::Right(tx)
        }
    })
}

fn check_cache<T: Clone + Send + 'static, I: Hash + 'static, E: Endpoint, Req: Requester>(
    req: &Req,
    id: &I,
) -> Option<T> {
    if !Req::ForceRefresh::FORCED {
        let mut cache = req.client().cache.lock();
        cache.get::<T, I, E>(id, req.client().language)
    } else {
        None
    }
}

fn get_or_ids<
    T: DeserializeOwned + Endpoint + Clone + Send + Sync + 'static,
    K: DeserializeOwned + Clone + Send + Sync + 'static,
    Req: Requester,
>(
    req: &Req,
) -> EndpointResult<K> {
    let lang = req.client().language;
    if let Some(c) = check_cache::<K, (), T, Req>(req, &()) {
        return Ok(c);
    }

    let tx = loop {
        let either = check_inflight::<K, (), T>(&req.client().inflight, &(), lang);
        match either {
            Some(Either::Left(mut rx)) => {
                return rx.recv().map_err(EndpointError::InflightReceiveFailed)
            }
            Some(Either::Right(tx)) => break tx,
            None => {
                if let Some(c) = check_cache::<K, (), T, Req>(req, &()) {
                    return Ok(c);
                }
            }
        }
    };

    let url = format!("{}/{}", req.client().host, T::URL);
    let request = req.client().agent.get(&url);
    let request = set_common_headers::<T, Req>(req, request)?;

    let response = request.call().map_err(EndpointError::RequestFailed)?;
    let result = cache_response::<(), K, T, Req>(req, &(), response)?;
    tx.lock().broadcast(result.clone());

    Ok(result)
}

fn set_common_headers<T: Endpoint, R: Requester>(
    req: &R,
    mut request: Request,
) -> Result<Request, EndpointError> {
    if T::AUTHENTICATED && !R::Authenticated::AUTHENTICATED {
        return Err(EndpointError::NotAuthenticated);
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
fn extract_many_from_cache<
    I: Display + Hash + 'static,
    K: EndpointWithId<I> + Clone + Send + 'static,
    Req: Requester,
>(
    req: &Req,
    ids: Vec<I>,
    result: &mut Vec<K>,
) -> Vec<I> {
    let mut cache = req.client().cache.lock();
    ids.into_iter()
        .filter(|i| {
            if let Some(cached) = cache.get::<K, I, K>(i, req.client().language) {
                result.push(cached);
                false
            } else {
                true
            }
        })
        .collect()
}

fn cache_response<
    I: Hash + 'static,
    K: DeserializeOwned + Clone + Send + 'static,
    T: Endpoint,
    Req: Requester,
>(
    req: &Req,
    id: &I,
    response: Response,
) -> Result<K, EndpointError> {
    let expires = get_cache_expiry(req, &response);
    let result: K = response
        .into_json()
        .map_err(EndpointError::InvalidJsonResponse)?;
    let res = result.clone();
    {
        let mut cache = req.client().cache.lock();
        cache.insert::<K, I, T>(id, res, expires, req.client().language);
    }
    Ok(result)
}

fn cache_response_many<
    I: Display + Hash + 'static,
    K: DeserializeOwned + EndpointWithId<I> + Clone + Send + 'static,
    Req: Requester,
>(
    req: &Req,
    response: Response,
    result: &mut Vec<K>,
) -> Result<(), EndpointError> {
    let expires = get_cache_expiry(req, &response);
    let res: Vec<K> = response
        .into_json()
        .map_err(EndpointError::InvalidJsonResponse)?;
    {
        let mut cache = req.client().cache.lock();
        for t in res {
            cache.insert::<K, I, K>(t.id(), t.clone(), expires, req.client().language);
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
fn join_ids<I: Display + 'static>(ids: &[I]) -> Vec<String> {
    use std::fmt::Write;
    let modulo = ids.len() % 200 != 0;
    let ceil = ids.len() / 200 + (modulo as usize);
    let mut result = Vec::with_capacity(ceil);
    for ids in ids.chunks(200) {
        let mut query_string = String::with_capacity(6 * ids.len()); // arbitrary. most ids are 5 digits + comma
        write!(&mut query_string, "{}", ids[0]).expect("failed to concatenate ids");
        for i in ids.iter().skip(1) {
            write!(&mut query_string, ",{}", i).expect("failed to concatenate ids");
        }
        result.push(query_string);
    }
    result
}

fn get_expire_from_header(response: &Response) -> Duration {
    let exp = response
        .header("cache-control")
        .and_then(|c| c.split('=').rev().next())
        .and_then(|d| d.parse::<i64>().ok())
        .unwrap_or(300);
    Duration::seconds(exp)
}
