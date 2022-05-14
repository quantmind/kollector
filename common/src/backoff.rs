use std::cmp::min;
use tokio::time::{sleep, Duration};

pub async fn millis_sleep(millis: u64) {
    let duration = Duration::from_millis(millis);
    sleep(duration).await;
}

/// Exponential backoff type.
#[derive(Debug, Clone)]
pub struct Backoff {
    //
    retries: u32,
    min: u32,
    max: u32,
    factor: u32,
    counter: u32,
    value: u32,
}

impl Backoff {
    pub fn new(retries: u32, min: u32, max: u32, factor: u32) -> Self {
        Self {
            retries,
            min,
            max,
            factor,
            counter: 0,
            value: 0,
        }
    }

    /// Get the next value for the retry count.
    pub fn next(&mut self) -> Option<u32> {
        if self.counter >= self.retries {
            return None;
        }
        let value = self.value;
        self.counter += 1;
        self.value = match self.counter {
            1 => self.min,
            _ => min(self.value * self.factor, self.max),
        };
        Some(value)
    }

    pub fn reset(&mut self) {
        self.value = self.min;
        self.counter = 0;
    }

    pub fn count(&self) -> u32 {
        self.counter
    }
}
