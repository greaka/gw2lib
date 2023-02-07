mod in_memory;
mod noop;
#[cfg(feature = "redis")]
mod redis;

use std::time::Duration;

use async_trait::async_trait;
pub use in_memory::BucketRateLimiter;
pub use noop::NoopRateLimiter;

#[cfg(feature = "redis")]
pub use self::redis::RedisRateLimiter;
use crate::EndpointError;

#[async_trait]
pub trait RateLimiter {
    /// takes the amount of requests
    /// returns the seconds to wait before executing them
    async fn take(&self, num: usize) -> Result<Duration, EndpointError>;
    /// incurs a penalty, indicating that the rate limit was hit
    async fn penalize(&self) -> Result<(), EndpointError>;
}
