use std::{fmt::Display, hash::Hash};

use async_trait::async_trait;
use chrono::NaiveDateTime;
use gw2lib_model::{Endpoint, Language};
use serde::{de::DeserializeOwned, Serialize};

pub(crate) mod in_memory;
pub use in_memory::InMemoryCache;
#[cfg(feature = "redis")]
mod redis;
#[cfg(feature = "redis")]
pub use self::redis::RedisCache;

/// the interface for caching API responses
/// ### Remarks
/// expects the language to be part of the caching key where relevant
/// (`E::LOCALE`)
#[async_trait]
pub trait Cache {
    async fn insert<T, I, E, A>(
        &mut self,
        id: &I,
        endpoint: &T,
        expiring: NaiveDateTime,
        lang: Language,
        auth: &Option<A>,
    ) where
        T: DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
        I: Display + Hash + Sync + 'static + ?Sized,
        E: Endpoint,
        A: Display + Hash + Sync + 'static;

    async fn get<T, I, E, A>(&mut self, id: &I, lang: Language, auth: &Option<A>) -> Option<T>
    where
        T: DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
        I: Display + Hash + Sync + 'static + ?Sized,
        E: Endpoint,
        A: Display + Hash + Sync + 'static;

    async fn cleanup(&mut self);

    async fn wipe(&mut self) {
        self.wipe_static().await;
        self.wipe_authenticated().await;
    }

    async fn wipe_static(&mut self);

    async fn wipe_authenticated(&mut self);
}

#[async_trait]
pub(crate) trait CleanupCache {
    async fn cleanup(&mut self);
}

#[async_trait]
impl<T> CleanupCache for T
where
    T: Cache + Send + Sync + 'static,
{
    async fn cleanup(&mut self) {
        Cache::cleanup(self).await;
    }
}

pub struct NoopCache;
#[async_trait]
impl Cache for NoopCache {
    async fn insert<T, I, E, A>(
        &mut self,
        _id: &I,
        _endpoint: &T,
        _expiring: NaiveDateTime,
        _lang: Language,
        _auth: &Option<A>,
    ) where
        T: Clone + Send + Sync + 'static,
        I: Hash + Sync + 'static + ?Sized,
        E: Endpoint,
        A: Display + Sync + 'static,
    {
    }

    async fn get<T, I, E, A>(&mut self, _id: &I, _lang: Language, _auth: &Option<A>) -> Option<T>
    where
        T: Clone + Send + Sync + 'static,
        I: Hash + Sync + 'static + ?Sized,
        E: Endpoint,
        A: Display + Sync + 'static,
    {
        None
    }

    async fn cleanup(&mut self) {}

    async fn wipe_static(&mut self) {}

    async fn wipe_authenticated(&mut self) {}
}
