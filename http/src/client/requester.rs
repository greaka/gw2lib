#![allow(private_bounds, mismatched_lifetime_syntaxes, async_fn_in_trait)]

use std::{
    any::TypeId,
    fmt::{Display, Write},
    hash::Hash,
    marker::PhantomData,
    ops::Deref,
    str::FromStr,
    sync::{Arc, Weak},
};

use chrono::{Duration, NaiveDateTime, Utc};
use dashmap::{DashMap, mapref::entry::Entry};
use either::Either;
use futures::{StreamExt, stream::FuturesUnordered};
use gw2lib_model::{
    Authentication, BulkEndpoint, Endpoint, EndpointWithId, ErrorResponse, FixedEndpoint, Language,
    PagedEndpoint,
};
use reqwest::{Method, Request, Response, Url};
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::{
    Mutex,
    broadcast::{self, Receiver, Sender},
};
#[cfg(feature = "tracing")]
use tracing::{Instrument, instrument};

use crate::{
    ApiError, Cache, CachedRequest, Client, EndpointError, EndpointResult, Forced, Inflight,
    RateLimiter,
    cache::in_memory::hash,
    client::{AllowsClient, Force},
    rate_limit::ApiPermit,
};

#[must_use]
pub trait Requester: Sized + Sync {
    type Authenticated: Authentication;
    type Caching: Cache + Send + Sync + 'static;
    type Force: Force;
    type RateLimiting: RateLimiter + Send + Sync + 'static;

    #[doc(hidden)]
    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, Self::Authenticated>;

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
    ) -> CachedRequest<Self::Caching, Self::RateLimiting, Self::Authenticated, Self::Force> {
        CachedRequest {
            client: self.client(),
            cache_duration,
            _phantom: PhantomData,
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
    ) -> CachedRequest<Self::Caching, Self::RateLimiting, Self::Authenticated, Forced> {
        CachedRequest {
            client: self.client(),
            cache_duration: Duration::zero(),
            _phantom: PhantomData,
        }
    }

    /// call the fixed endpoint
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(endpoint = %T::URL)))]
    async fn get<
        T: DeserializeOwned
            + Serialize
            + Clone
            + Send
            + Sync
            + FixedEndpoint
            + Endpoint<Authenticated: AllowsClient<Self::Authenticated>>
            + 'static,
    >(
        &self,
    ) -> EndpointResult<T> {
        get_or_ids::<T, T, Self>(self).await
    }

    /// retrieves an item from cache
    /// ```
    /// use gw2lib::{Client, Requester, model::items::Item};
    ///
    /// let client = Client::default();
    /// let from_cache: Option<Item> = client.try_get();
    /// ```
    #[cfg_attr(feature = "tracing", instrument(name = "get cached", skip_all, fields(endpoint = %T::URL)))]
    async fn try_get<T>(&self) -> Option<T>
    where
        T: DeserializeOwned + Serialize + Clone + FixedEndpoint + Send + Sync + 'static,
    {
        check_cache::<T, str, T, Self>(self, "").await
    }

    /// request a single item
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(id, endpoint = %T::URL)))]
    async fn single<T>(
        &self,
        id: impl Into<<T as EndpointWithId>::IdType> + Send,
    ) -> EndpointResult<T>
    where
        T: DeserializeOwned + Serialize + Clone + Send + Sync + EndpointWithId + 'static,
        <T as EndpointWithId>::IdType:
            Display + DeserializeOwned + Hash + Send + Sync + Clone + 'static,
        <T as Endpoint>::Authenticated: AllowsClient<Self::Authenticated>,
    {
        let id = id.into();
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("id", id.to_string());
        let lang = self.client().language;
        if let Some(c) = self.try_single(&id).await {
            return Ok(c);
        }

        let tx = loop {
            let either = check_inflight::<T, <T as EndpointWithId>::IdType, T, _>(
                &self.client().inflight,
                &id,
                lang,
                &self.client().identifier,
            )
            .await;
            match either {
                // todo: check cache again
                Some(Either::Left(mut rx)) => return rx.recv().await.map_err(Into::into),
                Some(Either::Right(tx)) => break tx,
                None => {
                    if let Some(c) = self.try_single(&id).await {
                        return Ok(c);
                    }
                }
            }
        };

        let request = build_request::<T, String, Self>(
            self,
            T::format_url(T::format_id(&id).as_ref()),
            None,
        )?;

        let response = exec_req::<Self>(self, request).await?;
        let result =
            cache_response::<<T as EndpointWithId>::IdType, T, T, Self>(self, &id, response)
                .await?;
        // ignoring the error is fine here
        // the receiving side will check the cache if nothing got sent
        let _ = tx.lock().await.send(result.clone());

        Ok(result)
    }

    /// retrieves an item from cache
    /// ```
    /// use gw2lib::{Client, Requester, model::items::Item};
    ///
    /// let client = Client::default();
    /// let from_cache: Option<Item> = client.try_single(&19721);
    /// ```
    #[cfg_attr(feature = "tracing", instrument(name = "single cached", skip_all, fields(%id, endpoint = %T::URL)))]
    async fn try_single<T>(&self, id: &<T as EndpointWithId>::IdType) -> Option<T>
    where
        T: DeserializeOwned + Serialize + Clone + EndpointWithId + Send + Sync + 'static,
        <T as EndpointWithId>::IdType:
            DeserializeOwned + Display + Hash + Clone + Send + Sync + 'static,
        <T as Endpoint>::Authenticated: AllowsClient<Self::Authenticated>,
    {
        check_cache::<T, <T as EndpointWithId>::IdType, T, Self>(self, id).await
    }

    /// request all available ids
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(endpoint = %T::URL)))]
    async fn ids<T>(&self) -> EndpointResult<Vec<<T as EndpointWithId>::IdType>>
    where
        T: DeserializeOwned + Serialize + EndpointWithId + Clone + Send + Sync + 'static,
        <T as EndpointWithId>::IdType:
            Display + DeserializeOwned + Serialize + Hash + Clone + Send + Sync + 'static,
        <T as Endpoint>::Authenticated: AllowsClient<Self::Authenticated>,
    {
        get_or_ids::<T, Vec<<T as EndpointWithId>::IdType>, Self>(self).await
    }

    /// retrieves an item from cache
    /// ```
    /// use gw2lib::{
    ///     Client, Requester,
    ///     model::items::{Item, ItemId},
    /// };
    ///
    /// let client = Client::default();
    /// let from_cache: Option<Vec<ItemId>> = client.try_ids::<Item>();
    /// ```
    #[cfg_attr(feature = "tracing", instrument(name = "get cached ids", skip_all, fields(endpoint = %T::URL)))]
    async fn try_ids<T>(&self) -> Option<Vec<<T as EndpointWithId>::IdType>>
    where
        T: DeserializeOwned + Serialize + Clone + EndpointWithId + Send + Sync + 'static,
        <T as EndpointWithId>::IdType:
            Display + DeserializeOwned + Serialize + Hash + Clone + Send + Sync + 'static,
        <T as Endpoint>::Authenticated: AllowsClient<Self::Authenticated>,
    {
        check_cache::<Vec<<T as EndpointWithId>::IdType>, str, T, Self>(self, "").await
    }

    /// request multiple ids at once
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(endpoint = %T::URL)))]
    async fn many<T>(
        &self,
        ids: Vec<impl Into<<T as EndpointWithId>::IdType> + Send>,
    ) -> EndpointResult<Vec<T>>
    where
        T: DeserializeOwned
            + Serialize
            + EndpointWithId
            + BulkEndpoint
            + Clone
            + Send
            + Sync
            + 'static,
        <T as EndpointWithId>::IdType:
            Display + DeserializeOwned + Hash + Clone + Eq + Send + Sync + 'static,
        <T as Endpoint>::Authenticated: AllowsClient<Self::Authenticated>,
    {
        let mut result = Vec::with_capacity(ids.len());
        let ids = if !Self::Force::FORCE {
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
                let either = check_inflight::<T, <T as EndpointWithId>::IdType, T, _>(
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
                            check_cache::<T, <T as EndpointWithId>::IdType, T, Self>(self, &id)
                                .await
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
                    let request = build_request::<T, _, Self>(self, T::URL, rest)?;

                    let response = exec_req::<Self>(self, request).await?;
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
    async fn page<
        T: DeserializeOwned
            + PagedEndpoint
            + Endpoint<Authenticated: AllowsClient<Self::Authenticated>>
            + Clone
            + Send
            + Sync
            + 'static,
    >(
        &self,
        page: usize,
        page_size: u8,
        result: &mut Vec<T>,
    ) -> EndpointResult<usize> {
        let queries = format!("page={page}&page_size={page_size}");
        let request = build_request::<T, _, Self>(self, T::URL, Some(queries))?;

        let response = exec_req::<Self>(self, request).await?;
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
    async fn all<T>(&self) -> EndpointResult<Vec<T>>
    where
        T: DeserializeOwned
            + Serialize
            + EndpointWithId
            + BulkEndpoint
            + Clone
            + Send
            + Sync
            + 'static,
        <T as EndpointWithId>::IdType:
            Display + DeserializeOwned + Serialize + Hash + Clone + Send + Sync + Eq + 'static,
        <T as Endpoint>::Authenticated: AllowsClient<Self::Authenticated>,
    {
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
    async fn get_all_by_ids_all<T>(&self) -> EndpointResult<Vec<T>>
    where
        T: DeserializeOwned
            + Serialize
            + EndpointWithId
            + BulkEndpoint
            + Clone
            + Send
            + Sync
            + 'static,
        <T as EndpointWithId>::IdType: Display + DeserializeOwned + Hash + Clone + Sync + 'static,
        <T as Endpoint>::Authenticated: AllowsClient<Self::Authenticated>,
    {
        if !T::ALL {
            return Err(EndpointError::UnsupportedEndpointQuery);
        }

        if let Some(c) = check_cache::<Vec<T>, str, T, Self>(self, "ids=all").await {
            return Ok(c);
        }

        let request = build_request::<T, _, Self>(self, T::URL, Some("ids=all"))?;

        let response = exec_req::<Self>(self, request).await?;
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
        T: DeserializeOwned
            + PagedEndpoint
            + Endpoint<Authenticated: AllowsClient<Self::Authenticated>>
            + Clone
            + Send
            + Sync
            + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        let mut result = Vec::with_capacity(200);
        let max_items = self.page(0, 200, &mut result).await?;
        let remaining = max_items.saturating_sub(200);
        result.reserve_exact(remaining);

        let pages = ((remaining as f64) / 200_f64).ceil() as usize;
        for page in 0..pages {
            // todo: run in parallel
            // todo: cache
            self.page(page + 1, 200, &mut result).await?;
        }

        Ok(result)
    }

    /// Gets all items by querying all ids
    ///
    /// use [`Self::all`] to use the most efficient way to request all items
    #[cfg_attr(feature = "tracing", instrument(name = "get all by requesting ids", skip_all, fields(endpoint = %T::URL)))]
    async fn get_all_by_requesting_ids<T>(&self) -> EndpointResult<Vec<T>>
    where
        T: DeserializeOwned
            + Serialize
            + EndpointWithId
            + BulkEndpoint
            + Clone
            + Send
            + Sync
            + 'static,
        <T as EndpointWithId>::IdType:
            Display + DeserializeOwned + Serialize + Hash + Clone + Send + Sync + Eq + 'static,
        <T as Endpoint>::Authenticated: AllowsClient<Self::Authenticated>,
    {
        let ids = self.ids::<T>().await?;
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
    Req: Requester,
>(
    req: &Req,
    id: &I,
) -> Option<T> {
    if !Req::Force::FORCE {
        req.client()
            .cache
            .get::<T, I, E, _>(id, req.client().language, &req.client().identifier)
            .await
    } else {
        None
    }
}

async fn get_or_ids<
    T: DeserializeOwned
        + Serialize
        + Endpoint<Authenticated: AllowsClient<Req::Authenticated>>
        + Clone
        + Send
        + Sync
        + 'static,
    K: DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
    Req: Requester,
>(
    req: &Req,
) -> EndpointResult<K> {
    let lang = req.client().language;
    if let Some(c) = check_cache::<K, str, T, Req>(req, "").await {
        return Ok(c);
    }

    let tx = loop {
        let either = check_inflight::<K, (), T, _>(
            &req.client().inflight,
            &(),
            lang,
            &req.client().identifier,
        )
        .await;
        match either {
            // todo: check cache again
            Some(Either::Left(mut rx)) => return rx.recv().await.map_err(Into::into),
            Some(Either::Right(tx)) => break tx,
            None => {
                if let Some(c) = check_cache::<K, str, T, Req>(req, "").await {
                    return Ok(c);
                }
            }
        }
    };

    let request = build_request::<T, String, Req>(req, T::URL, None)?;

    let response = exec_req::<Req>(req, request).await?;
    let result = cache_response::<str, K, T, Req>(req, "", response).await?;
    // ignoring the error is fine here
    // the receiving side will check the cache if nothing got sent
    let _ = tx.lock().await.send(result.clone());

    Ok(result)
}

#[cfg_attr(feature = "tracing", instrument(name = "execute request", skip_all, fields(uri = %request.uri().path())))]
async fn exec_req<Req: Requester>(req: &Req, request: Request) -> EndpointResult<Response> {
    let _permit = wait_for_rate_limit(req).await?;

    #[cfg(feature = "tracing")]
    let span = {
        let uri = request.uri().path();
        let ids = request
            .uri()
            .query()
            .unwrap_or_default()
            .split("ids=")
            .nth(1)
            .unwrap_or_default()
            .split("&")
            .next()
            .unwrap_or_default();
        let span = tracing::info_span!("gw2 request", %uri, %ids);
        let entered = span.enter();
        tracing::info!(%uri, %ids, "gw2 request");
        drop(entered);
        span
    };

    let fut = req.client().client.execute(request);

    #[cfg(feature = "tracing")]
    let fut = fut.instrument(span);

    fut.await.map_err(Into::into)
}

#[cfg_attr(
    feature = "tracing",
    instrument(name = "wait for rate limit", skip_all)
)]
async fn wait_for_rate_limit<Req: Requester>(
    req: &Req,
) -> EndpointResult<ApiPermit<Req::RateLimiting>> {
    let permit = req.client().rate_limiter.take(1).await?;

    permit
        .await
        .map_err(|_| EndpointError::RateLimiterCrashed("failed to receive request permit".into()))
}

fn build_request<
    T: Endpoint<Authenticated: AllowsClient<Req::Authenticated> + Authentication>,
    Q: AsRef<str>,
    Req: Requester,
>(
    req: &Req,
    path: impl AsRef<str>,
    extra_queries: Option<Q>,
) -> Result<Request, EndpointError> {
    let client = req.client();

    let mut query = String::with_capacity(400);
    write!(query, "v={}", T::VERSION).unwrap();

    if let Some(extra) = extra_queries {
        write!(query, "&{}", extra.as_ref()).unwrap();
    }

    if T::LOCALE {
        write!(query, "&lang={}", client.language.as_str()).unwrap();
    }

    if T::Authenticated::AUTHENTICATED {
        write!(query, "&access_token={}", client.api_key.as_ref().unwrap()).unwrap();
    }

    let mut url = Url::parse(&client.host).unwrap();
    url.set_path(path.as_ref());
    url.set_query(Some(&query));

    Ok(Request::new(Method::GET, url))
}

/// returns the remaining ids not found in cache
#[cfg_attr(feature = "tracing", instrument(name = "check cache many", skip_all, fields(endpoint = %K::URL)))]
async fn extract_many_from_cache<
    K: EndpointWithId<IdType: Display + Hash + Sync + 'static>
        + DeserializeOwned
        + Serialize
        + Clone
        + Send
        + Sync
        + 'static,
    Req: Requester,
>(
    req: &Req,
    ids: Vec<impl Into<K::IdType> + Send>,
    result: &mut Vec<K>,
) -> Vec<K::IdType> {
    let mut rest = Vec::with_capacity(ids.len());
    for i in ids {
        let i = i.into();
        if let Some(cached) = req
            .client()
            .cache
            .get::<K, K::IdType, K, _>(&i, req.client().language, &req.client().identifier)
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
    Req: Requester,
>(
    req: &Req,
    id: &I,
    response: Response,
) -> Result<K, EndpointError> {
    let (expires, result): (_, K) = parse_response(req, response).await?;

    req.client()
        .cache
        .insert::<K, I, T, _>(
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
    K: DeserializeOwned
        + Serialize
        + BulkEndpoint
        + EndpointWithId<IdType: Display + Hash + Sync + 'static>
        + Clone
        + Send
        + Sync
        + 'static,
    Req: Requester,
>(
    req: &Req,
    response: Response,
    result: &mut Vec<K>,
) -> Result<(), EndpointError> {
    let (expires, res): (_, Vec<K>) = parse_response(req, response).await?;

    cache_entries(req, result, expires, res).await;

    Ok(())
}

async fn cache_response_all<
    K: DeserializeOwned
        + Serialize
        + BulkEndpoint
        + EndpointWithId<IdType: Display + Hash + Sync + 'static>
        + Clone
        + Send
        + Sync
        + 'static,
    Req: Requester,
>(
    req: &Req,
    response: Response,
    result: &mut Vec<K>,
) -> Result<(), EndpointError> {
    let (expires, res): (_, Vec<K>) = parse_response(req, response).await?;

    req.client()
        .cache
        .insert::<Vec<K>, str, K, _>(
            "ids=all",
            &res,
            expires,
            req.client().language,
            &req.client().identifier,
        )
        .await;

    cache_entries(req, result, expires, res).await;

    Ok(())
}

async fn cache_entries<
    K: DeserializeOwned
        + Serialize
        + BulkEndpoint
        + EndpointWithId<IdType: Display + Hash + Sync + 'static>
        + Clone
        + Send
        + Sync
        + 'static,
    Req: Requester,
>(
    req: &Req,
    result: &mut Vec<K>,
    expires: NaiveDateTime,
    res: Vec<K>,
) {
    for t in res {
        req.client()
            .cache
            .insert::<K, K::IdType, K, _>(
                t.id(),
                &t,
                expires,
                req.client().language,
                &req.client().identifier,
            )
            .await;
        result.push(t);
    }
}

async fn parse_response<K: DeserializeOwned + Clone + Send + Sync + 'static, Req: Requester>(
    req: &Req,
    response: Response,
) -> Result<(NaiveDateTime, K), EndpointError> {
    let status = response.status();
    if !status.is_success() {
        let text = response.text().await?;
        let error: Result<ErrorResponse, _> = serde_json::from_str(&text);
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
            _ => ApiError::Other(status, text),
        }));
    }
    let expires = get_cache_expiry(req, &response);
    let result: K = response.json().await?;
    Ok((expires, result))
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
            write!(&mut query_string, ",{i}").expect("failed to concatenate ids");
        }
        result.push(query_string);
    }
    result
}

fn get_expire_from_header(response: &Response) -> Duration {
    let exp = get_header(response, "cache-control").unwrap_or(300);
    Duration::seconds(exp)
}

fn get_header<T: FromStr>(response: &Response, header: &str) -> Option<T> {
    response
        .headers()
        .iter()
        .find(|x| x.0 == header)
        .and_then(|(_, d)| d.to_str().ok())
        .and_then(|d| d.parse::<T>().ok())
}
