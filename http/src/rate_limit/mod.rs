mod in_memory;
mod noop;
#[cfg(feature = "redis")]
mod redis;

use std::{future::Future, sync::Arc};

pub use in_memory::BucketRateLimiter;
pub use noop::NoopRateLimiter;
use tokio::sync::oneshot::Receiver;

#[cfg(feature = "redis")]
pub use self::redis::RedisRateLimiter;
use crate::EndpointError;

pub trait RateLimiter: Send + Sync + 'static {
    /// takes the amount of requests
    /// returns a receiver that notifies when the requests can be executed
    fn take(
        self: &Arc<Self>,
        num: usize,
    ) -> impl Future<Output = Result<Receiver<ApiPermit<Self>>, EndpointError>> + Send;

    /// used internally to release the amount of requests back into the bucket
    fn release_semaphore(
        &self,
        num: usize,
    ) -> impl Future<Output = Result<(), EndpointError>> + Send;
    /// incurs a penalty, indicating that the rate limit was hit
    fn penalize(&self) -> impl Future<Output = Result<(), EndpointError>> + Send;
}

#[must_use]
/// A permit to execute a task. Dropping this will call [`R::release_semaphore`]
pub struct ApiPermit<R>
where
    R: RateLimiter + ?Sized,
{
    rate_limiter: Arc<R>,
    amount: usize,
}

impl<'a, R: RateLimiter + ?Sized> ApiPermit<R> {
    pub fn new(rate_limiter: Arc<R>, amount: usize) -> Self {
        Self {
            rate_limiter,
            amount,
        }
    }

    pub fn rate_limiter(&self) -> &Arc<R> {
        &self.rate_limiter
    }

    pub fn amount(&self) -> usize {
        self.amount
    }
}

impl<'a, R: RateLimiter + ?Sized> Drop for ApiPermit<R> {
    fn drop(&mut self) {
        let rate_limiter = self.rate_limiter.clone();
        let amount = self.amount;
        crate::block::spawn(async move {
            #[allow(unused)]
            let res = rate_limiter.release_semaphore(amount).await;
            #[cfg(feature = "tracing")]
            if let Err(e) = res {
                tracing::warn!("rate limiter task failed to release semaphore: {}", e);
            }
        });
    }
}
