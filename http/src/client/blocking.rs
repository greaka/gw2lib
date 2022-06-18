use std::{fmt::Display, hash::Hash};

use chrono::Duration;
use gw2api_model::{BulkEndpoint, Endpoint, EndpointWithId, FixedEndpoint};
use serde::de::DeserializeOwned;

use super::requester::Requester as Req;
use crate::{block::block, CachedRequest, Client, EndpointResult};

pub trait Requester<const AUTHENTICATED: bool, const FORCE: bool>:
    Req<AUTHENTICATED, FORCE>
{
    #[doc(hidden)]
    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, Self::Connector, AUTHENTICATED>;

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
    ) -> CachedRequest<Self::Caching, Self::RateLimiting, Self::Connector, AUTHENTICATED, FORCE>
    {
        Req::cached(self, cache_duration)
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
    ) -> CachedRequest<Self::Caching, Self::RateLimiting, Self::Connector, AUTHENTICATED, true>
    {
        Req::forced(self)
    }

    /// call the fixed endpoint
    fn get<T: DeserializeOwned + Clone + Send + Sync + FixedEndpoint + 'static>(
        &self,
    ) -> EndpointResult<T> {
        block(Req::get(self))
    }

    /// request a single item
    fn single<
        T: DeserializeOwned + Clone + Send + Sync + EndpointWithId<IdType = I> + 'static,
        I: Display + DeserializeOwned + Hash + Send + Sync + Clone + 'static,
    >(
        &self,
        id: I,
    ) -> EndpointResult<T> {
        block(Req::single(self, id))
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
        I: DeserializeOwned + Hash + Clone + Sync + 'static,
    >(
        &self,
        id: &I,
    ) -> Option<T> {
        block(Req::try_get(self, id))
    }

    /// request all available ids
    fn ids<
        T: DeserializeOwned + EndpointWithId<IdType = I> + Clone + Send + Sync + 'static,
        I: Display + DeserializeOwned + Hash + Clone + Send + Sync + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<I>> {
        block(Req::ids::<T, I>(self))
    }

    /// request multiple ids at once
    fn many<
        T: DeserializeOwned
            + EndpointWithId<IdType = I>
            + BulkEndpoint
            + Clone
            + Send
            + Sync
            + 'static,
        I: Display + DeserializeOwned + Hash + Clone + Eq + Send + Sync + 'static,
    >(
        &self,
        ids: Vec<I>,
    ) -> EndpointResult<Vec<T>> {
        block(Req::many(self, ids))
    }

    /// requests a page of items and returns the number of total items across
    /// all pages
    fn page<
        T: DeserializeOwned
            + EndpointWithId<IdType = I>
            + BulkEndpoint
            + Clone
            + Send
            + Sync
            + 'static,
        I: Display + DeserializeOwned + Hash + Clone + Sync + 'static,
    >(
        &self,
        page: usize,
        page_size: u8,
        result: &mut Vec<T>,
    ) -> EndpointResult<usize> {
        block(Req::page(self, page, page_size, result))
    }

    /// requests all items using the most efficient method available
    /// ### Remarks
    /// for most endpoints this means using [`Self::get_all_by_requesting_ids`].
    /// Compared to [`Self::get_all_by_paging`]
    /// this needs to perform an additional request to get all ids, but is much
    /// more cache friendly, being able to utilize the cache and inflight
    /// mechanisms.
    fn all<
        T: DeserializeOwned
            + EndpointWithId<IdType = I>
            + BulkEndpoint
            + Clone
            + Send
            + Sync
            + 'static,
        I: Display + DeserializeOwned + Hash + Clone + Send + Sync + Eq + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        block(Req::all(self))
    }

    /// Gets all items by querying ids=all
    ///
    /// use [`Self::all`] to use the most efficient way to request all items
    fn get_all_by_ids_all<
        T: DeserializeOwned
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
        block(Req::get_all_by_ids_all(self))
    }

    /// Gets all items by querying all pages
    ///
    /// use [`Self::all`] to use the most efficient way to request all items
    fn get_all_by_paging<
        T: DeserializeOwned
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
        block(Req::get_all_by_paging(self))
    }

    /// Gets all items by querying all ids
    ///
    /// use [`Self::all`] to use the most efficient way to request all items
    fn get_all_by_requesting_ids<
        T: DeserializeOwned
            + EndpointWithId<IdType = I>
            + BulkEndpoint
            + Clone
            + Send
            + Sync
            + 'static,
        I: Display + DeserializeOwned + Hash + Clone + Send + Sync + Eq + 'static,
    >(
        &self,
    ) -> EndpointResult<Vec<T>> {
        block(Req::get_all_by_requesting_ids(self))
    }
}

impl<T: Req<AUTHENTICATED, FORCE>, const AUTHENTICATED: bool, const FORCE: bool>
    Requester<AUTHENTICATED, FORCE> for T
{
    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, Self::Connector, AUTHENTICATED> {
        Req::client(self)
    }

    fn cache_duration(&self) -> Duration {
        Req::cache_duration(self)
    }
}
