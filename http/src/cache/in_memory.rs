use std::{
    any::{Any, TypeId},
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash, Hasher},
};

use async_trait::async_trait;
use chrono::{NaiveDateTime, Utc};
use dashmap::{mapref::entry::Entry, DashMap};
use gw2lib_model::{Endpoint, Language};

use crate::cache::Cache;

pub struct InMemoryCache {
    statics: DashMap<(TypeId, u64), (NaiveDateTime, Box<dyn Any + Send + Sync>)>,
    authenticated: DashMap<(TypeId, u64), (NaiveDateTime, Box<dyn Any + Send + Sync>)>,
}

impl Default for InMemoryCache {
    fn default() -> Self {
        let hasher = RandomState::new();
        Self {
            statics: DashMap::with_hasher(hasher.clone()),
            authenticated: DashMap::with_hasher(hasher),
        }
    }
}

#[async_trait]
impl Cache for InMemoryCache {
    async fn insert<T, I, E, A>(
        &self,
        id: &I,
        endpoint: &T,
        expiring: NaiveDateTime,
        lang: Language,
        auth: &Option<A>,
    ) where
        T: Clone + Send + Sync + 'static,
        I: Hash + Sync + 'static + ?Sized,
        E: Endpoint,
        A: Hash + Sync + 'static,
    {
        let hash = hash::<_, T, I, A>(self.statics.hasher(), id, E::LOCALE.then_some(lang), auth);
        let map = if E::AUTHENTICATED {
            &self.authenticated
        } else {
            &self.statics
        };
        map.insert(hash, (expiring, Box::new(endpoint.clone())));
    }

    async fn get<T, I, E, A>(&self, id: &I, lang: Language, auth: &Option<A>) -> Option<T>
    where
        T: Clone + Send + Sync + 'static,
        I: Hash + Sync + 'static + ?Sized,
        E: Endpoint,
        A: Hash + Sync + 'static,
    {
        let hash = hash::<_, T, I, A>(self.statics.hasher(), id, E::LOCALE.then_some(lang), auth);
        let map = if E::AUTHENTICATED {
            &self.authenticated
        } else {
            &self.statics
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

    async fn cleanup(&self) {
        let now = Utc::now().naive_utc();
        self.statics.retain(|_, (time, _)| *time < now);
        self.authenticated.retain(|_, (time, _)| *time < now);
    }

    async fn wipe_static(&self) {
        self.statics.clear();
    }

    async fn wipe_authenticated(&self) {
        self.authenticated.clear();
    }
}

#[inline]
pub(crate) fn hash<H: BuildHasher, T: 'static, I: 'static + Hash + ?Sized, A: 'static + Hash>(
    hasher: &H,
    id: &I,
    lang: Option<Language>,
    auth: &Option<A>,
) -> (TypeId, u64) {
    let type_id = TypeId::of::<T>();
    let hash = {
        let mut hasher = hasher.build_hasher();
        id.hash(&mut hasher);
        lang.hash(&mut hasher);
        auth.hash(&mut hasher);
        hasher.finish()
    };

    (type_id, hash)
}
