use std::{
    collections::VecDeque,
    fmt::Display,
    future::Future,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use futures::{FutureExt, StreamExt};
use kanal::{AsyncReceiver, AsyncSender};
use rand::distributions::{Alphanumeric, DistString};
use redis::{
    AsyncConnectionConfig, Client, PushInfo, RedisError, RedisResult, ToRedisArgs,
    aio::MultiplexedConnection,
};
use tokio::{
    select,
    sync::{Mutex, broadcast, broadcast::Sender, oneshot, oneshot::Receiver},
    time,
    time::{Duration, Instant},
};

use crate::{
    EndpointError,
    rate_limit::{ApiPermit, RateLimiter},
};

#[derive(Debug, Clone)]
pub struct RedisRateLimiter {
    /// maximum number of requests in burst
    burst: usize,
    /// requests per minute
    refill: usize,
    client: Client,
    connections: Arc<Mutex<VecDeque<MultiplexedConnection>>>,
    push_sender: Sender<PushInfo>,
    wait_sender: AsyncSender<()>,
    wait_receiver: AsyncReceiver<()>,

    // redis keys
    bucket: Arc<str>,
    semaphore: Arc<str>,
    pubsub: Arc<str>,
    waitlist: Arc<str>,
}

impl RedisRateLimiter {
    #[cfg(feature = "blocking")]
    pub fn new(client: Client) -> Result<Self, EndpointError> {
        crate::block::block(Self::with_values(client, 300, 300))
    }

    #[cfg(not(feature = "blocking"))]
    pub fn new(client: Client) -> impl Future<Output = Result<Self, EndpointError>> {
        Self::with_values(client, 300, 300)
    }

    /// burst takes the maximum number of requests in burst
    /// refill sets the requests per minute
    /// a refill value above 60000 might break the rate limiter
    pub fn with_values(
        client: Client,
        burst: usize,
        refill: usize,
    ) -> impl Future<Output = Result<Self, EndpointError>> {
        Self::with_values_and_shard(client, burst, refill, "")
    }

    /// different shards use separate rate limits
    pub async fn with_values_and_shard(
        client: Client,
        burst: usize,
        refill: usize,
        shard: impl Display,
    ) -> Result<Self, EndpointError> {
        let (tx, _) = broadcast::channel(burst);
        let (wait_sender, wait_receiver) = kanal::bounded_async(0);
        let this = Self {
            burst,
            refill,
            client,
            connections: Default::default(),
            push_sender: tx,
            wait_sender,
            wait_receiver,

            bucket: format!("ratelimit_bucket_{}", shard).into(),
            semaphore: format!("ratelimit_semaphore_{}", shard).into(),
            pubsub: format!("ratelimit_pub_{}", shard).into(),
            waitlist: format!("ratelimit_waitlist_{}", shard).into(),
        };

        this.setup().await?;

        Ok(this)
    }

    async fn setup(&self) -> Result<(), EndpointError> {
        let mut conn = self.connection().await?;

        redis::cmd("FUNCTION")
            .arg("LOAD")
            .arg("REPLACE")
            .arg(include_str!("lib.lua"))
            .query_async(&mut *conn)
            .await
            .map_err(|e| EndpointError::RateLimiterCrashed(e.to_string()))
    }

    async fn connection(&self) -> Result<ConnectionGuard, EndpointError> {
        let conn = loop {
            let (conn, count) = {
                let mut conns = self.connections.lock().await;
                (conns.pop_front(), conns.len() as u32)
            };
            if let Some(mut conn) = conn {
                let res: Result<RedisResult<String>, _> = time::timeout(
                    Duration::from_millis(200),
                    redis::cmd("PING").query_async(&mut conn),
                )
                .await;
                match res {
                    Ok(Ok(msg)) if msg == "PONG" => break conn,
                    _ => continue,
                }
            } else if self.wait_sender.sender_count() + count > self.burst {
                self.wait_receiver
                    .recv()
                    .await
                    .map_err(|e| EndpointError::RateLimiterCrashed(e.to_string()))?;
                continue;
            } else {
                let config = AsyncConnectionConfig::new().set_push_sender(self.push_sender.clone());
                let conn = self
                    .client
                    .get_multiplexed_async_connection_with_config(&config)
                    .await
                    .map_err(|e| EndpointError::RateLimiterCrashed(e.to_string()))?;
                break conn;
            }
        };

        Ok(ConnectionGuard {
            conn: Some(conn),
            pool: self.connections.clone(),
            notify: self.wait_sender.clone(),
        })
    }

    async fn redis_take(
        &self,
        num: usize,
        id: impl ToRedisArgs,
    ) -> Result<Duration, EndpointError> {
        if num > self.burst {
            return Err(EndpointError::RateLimiterBucketExceeded);
        }

        let mut conn = self.connection().await?;

        let wait = redis::cmd("FCALL")
            .arg("ratelimit_take")
            .arg(4)
            .arg(self.bucket.as_ref())
            .arg(self.semaphore.as_ref())
            .arg(self.pubsub.as_ref())
            .arg(self.waitlist.as_ref())
            .arg(num)
            .arg(self.burst)
            .arg(self.refill)
            .arg(id)
            .query_async(&mut *conn)
            .await
            .map_err(|e| EndpointError::RateLimiterCrashed(e.to_string()))?;

        Ok(Duration::from_millis(wait))
    }

    async fn redis_poke(&self) -> Result<(), EndpointError> {
        let mut conn = self.connection().await?;

        redis::cmd("FCALL")
            .arg("ratelimit_poke")
            .arg(4)
            .arg(self.bucket.as_ref())
            .arg(self.semaphore.as_ref())
            .arg(self.pubsub.as_ref())
            .arg(self.waitlist.as_ref())
            .arg(self.burst)
            .arg(self.refill)
            .query_async(&mut *conn)
            .await
            .map_err(|e| EndpointError::RateLimiterCrashed(e.to_string()))
    }

    async fn redis_release(&self, num: usize) -> Result<(), EndpointError> {
        let mut conn = self.connection().await?;

        redis::cmd("FCALL")
            .arg("ratelimit_release")
            .arg(4)
            .arg(self.bucket.as_ref())
            .arg(self.semaphore.as_ref())
            .arg(self.pubsub.as_ref())
            .arg(self.waitlist.as_ref())
            .arg(num)
            .arg(self.burst)
            .arg(self.refill)
            .query_async(&mut *conn)
            .await
            .map_err(|e| EndpointError::RateLimiterCrashed(e.to_string()))
    }

    async fn redis_penalize(&self) -> Result<(), EndpointError> {
        let mut conn = self.connection().await?;

        redis::cmd("FCALL")
            .arg("ratelimit_penalize")
            .arg(1)
            .arg(self.bucket.as_ref())
            .arg(self.refill)
            .query_async(&mut *conn)
            .await
            .map_err(|e| EndpointError::RateLimiterCrashed(e.to_string()))
    }
}

impl RateLimiter for RedisRateLimiter {
    async fn take(
        self: &Arc<Self>,
        num: usize,
    ) -> Result<Receiver<ApiPermit<Self>>, EndpointError> {
        let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

        let mut conn = self.connection().await?;

        let mut sub = self.push_sender.subscribe();

        conn.subscribe(self.pubsub.as_ref())
            .await
            .map_err(|e| EndpointError::RateLimiterCrashed(e.to_string()))?;

        let eta = self.redis_take(num, &id).await?;

        let (tx, rx) = oneshot::channel();
        let this = self.clone();

        crate::block::spawn(async move {
            let mut target_time = Instant::now() + eta + Duration::from_secs(1);
            let mut counter = 0;
            loop {
                select! {
                    msg = sub.recv() => match msg {
                        Some(msg) => {
                            let msg: String = msg.get_payload().unwrap();
                            if msg == id {
                                counter += 1;
                                if counter >= num {
                                    break;
                                }
                            }
                        }
                        None => break,
                    },
                    _ = time::sleep_until(target_time) => {
                        target_time += Duration::from_secs(5);
                        this.redis_poke().await.ok();
                    }
                }
            }

            let permit = ApiPermit::new(this, num);
            // ok() will drop the permit on error, adding the permits back
            tx.send(permit).ok();
        });

        Ok(rx)
    }

    async fn release_semaphore(&self, num: usize) -> Result<(), EndpointError> {
        self.redis_release(num).await
    }

    async fn penalize(&self) -> Result<(), EndpointError> {
        self.redis_penalize().await
    }
}

struct ConnectionGuard {
    conn: Option<MultiplexedConnection>,
    pool: Arc<Mutex<VecDeque<MultiplexedConnection>>>,
    notify: AsyncSender<()>,
}

impl Deref for ConnectionGuard {
    type Target = MultiplexedConnection;

    fn deref(&self) -> &Self::Target {
        self.conn.as_ref().unwrap()
    }
}

impl DerefMut for ConnectionGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.conn.as_mut().unwrap()
    }
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        crate::block::spawn(async {
            self.pool.lock().await.push_back(self.conn.take().unwrap());
            self.notify.try_send(()).ok();
        });
    }
}
