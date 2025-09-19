use std::{fmt::Display, future::Future, hash::Hash, ops::Deref, pin::Pin};

use chrono::NaiveDateTime;
use gw2lib_model::{Endpoint, Language};
use serde::{Serialize, de::DeserializeOwned};

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
pub trait Cache {
    fn insert<T, I, E, A>(
        &self,
        id: &I,
        endpoint: &T,
        expiring: NaiveDateTime,
        lang: Language,
        auth: &Option<A>,
    ) -> impl Future<Output = ()> + Send
    where
        T: DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
        I: Display + Hash + Sync + 'static + ?Sized,
        E: Endpoint,
        A: Display + Hash + Sync + 'static;

    fn get<T, I, E, A>(
        &self,
        id: &I,
        lang: Language,
        auth: &Option<A>,
    ) -> impl Future<Output = Option<T>> + Send
    where
        T: DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
        I: Display + Hash + Sync + 'static + ?Sized,
        E: Endpoint,
        A: Display + Hash + Sync + 'static;

    fn cleanup(&self) -> impl Future<Output = ()> + Send;

    #[expect(async_fn_in_trait)]
    async fn wipe(&self) {
        self.wipe_static().await;
        self.wipe_authenticated().await;
    }

    fn wipe_static(&self) -> impl Future<Output = ()> + Send;

    fn wipe_authenticated(&self) -> impl Future<Output = ()> + Send;
}

pub(crate) trait CleanupCache {
    fn cleanup(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}

impl<T> CleanupCache for T
where
    T: Cache + Send + Sync + 'static,
{
    fn cleanup(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(Cache::cleanup(self))
    }
}

impl<X, K> Cache for X
where
    X: Deref<Target = K> + Sync,
    K: Cache + Sync,
{
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
        A: Display + Hash + Sync + 'static,
    {
        self.deref()
            .insert::<T, I, E, A>(id, endpoint, expiring, lang, auth)
            .await
    }

    async fn get<T, I, E, A>(&self, id: &I, lang: Language, auth: &Option<A>) -> Option<T>
    where
        T: DeserializeOwned + Serialize + Clone + Send + Sync + 'static,
        I: Display + Hash + Sync + 'static + ?Sized,
        E: Endpoint,
        A: Display + Hash + Sync + 'static,
    {
        self.deref().get::<T, I, E, A>(id, lang, auth).await
    }

    async fn cleanup(&self) {
        self.deref().cleanup().await
    }

    async fn wipe_static(&self) {
        self.deref().wipe_static().await
    }

    async fn wipe_authenticated(&self) {
        self.deref().wipe_authenticated().await
    }
}
