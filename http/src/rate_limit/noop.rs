use std::time::Duration;

use async_trait::async_trait;

use crate::{rate_limit::RateLimiter, EndpointError};

pub struct NoopRateLimiter;
#[async_trait]
impl RateLimiter for NoopRateLimiter {
    async fn take(&self, _num: usize) -> Result<Duration, EndpointError> {
        Ok(Duration::ZERO)
    }

    async fn penalize(&self) -> Result<(), EndpointError> {
        Ok(())
    }
}
