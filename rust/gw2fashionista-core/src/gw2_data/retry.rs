use std::time::Duration;

use gw2lib::{ApiError, EndpointError};
use itertools::Itertools;

pub struct Retry {
    max_retries: u32,
    sleep_duration: Duration,
}

impl Retry {
    pub fn new(max_retries: u32, sleep_duration: Duration) -> Self {
        Retry{
            max_retries,
            sleep_duration,
        }
    }

    pub fn retry<T, F>(&self, mut f: F) -> Result<T, EndpointError>
    where
        F: FnMut() -> Result<T, EndpointError>,
    {
        (0..(self.max_retries + 1))
            .map(|attempt| (attempt, f()))
            .take_while_inclusive(|(attempt, res)| match res {
                Ok(_) => false,
                Err(e) if self.should_retry(e) && *attempt < self.max_retries => {
                    let sleep_duration = self.sleep_duration * (attempt + 1);
                    log::info!("GW2 API request failed with error {}, retrying in {:?}", e, sleep_duration);
                    std::thread::sleep(sleep_duration);
                    true
                },
                Err(_) => false,
            }).last().unwrap().1
    }

    fn should_retry(&self, e: &EndpointError) -> bool {
        match e {
            EndpointError::ApiError(ApiError::Other(status, _)) => status.is_server_error(),
            EndpointError::RateLimiterCrashed(_)
            | EndpointError::RateLimiterBucketExceeded
            | EndpointError::RequestFailed(_)
            | EndpointError::ApiError(ApiError::RateLimited) => true,
            _ => false
        }
    }
}

impl Default for Retry {
    fn default() -> Self {
        Self::new(10, Duration::from_millis(1000))
    }
}