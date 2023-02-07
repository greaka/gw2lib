use std::{fmt::Display, hash::Hash};

use async_trait::async_trait;
use chrono::NaiveDateTime;
use gw2lib_model::{Endpoint, Language};
use serde::{de::DeserializeOwned, Serialize};

pub(crate) mod in_memory;
pub use in_memory::InMemoryCache;
pub use noop::NoopCache;
mod noop;
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
        &self,
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

    async fn get<T, I, E, A>(&self, id: &I, lang: Language, auth: &Option<A>) -> Option<T>
    where
        T: DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
        I: Display + Hash + Sync + 'static + ?Sized,
        E: Endpoint,
        A: Display + Hash + Sync + 'static;

    async fn cleanup(&self);

    async fn wipe(&self) {
        self.wipe_static().await;
        self.wipe_authenticated().await;
    }

    async fn wipe_static(&self);

    async fn wipe_authenticated(&self);
}

#[async_trait]
pub(crate) trait CleanupCache {
    async fn cleanup(&self);
}

#[async_trait]
impl<T> CleanupCache for T
where
    T: Cache + Send + Sync + 'static,
{
    async fn cleanup(&self) {
        Cache::cleanup(self).await;
    }
}
