use std::time::{Duration, Instant};

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::{rate_limit::RateLimiter, EndpointError};

pub struct BucketRateLimiter {
    /// maximum number of requests in burst
    burst: usize,
    /// requests per minute
    refill: usize,
    time: Mutex<Instant>,
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
        }
    }
}

impl Default for BucketRateLimiter {
    fn default() -> Self {
        Self::new(300, 300)
    }
}

#[async_trait]
impl RateLimiter for BucketRateLimiter {
    async fn take(&self, num: usize) -> Result<Duration, EndpointError> {
        if num > self.burst {
            return Err(EndpointError::RateLimiterBucketExceeded);
        }
        let now = Instant::now();
        let max = (60_f64 * 1000_f64 * (self.burst as f64) / (self.refill as f64)) as u64;
        let base = now - Duration::from_millis(max);
        let ratio = 60 * 1000 / self.refill as u64;
        let mut time = self.time.lock().await;
        if *time < base {
            *time = base;
        }
        *time += Duration::from_millis(ratio * num as u64);

        Ok(time.checked_duration_since(now).unwrap_or(Duration::ZERO))
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
