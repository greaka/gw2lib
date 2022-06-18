use async_trait::async_trait;
use chrono::{Duration, NaiveDateTime, Utc};

use crate::EndpointError;

#[async_trait]
pub trait RateLimiter: Send {
    /// takes the amount of requests
    /// returns the seconds to wait before executing them
    async fn take(&mut self, num: usize) -> Result<u64, EndpointError>;
    /// incurs a penalty, indicating that the rate limit was hit
    async fn penalize(&mut self) -> Result<(), EndpointError>;
}

pub struct BucketRateLimiter {
    /// maximum number of requests in burst
    burst: usize,
    /// requests per minute
    refill: usize,
    time: NaiveDateTime,
}

impl BucketRateLimiter {
    /// burst takes the maximum number of requests in burst
    /// refill sets the requests per minute
    pub fn new(burst: usize, refill: usize) -> Self {
        let now = Utc::now().naive_utc();
        let max = (60_f64 * 1000_f64 * (burst as f64) / (refill as f64)) as i64;
        let base = now - Duration::milliseconds(max);
        Self {
            burst,
            refill,
            time: base,
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
    async fn take(&mut self, num: usize) -> Result<u64, EndpointError> {
        let now = Utc::now().naive_utc();
        let max = (60_f64 * 1000_f64 * (self.burst as f64) / (self.refill as f64)) as i64;
        let base = now - Duration::milliseconds(max);
        if self.time < base {
            self.time = base;
        }
        let ratio = 60 * 1000 / self.refill as i64;
        self.time += Duration::milliseconds(ratio * num as i64);
        let sleep = self.time - now;
        let millis = sleep.num_milliseconds();
        // the api calculates the rate limit in intervals of 1s
        // as a result, this code ensures that we never hit it at the cost of a bit of
        // our burst
        Ok(if millis < -1000 {
            0
        } else {
            let millis = millis.abs();
            let modulo = millis % 1000 != 0;
            let ceil = millis / 1000 + (modulo as i64);
            ceil as _
        })
    }

    async fn penalize(&mut self) -> Result<(), EndpointError> {
        let ratio = 60 * 1000 / self.refill as i64;
        let now = Utc::now().naive_utc();
        if self.time < now {
            self.time = now;
        }
        // the api penalizes us for half a request worth of time when we hit it while
        // rate limited
        self.time += Duration::milliseconds(ratio / 2);
        Ok(())
    }
}

pub struct NoopRateLimiter;
#[async_trait]
impl RateLimiter for NoopRateLimiter {
    async fn take(&mut self, _num: usize) -> Result<u64, EndpointError> {
        Ok(0)
    }

    async fn penalize(&mut self) -> Result<(), EndpointError> {
        Ok(())
    }
}
