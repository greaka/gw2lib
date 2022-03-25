use std::sync::Mutex;

use chrono::{Duration, NaiveDateTime, Utc};
use ureq::{Error, Middleware, MiddlewareNext, Request, Response};

pub trait RateLimiter: Send {
    /// takes the amount of requests
    /// returns the seconds to wait before executing them
    fn take(&mut self, num: usize) -> u64;
    /// incurs a penalty, indicating that the rate limit was hit
    fn penalize(&mut self);
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

impl RateLimiter for BucketRateLimiter {
    fn take(&mut self, num: usize) -> u64 {
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
        if millis < -1000 {
            0
        } else {
            let millis = millis.abs();
            let modulo = millis % 1000 != 0;
            let ceil = millis / 1000 + (modulo as i64);
            ceil as _
        }
    }

    fn penalize(&mut self) {
        let ratio = 60 * 1000 / self.refill as i64;
        // the api penalizes us for half a request worth of time when we hit it while
        // rate limited
        self.time = Utc::now().naive_utc() + Duration::milliseconds(ratio / 2);
    }
}

pub struct NoopRateLimiter;
impl RateLimiter for NoopRateLimiter {
    fn take(&mut self, _num: usize) -> u64 {
        0
    }

    fn penalize(&mut self) {}
}

pub(crate) struct UreqRateLimit<T: RateLimiter + 'static>(Mutex<T>);

impl<T: RateLimiter + 'static> UreqRateLimit<T> {
    pub fn new(r: T) -> Self {
        Self(Mutex::new(r))
    }
}

impl<T: RateLimiter + 'static> Middleware for UreqRateLimit<T> {
    fn handle(&self, request: Request, next: MiddlewareNext) -> Result<Response, Error> {
        let sleep = { self.0.lock().unwrap().take(1) };
        if sleep > 0 {
            std::thread::sleep(std::time::Duration::from_secs(sleep));
        }

        let res = next.handle(request);

        if let Err(ureq::Error::Status(429, _)) = res {
            self.0.lock().unwrap().penalize();
        }

        res
    }
}
