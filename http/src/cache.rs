use std::{
    any::{Any, TypeId},
    collections::hash_map::Entry,
    hash::{Hash, Hasher},
};

use chrono::{NaiveDateTime, Utc};
use fxhash::{FxHashMap, FxHasher};
use gw2api_model::{Endpoint, Language};

/// the interface for caching API responses
/// ### Remarks
/// expects the language to be part of the caching key where relevant
/// (`E::LOCALE`)
pub trait Cache {
    fn insert<T, I, E>(&mut self, id: &I, endpoint: T, expiring: NaiveDateTime, lang: Language)
    where
        T: Clone + 'static,
        I: Hash + 'static,
        E: Endpoint;

    fn get<T, I, E>(&mut self, id: &I, lang: Language) -> Option<T>
    where
        T: Clone + 'static,
        I: Hash + 'static,
        E: Endpoint;

    fn cleanup(&mut self);

    fn wipe(&mut self) {
        self.wipe_static();
        self.wipe_authenticated();
    }

    fn wipe_static(&mut self);

    fn wipe_authenticated(&mut self);
}

#[derive(Default)]
pub struct InMemoryCache {
    statics: FxHashMap<(TypeId, u64), (NaiveDateTime, Box<dyn Any>)>,
    authenticated: FxHashMap<(TypeId, u64), (NaiveDateTime, Box<dyn Any>)>,
}

impl Cache for InMemoryCache {
    fn insert<T, I, E>(&mut self, id: &I, endpoint: T, expiring: NaiveDateTime, lang: Language)
    where
        T: Clone + 'static,
        I: Hash + 'static,
        E: Endpoint,
    {
        let hash = hash::<T, I>(id, E::LOCALE.then(|| lang));
        let map = if E::AUTHENTICATED {
            &mut self.authenticated
        } else {
            &mut self.statics
        };
        map.insert(hash, (expiring, Box::new(endpoint)));
    }

    fn get<T, I, E>(&mut self, id: &I, lang: Language) -> Option<T>
    where
        T: Clone + 'static,
        I: Hash + 'static,
        E: Endpoint,
    {
        let hash = hash::<T, I>(id, E::LOCALE.then(|| lang));
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

    fn cleanup(&mut self) {
        let now = Utc::now().naive_utc();
        self.statics.retain(|_, (time, _)| *time < now);
        self.authenticated.retain(|_, (time, _)| *time < now);
    }

    fn wipe_static(&mut self) {
        self.statics.clear();
    }

    fn wipe_authenticated(&mut self) {
        self.authenticated.clear();
    }
}

#[inline]
fn hash<T: 'static, I: 'static + Hash>(id: &I, lang: Option<Language>) -> (TypeId, u64) {
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
impl Cache for NoopCache {
    fn insert<T, I, E>(&mut self, _id: &I, _endpoint: T, _expiring: NaiveDateTime, _lang: Language)
    where
        T: Clone + 'static,
        I: Hash + 'static,
        E: Endpoint,
    {
    }

    fn get<T, I, E>(&mut self, _id: &I, _lang: Language) -> Option<T>
    where
        T: Clone + 'static,
        I: Hash + 'static,
        E: Endpoint,
    {
        None
    }

    fn cleanup(&mut self) {}

    fn wipe(&mut self) {}

    fn wipe_static(&mut self) {}

    fn wipe_authenticated(&mut self) {}
}
