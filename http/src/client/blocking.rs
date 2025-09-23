#![allow(private_bounds)]

use std::{fmt::Display, hash::Hash};

use chrono::Duration;
use gw2lib_model::{BulkEndpoint, Endpoint, EndpointWithId, FixedEndpoint};
use serde::{Serialize, de::DeserializeOwned};

use super::requester::Requester as Req;
use crate::{CachedRequest, Client, EndpointResult, Forced, block::block, client::AllowsClient};

pub trait Requester: Req {
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
    ) -> CachedRequest<'_, Self::Caching, Self::RateLimiting, Self::Authenticated, Self::Force>
    {
        Req::cached(self, cache_duration)
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
    ) -> CachedRequest<'_, Self::Caching, Self::RateLimiting, Self::Authenticated, Forced> {
        Req::forced(self)
    }

    /// call the fixed endpoint
    fn get<
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
        block(Req::get(self))
    }

    /// retrieves an item from cache
    /// ```
    /// use gw2lib::{Client, Requester, model::items::Item};
    ///
    /// let client = Client::default();
    /// let from_cache: Option<Item> = client.try_get();
    /// ```
    fn try_get<T>(&self) -> Option<T>
    where
        T: DeserializeOwned + Serialize + Clone + FixedEndpoint + Send + Sync + 'static,
    {
        block(Req::try_get(self))
    }

    /// request a single item
    fn single<T>(&self, id: <T as EndpointWithId>::IdType) -> EndpointResult<T>
    where
        T: DeserializeOwned
            + Serialize
            + Clone
            + Send
            + Sync
            + EndpointWithId
            + Endpoint<Authenticated: AllowsClient<Self::Authenticated>>
            + 'static,
        <T as EndpointWithId>::IdType:
            Display + DeserializeOwned + Hash + Send + Sync + Clone + 'static,
    {
        block(Req::single(self, id))
    }

    /// retrieves an item from cache
    /// ```
    /// use gw2lib::{Client, Requester, model::items::Item};
    ///
    /// let client = Client::default();
    /// let from_cache: Option<Item> = client.try_single(&19721);
    /// ```
    fn try_single<T>(&self, id: &<T as EndpointWithId>::IdType) -> Option<T>
    where
        T: DeserializeOwned + Serialize + Clone + EndpointWithId + Send + Sync + 'static,
        <T as EndpointWithId>::IdType:
            DeserializeOwned + Display + Hash + Clone + Send + Sync + 'static,
    {
        block(Req::try_single(self, id))
    }

    /// request all available ids
    fn ids<T>(&self) -> EndpointResult<Vec<<T as EndpointWithId>::IdType>>
    where
        T: DeserializeOwned
            + Serialize
            + EndpointWithId
            + Endpoint<Authenticated: AllowsClient<Self::Authenticated>>
            + Clone
            + Send
            + Sync
            + 'static,
        <T as EndpointWithId>::IdType:
            Display + DeserializeOwned + Serialize + Hash + Clone + Send + Sync + 'static,
    {
        block(Req::ids::<T>(self))
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
    fn try_ids<T>(&self) -> Option<Vec<<T as EndpointWithId>::IdType>>
    where
        T: DeserializeOwned + Serialize + Clone + EndpointWithId + Send + Sync + 'static,
        <T as EndpointWithId>::IdType:
            Display + DeserializeOwned + Serialize + Hash + Clone + Send + Sync + 'static,
    {
        block(Req::try_ids::<T>(self))
    }

    /// request multiple ids at once
    fn many<T>(&self, ids: Vec<<T as EndpointWithId>::IdType>) -> EndpointResult<Vec<T>>
    where
        T: DeserializeOwned
            + Serialize
            + EndpointWithId
            + Endpoint<Authenticated: AllowsClient<Self::Authenticated>>
            + BulkEndpoint
            + Clone
            + Send
            + Sync
            + 'static,
        <T as EndpointWithId>::IdType:
            Display + DeserializeOwned + Hash + Clone + Eq + Send + Sync + 'static,
    {
        block(Req::many(self, ids))
    }

    /// requests a page of items and returns the number of total items across
    /// all pages
    fn page<T>(&self, page: usize, page_size: u8, result: &mut Vec<T>) -> EndpointResult<usize>
    where
        T: DeserializeOwned
            + EndpointWithId
            + Endpoint<Authenticated: AllowsClient<Self::Authenticated>>
            + BulkEndpoint
            + Clone
            + Send
            + Sync
            + 'static,
        <T as EndpointWithId>::IdType: Display + DeserializeOwned + Hash + Clone + Sync + 'static,
    {
        block(Req::page(self, page, page_size, result))
    }

    /// requests all items using the most efficient method available
    /// ### Remarks
    /// for most endpoints this means using [`Self::get_all_by_requesting_ids`].
    /// Compared to [`Self::get_all_by_paging`]
    /// this needs to perform an additional request to get all ids, but is much
    /// more cache friendly, being able to utilize the cache and inflight
    /// mechanisms.
    fn all<T>(&self) -> EndpointResult<Vec<T>>
    where
        T: DeserializeOwned
            + Serialize
            + EndpointWithId
            + BulkEndpoint
            + Endpoint<Authenticated: AllowsClient<Self::Authenticated>>
            + Clone
            + Send
            + Sync
            + 'static,
        <T as EndpointWithId>::IdType:
            Display + DeserializeOwned + Serialize + Hash + Clone + Send + Sync + Eq + 'static,
    {
        block(Req::all(self))
    }

    /// Gets all items by querying ids=all
    ///
    /// use [`Self::all`] to use the most efficient way to request all items
    fn get_all_by_ids_all<T>(&self) -> EndpointResult<Vec<T>>
    where
        T: DeserializeOwned
            + Serialize
            + EndpointWithId
            + BulkEndpoint
            + Endpoint<Authenticated: AllowsClient<Self::Authenticated>>
            + Clone
            + Send
            + Sync
            + 'static,
        <T as EndpointWithId>::IdType: Display + DeserializeOwned + Hash + Clone + Sync + 'static,
    {
        block(Req::get_all_by_ids_all(self))
    }

    /// Gets all items by querying all pages
    ///
    /// use [`Self::all`] to use the most efficient way to request all items
    fn get_all_by_paging<T>(&self) -> EndpointResult<Vec<T>>
    where
        T: DeserializeOwned
            + EndpointWithId
            + BulkEndpoint
            + Endpoint<Authenticated: AllowsClient<Self::Authenticated>>
            + Clone
            + Send
            + Sync
            + 'static,
        <T as EndpointWithId>::IdType: Display + DeserializeOwned + Hash + Clone + Sync + 'static,
    {
        block(Req::get_all_by_paging(self))
    }

    /// Gets all items by querying all ids
    ///
    /// use [`Self::all`] to use the most efficient way to request all items
    fn get_all_by_requesting_ids<T>(&self) -> EndpointResult<Vec<T>>
    where
        T: DeserializeOwned
            + Serialize
            + EndpointWithId
            + Endpoint<Authenticated: AllowsClient<Self::Authenticated>>
            + BulkEndpoint
            + Clone
            + Send
            + Sync
            + 'static,
        <T as EndpointWithId>::IdType:
            Display + DeserializeOwned + Serialize + Hash + Clone + Send + Sync + Eq + 'static,
    {
        block(Req::get_all_by_requesting_ids(self))
    }
}

impl<T: Req> Requester for T {
    fn client(&self) -> &Client<Self::Caching, Self::RateLimiting, Self::Authenticated> {
        Req::client(self)
    }

    fn cache_duration(&self) -> Duration {
        Req::cache_duration(self)
    }
}
