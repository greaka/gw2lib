use std::{
    any::{Any, TypeId},
    collections::hash_map::Entry,
    hash::{Hash, Hasher},
};

use async_trait::async_trait;
use chrono::{NaiveDateTime, Utc};
use fxhash::{FxHashMap, FxHasher};
use gw2lib_model::{Endpoint, Language};

/// the interface for caching API responses
/// ### Remarks
/// expects the language to be part of the caching key where relevant
/// (`E::LOCALE`)
#[async_trait]
pub trait Cache {
    async fn insert<T, I, E>(
        &mut self,
        id: &I,
        endpoint: T,
        expiring: NaiveDateTime,
        lang: Language,
    ) where
        T: Clone + Send + Sync + 'static,
        I: Hash + Sync + 'static,
        E: Endpoint;

    async fn get<T, I, E>(&mut self, id: &I, lang: Language) -> Option<T>
    where
        T: Clone + Send + Sync + 'static,
        I: Hash + Sync + 'static,
        E: Endpoint;

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

#[derive(Default)]
pub struct InMemoryCache {
    statics: FxHashMap<(TypeId, u64), (NaiveDateTime, Box<dyn Any + Send + Sync>)>,
    authenticated: FxHashMap<(TypeId, u64), (NaiveDateTime, Box<dyn Any + Send + Sync>)>,
}

#[async_trait]
impl Cache for InMemoryCache {
    async fn insert<T, I, E>(
        &mut self,
        id: &I,
        endpoint: T,
        expiring: NaiveDateTime,
        lang: Language,
    ) where
        T: Clone + Send + Sync + 'static,
        I: Hash + Sync + 'static,
        E: Endpoint,
    {
        let hash = hash::<T, I>(id, E::LOCALE.then_some(lang));
        let map = if E::AUTHENTICATED {
            &mut self.authenticated
        } else {
            &mut self.statics
        };
        map.insert(hash, (expiring, Box::new(endpoint)));
    }

    async fn get<T, I, E>(&mut self, id: &I, lang: Language) -> Option<T>
    where
        T: Clone + Send + Sync + 'static,
        I: Hash + Sync + 'static,
        E: Endpoint,
    {
        let hash = hash::<T, I>(id, E::LOCALE.then_some(lang));
        let map = if E::AUTHENTICATED {
            &mut self.authenticated
        } else {
            &mut self.statics
        };
        let entry = map.entry(hash);
        match entry {
            Entry::Occupied(entry) => {
                let (expiring, any) = entry.get();
                let now = Utc::now().naive_utc();
                if now < *expiring {
                    any.downcast_ref().cloned()
                } else {
                    entry.remove();
                    None
                }
            }
            Entry::Vacant(_) => None,
        }
    }

    async fn cleanup(&mut self) {
        let now = Utc::now().naive_utc();
        self.statics.retain(|_, (time, _)| *time < now);
        self.authenticated.retain(|_, (time, _)| *time < now);
    }

    async fn wipe_static(&mut self) {
        self.statics.clear();
    }

    async fn wipe_authenticated(&mut self) {
        self.authenticated.clear();
    }
}

#[inline]
pub(crate) fn hash<T: 'static, I: 'static + Hash>(id: &I, lang: Option<Language>) -> (TypeId, u64) {
    let type_id = TypeId::of::<T>();
    let hash = {
        let mut hasher = FxHasher::default();
        id.hash(&mut hasher);
        lang.hash(&mut hasher);
        hasher.finish()
    };

    (type_id, hash)
}

pub struct NoopCache;
#[async_trait]
impl Cache for NoopCache {
    async fn insert<T, I, E>(
        &mut self,
        _id: &I,
        _endpoint: T,
        _expiring: NaiveDateTime,
        _lang: Language,
    ) where
        T: Clone + Send + Sync + 'static,
        I: Hash + Sync + 'static,
        E: Endpoint,
    {
    }

    async fn get<T, I, E>(&mut self, _id: &I, _lang: Language) -> Option<T>
    where
        T: Clone + Send + Sync + 'static,
        I: Hash + Sync + 'static,
        E: Endpoint,
    {
        None
    }

    async fn cleanup(&mut self) {}

    async fn wipe_static(&mut self) {}

    async fn wipe_authenticated(&mut self) {}
}
