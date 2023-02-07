use std::{future::Future, time::Duration};

use async_trait::async_trait;
use redis::{aio::Connection, Client, RedisError};

use crate::{block, rate_limit::RateLimiter, EndpointError};

pub struct RedisRateLimiter {
    /// maximum number of requests in burst
    burst: usize,
    /// requests per minute
    refill: usize,
    client: Client,
}

impl RedisRateLimiter {
    #[cfg(feature = "blocking")]
    pub fn new(client: Client) -> Result<Self, RedisError> {
        block::block(Self::with_values(client, 300, 300))
    }

    #[cfg(not(feature = "blocking"))]
    pub fn new(client: Client) -> impl Future<Output = Result<Self, RedisError>> {
        Self::with_values(client, 300, 300)
    }

    /// burst takes the maximum number of requests in burst
    /// refill sets the requests per minute
    pub async fn with_values(
        client: Client,
        burst: usize,
        refill: usize,
    ) -> Result<Self, RedisError> {
        let this = Self {
            burst,
            refill,
            client,
        };

        this.setup().await?;

        Ok(this)
    }

    async fn setup(&self) -> Result<(), RedisError> {
        let mut conn = self.connection().await?;
        redis::cmd("FUNCTION")
            .arg("LOAD")
            .arg("REPLACE")
            .arg(include_str!("lib.lua"))
            .query_async(&mut conn)
            .await
    }

    fn connection(&self) -> impl Future<Output = Result<Connection, RedisError>> + '_ {
        self.client.get_async_connection()
    }
}

#[async_trait]
impl RateLimiter for RedisRateLimiter {
    async fn take(&self, num: usize) -> Result<Duration, EndpointError> {
        if num > self.burst {
            return Err(EndpointError::RateLimiterBucketExceeded);
        }

        let mut conn = self
            .connection()
            .await
            .map_err(|e| EndpointError::RateLimiterCrashed(e.to_string()))?;
        let wait = redis::cmd("FCALL")
            .arg("ratelimit_take")
            .arg(1)
            .arg("gw2lib_ratelimit")
            .arg(num)
            .arg(self.burst)
            .arg(self.refill)
            .query_async(&mut conn)
            .await
            .map_err(|e| EndpointError::RateLimiterCrashed(e.to_string()))?;

        Ok(Duration::from_millis(wait))
    }

    async fn penalize(&self) -> Result<(), EndpointError> {
        let mut conn = self
            .connection()
            .await
            .map_err(|e| EndpointError::RateLimiterCrashed(e.to_string()))?;
        redis::cmd("FCALL")
            .arg("ratelimit_penalize")
            .arg(1)
            .arg("gw2lib_ratelimit")
            .arg(self.refill)
            .query_async(&mut conn)
            .await
            .map_err(|e| EndpointError::RateLimiterCrashed(e.to_string()))
    }
}
