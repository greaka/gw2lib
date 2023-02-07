use std::{
    fmt::{Display, Write},
    hash::Hash,
};

use async_trait::async_trait;
use chrono::{NaiveDateTime, Utc};
use gw2lib_model::{Endpoint, Language};
use redis::{
    aio::{Connection, ConnectionLike},
    AsyncCommands, Client, Cmd, RedisError,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::cache::Cache;

pub struct RedisCache {
    client: Client,
}

#[async_trait]
impl Cache for RedisCache {
    async fn insert<T, I, E, A>(
        &self,
        id: &I,
        endpoint: &T,
        expiring: NaiveDateTime,
        lang: Language,
        auth: &Option<A>,
    ) where
        T: DeserializeOwned + Serialize + Clone + Send + Sync + Sync + 'static,
        I: Display + Hash + Sync + 'static + ?Sized,
        E: Endpoint,
        A: Display + Hash + Sync + 'static,
    {
        let mut conn = match self.connection().await {
            Some(conn) => conn,
            None => return,
        };
        let ex = expiring - Utc::now().naive_utc();
        let key = self.gen_key::<E, I, A>(id, lang, auth);
        match serde_json::to_string(endpoint) {
            Ok(value) => {
                conn.set_ex::<_, _, ()>(
                    key,
                    value,
                    ex.num_seconds().try_into().unwrap_or_default(),
                )
                .await
                .ok();
            }
            _ => (),
        }
    }

    async fn get<T, I, E, A>(&self, id: &I, lang: Language, auth: &Option<A>) -> Option<T>
    where
        T: DeserializeOwned + Serialize + Clone + Send + Sync + Sync + 'static,
        I: Display + Hash + Sync + 'static + ?Sized,
        E: Endpoint,
        A: Display + Hash + Sync + 'static,
    {
        let mut conn = self.connection().await?;
        let key = self.gen_key::<E, I, A>(id, lang, auth);
        conn.get(key)
            .await
            .ok()
            .and_then(|x: String| serde_json::from_str(&x).ok())
    }

    async fn cleanup(&self) {}

    async fn wipe_static(&self) {
        self.delete_keys("gw2lib_static_*").await;
    }

    async fn wipe_authenticated(&self) {
        self.delete_keys("gw2lib_auth_*").await;
    }
}

impl RedisCache {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl RedisCache {
    async fn connection(&self) -> Option<Connection> {
        self.client.get_async_connection().await.ok()
    }

    async fn delete_keys(&self, pattern: &str) {
        let mut conn = match self.connection().await {
            Some(conn) => conn,
            None => return,
        };
        let cmd = redis::cmd("SCAN").arg("MATCH").arg(pattern).clone();
        let mut cursor = 0;

        loop {
            Self::delete_keys_from_cursor(&mut conn, cmd.clone(), &mut cursor)
                .await
                .ok();

            if cursor == 0 {
                break;
            }
        }
    }

    async fn delete_keys_from_cursor(
        conn: &mut Connection,
        mut cmd: Cmd,
        cursor: &mut u64,
    ) -> Result<(), RedisError> {
        let cmd = cmd.cursor_arg(*cursor);

        let res = conn.req_packed_command(cmd).await?;

        let (cur, chunk) = if res.looks_like_cursor() {
            redis::from_redis_value::<(u64, Vec<String>)>(&res)?
        } else {
            let res = redis::from_redis_value(&res)?;
            (0, res)
        };
        *cursor = cur;

        conn.del::<_, ()>(chunk).await
    }

    fn gen_key<E: Endpoint, I: Display + ?Sized, A: Display>(
        &self,
        id: &I,
        lang: Language,
        auth: &Option<A>,
    ) -> String {
        let mut key = String::with_capacity(128);
        let mut push = |s: &str| {
            key.push_str(s);
            key.push('_');
        };

        push("gw2lib");

        if E::AUTHENTICATED {
            push("auth");
        } else {
            push("static");
        }

        push(E::URL);

        if E::LOCALE {
            push(lang.as_str());
        }

        if E::AUTHENTICATED {
            write!(key, "{}_", auth.as_ref().unwrap()).unwrap();
        }

        write!(key, "{}", id).unwrap();

        key
    }
}
