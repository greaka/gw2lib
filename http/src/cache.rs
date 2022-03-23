use std::{
    any::{Any, TypeId},
    collections::hash_map::Entry,
    hash::{Hash, Hasher},
};

use chrono::{NaiveDateTime, Utc};
use fxhash::{FxHashMap, FxHasher};

pub trait Cache {
    fn insert<T, I>(&mut self, id: &I, endpoint: T, expiring: NaiveDateTime)
    where
        T: Clone + 'static,
        I: Hash + 'static;

    fn get<T, I>(&mut self, id: &I) -> Option<T>
    where
        T: Clone + 'static,
        I: Hash + 'static;

    fn cleanup(&mut self);

    fn wipe(&mut self);
}

#[derive(Default)]
pub struct InMemoryCache {
    map: FxHashMap<(TypeId, u64), (NaiveDateTime, Box<dyn Any>)>,
}

impl Cache for InMemoryCache {
    fn insert<T, I>(&mut self, id: &I, endpoint: T, expiring: NaiveDateTime)
    where
        T: Clone + 'static,
        I: Hash + 'static,
    {
        let hash = hash::<T, I>(id);
        self.map.insert(hash, (expiring, Box::new(endpoint)));
    }

    fn get<T, I>(&mut self, id: &I) -> Option<T>
    where
        T: Clone + 'static,
        I: Hash + 'static,
    {
        let hash = hash::<T, I>(id);
        let entry = self.map.entry(hash);
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
        self.map.retain(|_, (time, _)| *time < now);
    }

    fn wipe(&mut self) {
        self.map.clear();
    }
}

#[inline]
fn hash<T: 'static, I: 'static + Hash>(id: &I) -> (TypeId, u64) {
    let type_id = TypeId::of::<T>();
    let hash = {
        let mut hasher = FxHasher::default();
        id.hash(&mut hasher);
        hasher.finish()
    };

    (type_id, hash)
}

pub struct NoopCache;
impl Cache for NoopCache {
    fn insert<T, I>(&mut self, _id: &I, _endpoint: T, _expiring: NaiveDateTime)
    where
        T: Clone + 'static,
        I: Hash + 'static,
    {
    }

    fn get<T, I>(&mut self, _id: &I) -> Option<T>
    where
        T: Clone + 'static,
        I: Hash + 'static,
    {
        None
    }

    fn cleanup(&mut self) {}

    fn wipe(&mut self) {}
}
