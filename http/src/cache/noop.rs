use std::{fmt::Display, hash::Hash};

use chrono::NaiveDateTime;
use gw2lib_model::{Endpoint, Language};

use crate::cache::Cache;

pub struct NoopCache;

impl Cache for NoopCache {
    async fn insert<T, I, E, A>(
        &self,
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

    async fn get<T, I, E, A>(&self, _id: &I, _lang: Language, _auth: &Option<A>) -> Option<T>
    where
        T: Clone + Send + Sync + 'static,
        I: Hash + Sync + 'static + ?Sized,
        E: Endpoint,
        A: Display + Sync + 'static,
    {
        None
    }

    async fn cleanup(&self) {}

    async fn wipe_static(&self) {}

    async fn wipe_authenticated(&self) {}
}
