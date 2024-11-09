use std::{
    any::TypeId,
    fmt::Display,
    hash::Hash,
    ops::Deref,
    str::FromStr,
    sync::{Arc, Weak},
};

use async_trait::async_trait;
use chrono::{Duration, NaiveDateTime, Utc};
use dashmap::{mapref::entry::Entry, DashMap};
use either::Either;
use futures::{stream::FuturesUnordered, StreamExt};
use gw2lib_model::{
    BulkEndpoint, Endpoint, EndpointWithId, ErrorResponse, FixedEndpoint, Language, PagedEndpoint,
};
use hyper::{body::Buf, client::connect::Connect, Request, Response, Uri};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    Mutex,
};
#[cfg(feature = "tracing")]
use tracing::instrument;

use crate::{
    cache::in_memory::hash, ApiError, Cache, CachedRequest, Client, EndpointError, EndpointResult,
    Inflight, RateLimiter,
};

#[async_trait]
#[must_use]
pub trait Requester<const AUTHENTICATED: bool, const FORCE: bool>: Sized + Sync {
    type Caching: Cache + Send + Sync + 'static;
    type Connector: Connect + Clone + Send + Sync + 'static;
    type RateLimiting: RateLimiter + Send + Sync + 'static;

    #[doc(hidden)]
    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, Self::Connector, AUTHENTICATED>;

    #[doc(hidden)]
    fn cache_duration(&self) -> Duration;

    /// overwrites the cache duration for all requests returned from this
    /// function ## Example
    /// ```
    /// use chrono::Duration;
    /// use gw2lib::{Client, Requester};
    /// use gw2lib::model::items::Item;
    /// use gw2lib::model::misc::build::Build;
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
    ) -> CachedRequest<Self::Caching, Self::RateLimiting, Self::Connector, AUTHENTICATED, FORCE>
    {
        CachedRequest {
            client: self.client(),
            cache_duration,
        }
    }

    /// forces a fresh copy from the api
    /// ## Example
    /// ```
    /// use gw2lib::{Client, Requester};
    /// use gw2lib::model::misc::build::Build;
    ///
    /// let client = Client::default();
    /// let build_id: Build = client.get().unwrap();
    /// // this is cached and won't hit the api
    /// let build_id: Build = client.get().unwrap();
    /// // this will not check the cache and ask the api directly
    /// let build_id: Build = client.forced().get().unwrap();
    fn forced(
        &self,
    ) -> CachedRequest<Self::Caching, Self::RateLimiting, Self::Connector, AUTHENTICATED, true>
    {
        CachedRequest {
            client: self.client(),
            cache_duration: Duration::zero(),
        }
    }

    /// call the fixed endpoint
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(endpoint = %T::URL)))]
    async fn get<
        T: DeserializeOwned + Serialize + Clone + Send + Sync + FixedEndpoint + 'static,
    >(
        &self,
    ) -> EndpointResult<T> {
        get_or_ids::<T, T, Self, AUTHENTICATED, FORCE>(self).await
    }

    /// request a single item
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(id, endpoint = %T::URL)))]
    async fn single<
        T: DeserializeOwned + Serialize + Clone + Send + Sync + EndpointWithId<IdType = I> + 'static,
        I: Display + DeserializeOwned + Hash + Send + Sync + Clone + 'static,
    >(
        &self,
        id: impl Into<I> + Send,
    ) -> EndpointResult<T> {
        let id = id.into();
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("id", id.to_string());
        let lang = self.client().language;
        if let Some(c) = self.try_get(&id).await {
            return Ok(c);
        }

        let tx = loop {
            let either = check_inflight::<T, I, T, String>(
                &self.client().inflight,
                &id,
                lang,
                &self.client().identifier,
            )
            .await;
            match either {
                Some(Either::Left(mut rx)) => return rx.recv().await.map_err(Into::into),
                Some(Either::Right(tx)) => break tx,
                None => {
                    if let Some(c) = self.try_get(&id).await {
                        return Ok(c);
                    }
                }
            }
        };

        let request = build_request::<T, String, Self, AUTHENTICATED, FORCE>(
            self,
            T::format_url(T::format_id(&id).as_ref()),
            None,
        )?;

        let response = exec_req::<Self, AUTHENTICATED, FORCE>(self, request).await?;
        let result =
            cache_response::<I, T, T, Self, AUTHENTICATED, FORCE>(self, &id, response).await?;
        // ignoring the error is fine here
        // the receiving side will check the cache if nothing got sent
        let _ = tx.lock().await.send(result.clone());

        Ok(result)
    }

    /// retrieves an item from cache
    /// ```
    /// use gw2lib::{model::items::Item, Client, Requester};
    ///
    /// let client = Client::default();
    /// let from_cache: Option<Item> = client.try_get(&19721);
    /// ```
    #[cfg_attr(feature = "tracing", instrument(name = "get cached", skip_all, fields(id, endpoint = %T::URL)))]
    async fn try_get<
        T: DeserializeOwned + Serialize + Clone + Endpoint + Send + Sync + 'static,
        I: DeserializeOwned + Display + Hash + Clone + Sync + 'static,
    >(
        &self,
        id: impl Into<&I> + Send,
    ) -> Option<T> {
        let id = id.into();
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("id", id.to_string());
        check_cache::<T, I, T, Self, AUTHENTICATED, FORCE>(self, id).await
    }

    /// request all available ids
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(endpoint = %T::URL)))]
    async fn ids<
        T: DeserializeOwned + Serialize + EndpointWithId<IdType = I> + Clone + Send + Sync + 'static,
        I: Display + DeserializeOwned + Serialize + Hash + Clone + Send + Sync + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<I>> {
        get_or_ids::<T, Vec<I>, Self, AUTHENTICATED, FORCE>(self).await
    }

    /// request multiple ids at once
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(endpoint = %T::URL)))]
    async fn many<
        T: DeserializeOwned
            + Serialize
            + EndpointWithId<IdType = I>
            + BulkEndpoint
            + Clone
            + Send
            + Sync
            + 'static,
        I: Display + DeserializeOwned + Hash + Clone + Eq + Send + Sync + 'static,
    >(
        &self,
        ids: Vec<impl Into<I> + Send>,
    ) -> EndpointResult<Vec<T>> {
        let mut result = Vec::with_capacity(ids.len());
        let ids = if !FORCE {
            let ids = extract_many_from_cache(self, ids, &mut result).await;
            if ids.is_empty() {
                return Ok(result);
            }
            ids
        } else {
            ids.into_iter().map(|id| id.into()).collect()
        };

        let txs = DashMap::with_capacity(ids.len());
        let mut rxs = Vec::with_capacity(ids.len());
        let mut remaining_ids = Vec::with_capacity(ids.len());
        for id in ids {
            let retain = loop {
                let either = check_inflight::<T, I, T, String>(
                    &self.client().inflight,
                    &id,
                    self.client().language,
                    &self.client().identifier,
                )
                .await;
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
                        if let Some(c) =
                            check_cache::<T, I, T, Self, AUTHENTICATED, FORCE>(self, &id).await
                        {
                            result.push(c);
                            break false;
                        }
                    }
                }
            };
            if retain {
                remaining_ids.push(id);
            }
        }

        let result = Mutex::new(result);
        let chunks = join_ids(&remaining_ids);
        let futs: FuturesUnordered<_> = chunks
            .into_iter()
            .map(|rest| {
                let rest = Some(format!("ids={rest}"));
                async {
                    let request =
                        build_request::<T, _, Self, AUTHENTICATED, FORCE>(self, T::URL, rest)?;

                    let response = exec_req::<Self, AUTHENTICATED, FORCE>(self, request).await?;
                    let mut result = result.lock().await;
                    let index = result.len();
                    // TODO: consider postponing the locking
                    cache_response_many(self, response, &mut result).await?;

                    for x in result.iter().skip(index) {
                        let (_, tx) = txs
                            .remove(x.id())
                            .expect("received unexpected entry from api");
                        // ignoring the error is fine here
                        // the receiving side will check the cache if nothing got sent
                        let _ = tx.lock().await.send(x.clone());
                    }
                    Result::<(), EndpointError>::Ok(())
                }
            })
            .collect();
        let mut futs = futs.into_future();
        let mut error = None;
        while let (Some(res), fut) = futs.await {
            futs = fut.into_future();
            if let Err(e) = res {
                error = Some(e);
            }
        }
        if let Some(e) = error {
            return Err(e);
        }

        let mut result = result.into_inner();
        for mut rx in rxs {
            // TODO: check cache again
            result.push(rx.recv().await?);
        }

        Ok(result)
    }

    /// requests a page of items and returns the number of total items across
    /// all pages
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(endpoint = %T::URL)))]
    async fn page<T: DeserializeOwned + PagedEndpoint + Clone + Send + Sync + 'static>(
        &self,
        page: usize,
        page_size: u8,
        result: &mut Vec<T>,
    ) -> EndpointResult<usize> {
        let queries = format!("page={page}&page_size={page_size}");
        let request =
            build_request::<T, _, Self, AUTHENTICATED, FORCE>(self, T::URL, Some(queries))?;

        let response = exec_req::<Self, AUTHENTICATED, FORCE>(self, request).await?;
        let count = get_header(&response, "x-result-total").unwrap_or(0);
        let (_expires, res): (_, Vec<T>) = parse_response(self, response).await?;
        result.extend_from_slice(&res);

        Ok(count)
    }

    /// requests all items using the most efficient method available
    /// ### Remarks
    /// for most endpoints this means using [`Self::get_all_by_requesting_ids`].
    /// Compared to [`Self::get_all_by_paging`]
    /// this needs to perform an additional request to get all ids, but is much
    /// more cache friendly, being able to utilize the cache and inflight
    /// mechanisms.
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(endpoint = %T::URL)))]
    async fn all<
        T: DeserializeOwned
            + Serialize
            + EndpointWithId<IdType = I>
            + BulkEndpoint
            + Clone
            + Send
            + Sync
            + 'static,
        I: Display + DeserializeOwned + Serialize + Hash + Clone + Send + Sync + Eq + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        if T::ALL {
            self.get_all_by_ids_all().await
        // paging cannot utilize the cache, so we won't use it by default
        // } else if T::PAGING {
        //     self.get_all_by_paging()
        } else {
            self.get_all_by_requesting_ids().await
        }
    }

    /// Gets all items by querying ids=all
    ///
    /// use [`Self::all`] to use the most efficient way to request all items
    #[cfg_attr(feature = "tracing", instrument(name = "get all by ids all", skip_all, fields(endpoint = %T::URL)))]
    async fn get_all_by_ids_all<
        T: DeserializeOwned
            + Serialize
            + EndpointWithId<IdType = I>
            + BulkEndpoint
            + Clone
            + Send
            + Sync
            + 'static,
        I: Display + DeserializeOwned + Hash + Clone + Sync + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        if !T::ALL {
            return Err(EndpointError::UnsupportedEndpointQuery);
        }

        if let Some(c) =
            check_cache::<Vec<T>, str, T, Self, AUTHENTICATED, FORCE>(self, "ids=all").await
        {
            return Ok(c);
        }

        let request =
            build_request::<T, _, Self, AUTHENTICATED, FORCE>(self, T::URL, Some("ids=all"))?;

        let response = exec_req::<Self, AUTHENTICATED, FORCE>(self, request).await?;
        let count = get_header(&response, "x-result-total").unwrap_or(0);
        let mut result = Vec::with_capacity(count);
        cache_response_all(self, response, &mut result).await?;

        Ok(result)
    }

    /// Gets all items by querying all pages
    ///
    /// use [`Self::all`] to use the most efficient way to request all items
    #[cfg_attr(feature = "tracing", instrument(name = "get all by paging", skip_all, fields(endpoint = %T::URL)))]
    async fn get_all_by_paging<
        T: DeserializeOwned + PagedEndpoint + Clone + Send + Sync + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        let mut result = Vec::with_capacity(200);
        let max_items = self.page(0, 200, &mut result).await?;
        let remaining = max_items.saturating_sub(200);
        result.reserve_exact(remaining);

        let pages = ((remaining as f64) / 200_f64).ceil() as usize;
        for page in 0..pages {
            self.page(page + 1, 200, &mut result).await?;
        }

        Ok(result)
    }

    /// Gets all items by querying all ids
    ///
    /// use [`Self::all`] to use the most efficient way to request all items
    #[cfg_attr(feature = "tracing", instrument(name = "get all by requesting ids", skip_all, fields(endpoint = %T::URL)))]
    async fn get_all_by_requesting_ids<
        T: DeserializeOwned
            + Serialize
            + EndpointWithId<IdType = I>
            + BulkEndpoint
            + Clone
            + Send
            + Sync
            + 'static,
        I: Display + DeserializeOwned + Serialize + Hash + Clone + Send + Sync + Eq + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        let ids = self.ids::<T, I>().await?;
        self.many(ids).await
    }
}

struct SenderGuard<'client, T: Send> {
    sender: Arc<Mutex<Sender<T>>>,
    inflight: &'client Inflight,
    hash: (TypeId, u64),
}

impl<T: Send> Deref for SenderGuard<'_, T> {
    type Target = Mutex<Sender<T>>;

    fn deref(&self) -> &Self::Target {
        &self.sender
    }
}

impl<T: Send> Drop for SenderGuard<'_, T> {
    fn drop(&mut self) {
        let inflight = self.inflight.clone();
        let hash = self.hash;

        let task = async move { inflight.remove(&hash) };

        crate::block::spawn(task);
    }
}

#[cfg_attr(feature = "tracing", instrument(name = "check inflight", skip_all, fields(endpoint = %T::URL)))]
async fn check_inflight<
    'client,
    H: Send + Clone + 'static,
    I: 'static + Hash,
    T: Endpoint + Send + 'static,
    A: 'static + Hash,
>(
    inflight: &'client Inflight,
    id: &I,
    lang: Language,
    auth: &Option<A>,
) -> Option<Either<Receiver<H>, SenderGuard<'client, H>>> {
    let hash = hash::<_, H, I, A>(inflight.hasher(), id, T::LOCALE.then_some(lang), auth);
    Some(match inflight.entry(hash) {
        Entry::Occupied(mut e) => {
            let r = e
                .get_mut()
                .downcast_mut::<Weak<Mutex<Sender<H>>>>()
                .unwrap();
            let r = r.upgrade()?;
            let r = r.lock().await;
            Either::Left(r.subscribe())
        }
        Entry::Vacant(e) => {
            let tx = Arc::new(Mutex::new(broadcast::channel(1).0));
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

#[cfg_attr(feature = "tracing", instrument(name = "check cache", skip_all, fields(%id, endpoint = %E::URL)))]
async fn check_cache<
    T: DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
    I: Display + Hash + Sync + 'static + ?Sized,
    E: Endpoint,
    Req: Requester<A, F>,
    const A: bool,
    const F: bool,
>(
    req: &Req,
    id: &I,
) -> Option<T> {
    if !F {
        req.client()
            .cache
            .get::<T, I, E, String>(id, req.client().language, &req.client().identifier)
            .await
    } else {
        None
    }
}

async fn get_or_ids<
    T: DeserializeOwned + Serialize + Endpoint + Clone + Send + Sync + 'static,
    K: DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
    Req: Requester<A, F>,
    const A: bool,
    const F: bool,
>(
    req: &Req,
) -> EndpointResult<K> {
    let lang = req.client().language;
    if let Some(c) = check_cache::<K, str, T, Req, A, F>(req, "").await {
        return Ok(c);
    }

    let tx = loop {
        let either = check_inflight::<K, (), T, String>(
            &req.client().inflight,
            &(),
            lang,
            &req.client().identifier,
        )
        .await;
        match either {
            Some(Either::Left(mut rx)) => return rx.recv().await.map_err(Into::into),
            Some(Either::Right(tx)) => break tx,
            None => {
                if let Some(c) = check_cache::<K, str, T, Req, A, F>(req, "").await {
                    return Ok(c);
                }
            }
        }
    };

    let request = build_request::<T, String, Req, A, F>(req, T::URL, None)?;

    let response = exec_req::<Req, A, F>(req, request).await?;
    let result = cache_response::<str, K, T, Req, A, F>(req, "", response).await?;
    // ignoring the error is fine here
    // the receiving side will check the cache if nothing got sent
    let _ = tx.lock().await.send(result.clone());

    Ok(result)
}

#[cfg_attr(feature = "tracing", instrument(name = "execute request", skip_all, fields(uri = %request.uri().path())))]
async fn exec_req<Req: Requester<A, F>, const A: bool, const F: bool>(
    req: &Req,
    request: Request<hyper::Body>,
) -> EndpointResult<Response<hyper::Body>> {
    wait_for_rate_limit(req).await?;
    #[cfg(feature = "tracing")]
    {
        let uri = request.uri().path();
        let ids = request
            .uri()
            .query()
            .unwrap_or_default()
            .strip_prefix("ids=")
            .unwrap_or_default();
        tracing::info!(%uri, %ids, "gw2 request");
    }

    req.client()
        .client
        .request(request)
        .await
        .map_err(Into::into)
}

#[cfg_attr(
    feature = "tracing",
    instrument(name = "wait for rate limit", skip_all)
)]
async fn wait_for_rate_limit<Req: Requester<A, F>, const A: bool, const F: bool>(
    req: &Req,
) -> EndpointResult<()> {
    let time = req.client().rate_limiter.take(1).await?;
    tokio::time::sleep(time).await;
    Ok(())
}

fn build_request<T: Endpoint, Q: AsRef<str>, Req: Requester<A, F>, const A: bool, const F: bool>(
    req: &Req,
    path: impl AsRef<str>,
    extra_queries: Option<Q>,
) -> Result<Request<hyper::Body>, EndpointError> {
    if T::AUTHENTICATED && !A {
        return Err(EndpointError::NotAuthenticated);
    }

    let client = req.client();

    let mut pnq = String::with_capacity(400);
    pnq.push('/');
    pnq.push_str(path.as_ref());
    pnq.push('?');

    pnq.push_str("v=");
    pnq.push_str(T::VERSION);

    if let Some(extra) = extra_queries {
        pnq.push('&');
        pnq.push_str(extra.as_ref());
    }

    if T::LOCALE {
        pnq.push_str("&lang=");
        pnq.push_str(client.language.as_str());
    }

    if T::AUTHENTICATED {
        pnq.push_str("&access_token=");
        pnq.push_str(client.api_key.as_ref().unwrap());
    }

    let (scheme, host) = client.host.split_once("://").expect("invalid host");
    let uri = Uri::builder()
        .scheme(scheme)
        .authority(host)
        .path_and_query(pnq)
        .build()
        .expect("invalid uri");

    let request = hyper::Request::builder()
        .uri(uri)
        .body(hyper::Body::empty())
        .unwrap();

    Ok(request)
}

/// returns the remaining ids not found in cache
#[cfg_attr(feature = "tracing", instrument(name = "check cache many", skip_all, fields(endpoint = %K::URL)))]
async fn extract_many_from_cache<
    I: Display + Hash + Sync + 'static,
    K: EndpointWithId<IdType = I> + DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
    Req: Requester<A, F>,
    const A: bool,
    const F: bool,
>(
    req: &Req,
    ids: Vec<impl Into<I> + Send>,
    result: &mut Vec<K>,
) -> Vec<I> {
    let mut rest = Vec::with_capacity(ids.len());
    for i in ids {
        let i = i.into();
        if let Some(cached) = req
            .client()
            .cache
            .get::<K, I, K, String>(&i, req.client().language, &req.client().identifier)
            .await
        {
            result.push(cached);
        } else {
            rest.push(i);
        }
    }
    rest
}

async fn cache_response<
    I: Hash + Sync + 'static + Display + ?Sized,
    K: DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
    T: Endpoint,
    Req: Requester<A, F>,
    const A: bool,
    const F: bool,
>(
    req: &Req,
    id: &I,
    response: Response<hyper::Body>,
) -> Result<K, EndpointError> {
    let (expires, result): (_, K) = parse_response(req, response).await?;

    req.client()
        .cache
        .insert::<K, I, T, String>(
            id,
            &result,
            expires,
            req.client().language,
            &req.client().identifier,
        )
        .await;

    Ok(result)
}

async fn cache_response_many<
    I: Display + Hash + Sync + 'static,
    K: DeserializeOwned
        + Serialize
        + BulkEndpoint
        + EndpointWithId<IdType = I>
        + Clone
        + Send
        + Sync
        + 'static,
    Req: Requester<A, F>,
    const A: bool,
    const F: bool,
>(
    req: &Req,
    response: Response<hyper::Body>,
    result: &mut Vec<K>,
) -> Result<(), EndpointError> {
    let (expires, res): (_, Vec<K>) = parse_response(req, response).await?;

    for t in res {
        req.client()
            .cache
            .insert::<K, I, K, String>(
                t.id(),
                &t,
                expires,
                req.client().language,
                &req.client().identifier,
            )
            .await;
        result.push(t);
    }

    Ok(())
}

async fn cache_response_all<
    I: Display + Hash + Sync + 'static,
    K: DeserializeOwned
        + Serialize
        + BulkEndpoint
        + EndpointWithId<IdType = I>
        + Clone
        + Send
        + Sync
        + 'static,
    Req: Requester<A, F>,
    const A: bool,
    const F: bool,
>(
    req: &Req,
    response: Response<hyper::Body>,
    result: &mut Vec<K>,
) -> Result<(), EndpointError> {
    let (expires, res): (_, Vec<K>) = parse_response(req, response).await?;

    req.client()
        .cache
        .insert::<Vec<K>, str, K, String>(
            "ids=all",
            &res,
            expires,
            req.client().language,
            &req.client().identifier,
        )
        .await;

    for t in res {
        req.client()
            .cache
            .insert::<K, I, K, String>(
                t.id(),
                &t,
                expires,
                req.client().language,
                &req.client().identifier,
            )
            .await;
        result.push(t);
    }

    Ok(())
}

async fn parse_response<
    K: DeserializeOwned + Clone + Send + Sync + 'static,
    Req: Requester<A, F>,
    const A: bool,
    const F: bool,
>(
    req: &Req,
    response: Response<hyper::Body>,
) -> Result<(NaiveDateTime, K), EndpointError> {
    let status = response.status();
    if !status.is_success() {
        let bytes = hyper::body::to_bytes(response.into_body()).await?;
        let error = serde_json::from_slice::<'_, ErrorResponse>(&bytes);
        return Err(EndpointError::ApiError(match (status.as_u16(), error) {
            (401, _) => ApiError::Unauthorized,
            (400, Ok(ErrorResponse { text })) if &text == "invalid key" => ApiError::Unauthorized,
            (400, Ok(ErrorResponse { text })) if &text == "Invalid access token" => {
                ApiError::Unauthorized
            }
            (400, Ok(ErrorResponse { text })) if &text == "account does not have game access" => {
                ApiError::MissingGameAccess
            }
            (429, _) => {
                let _ = req.client().rate_limiter.penalize().await;
                ApiError::RateLimited
            }
            (_, Ok(ErrorResponse { text })) => ApiError::Other(status, text),
            _ => {
                let body = String::from_utf8_lossy(&bytes);
                ApiError::Other(status, body.to_string())
            }
        }));
    }
    let expires = get_cache_expiry(req, &response);
    let body = hyper::body::aggregate(response.into_body()).await?;
    let result: K = serde_json::from_reader(&mut body.reader())?;
    Ok((expires, result))
}

fn get_cache_expiry<Req: Requester<A, F>, const A: bool, const F: bool>(
    req: &Req,
    response: &Response<hyper::Body>,
) -> NaiveDateTime {
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
            write!(&mut query_string, ",{i}").expect("failed to concatenate ids");
        }
        result.push(query_string);
    }
    result
}

fn get_expire_from_header(response: &Response<hyper::Body>) -> Duration {
    let exp = get_header(response, "cache-control").unwrap_or(300);
    Duration::seconds(exp)
}

fn get_header<T: FromStr>(response: &Response<hyper::Body>, header: &str) -> Option<T> {
    response
        .headers()
        .iter()
        .find(|x| x.0 == header)
        .and_then(|(_, d)| d.to_str().ok())
        .and_then(|d| d.parse::<T>().ok())
}
