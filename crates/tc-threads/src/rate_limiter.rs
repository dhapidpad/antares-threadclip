use std::time::Instant;

#[derive(Debug)]
pub struct TokenBucketRateLimiter {
    capacity: f64,
    tokens: f64,
    refill_rate: f64, // tokens per second
    last_update: Instant,
}

impl TokenBucketRateLimiter {
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_rate,
            last_update: Instant::now(),
        }
    }

    pub fn default_threads_limiter() -> Self {
        // Threads limit: ~4800 calls per day => ~3.33 calls/min = ~0.055 calls/sec. Burst = 20.
        Self::new(20.0, 0.055)
    }

    pub fn try_acquire(&mut self) -> bool {
        self.refill();
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    pub fn remaining_tokens(&mut self) -> f64 {
        self.refill();
        self.tokens
    }

    pub fn time_until_available(&mut self) -> f64 {
        self.refill();
        if self.tokens >= 1.0 {
            0.0
        } else {
            (1.0 - self.tokens) / self.refill_rate
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.capacity);
        self.last_update = now;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_bucket() {
        let mut limiter = TokenBucketRateLimiter::new(3.0, 1.0);
        assert!(limiter.try_acquire());
        assert!(limiter.try_acquire());
        assert!(limiter.try_acquire());
        assert!(!limiter.try_acquire());
    }
}
