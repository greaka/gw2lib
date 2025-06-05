use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use tokio::sync::{oneshot, oneshot::Receiver, Mutex, Semaphore};

use crate::{
    rate_limit::{ApiPermit, RateLimiter},
    EndpointError,
};

pub struct BucketRateLimiter {
    /// maximum number of requests in burst
    burst: usize,
    /// requests per minute
    refill: usize,
    time: Mutex<Instant>,
    semaphore: Arc<Semaphore>,
}

impl BucketRateLimiter {
    /// burst takes the maximum number of requests in burst
    /// refill sets the requests per minute
    pub fn new(burst: usize, refill: usize) -> Self {
        let now = Instant::now();
        let max = (60_f64 * 1000_f64 * (burst as f64) / (refill as f64)) as u64;
        let base = now - Duration::from_millis(max);
        Self {
            burst,
            refill,
            time: base.into(),
            semaphore: Semaphore::new(burst).into(),
        }
    }
}

impl BucketRateLimiter {
    async fn estimated_eta<const UPDATE: bool>(&self, num: usize) -> Duration {
        let ratio = 60 * 1000 / self.refill as u64;
        let max = (60_f64 * 1000_f64 * (self.burst as f64) / (self.refill as f64)) as u64;

        let mut time = self.time.lock().await;
        let now = Instant::now();
        let base = now - Duration::from_millis(max);
        let mut value = *time;
        if value < base {
            value = base;
        }
        value += Duration::from_millis(ratio * num as u64);

        if UPDATE {
            *time = value;
        }

        value.checked_duration_since(now).unwrap_or(Duration::ZERO)
    }
}

impl Default for BucketRateLimiter {
    fn default() -> Self {
        Self::new(300, 300)
    }
}

impl RateLimiter for BucketRateLimiter {
    async fn take(
        self: &Arc<Self>,
        num: usize,
    ) -> Result<Receiver<ApiPermit<Self>>, EndpointError> {
        if num > self.burst {
            return Err(EndpointError::RateLimiterBucketExceeded);
        }

        let (tx, rx) = oneshot::channel();

        let semaphore = self.semaphore.clone();
        let rt = self.clone();
        crate::block::spawn(async move {
            let permits = semaphore.acquire_many(num as _).await.unwrap();
            permits.forget();

            let duration = rt.estimated_eta::<false>(num).await;
            tokio::time::sleep(duration).await;

            let permit = ApiPermit::new(rt, num);
            // ok() will drop the permit on error, adding the permits back
            tx.send(permit).ok();
            Some(())
        });

        Ok(rx)
    }

    async fn release_semaphore(&self, num: usize) -> Result<(), EndpointError> {
        self.estimated_eta::<true>(num).await;
        self.semaphore.add_permits(num as _);

        Ok(())
    }

    async fn penalize(&self) -> Result<(), EndpointError> {
        let ratio = 60 * 1000 / self.refill as u64;
        let now = Instant::now();
        let mut time = self.time.lock().await;
        if *time < now {
            *time = now;
        }
        // the api penalizes us for half a request worth of time when we hit it while
        // rate limited
        *time += Duration::from_millis(ratio / 2);
        Ok(())
    }
}
