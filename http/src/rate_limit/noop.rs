use std::sync::Arc;

use tokio::sync::{oneshot, oneshot::Receiver};

use crate::{
    rate_limit::{ApiPermit, RateLimiter},
    EndpointError,
};

pub struct NoopRateLimiter;

impl RateLimiter for NoopRateLimiter {
    async fn take(
        self: &Arc<Self>,
        num: usize,
    ) -> Result<Receiver<ApiPermit<Self>>, EndpointError> {
        let (tx, rx) = oneshot::channel();
        tx.send(ApiPermit::new(self.clone(), num)).ok();
        Ok(rx)
    }

    async fn release_semaphore(&self, _num: usize) -> Result<(), EndpointError> {
        Ok(())
    }

    async fn penalize(&self) -> Result<(), EndpointError> {
        Ok(())
    }
}
