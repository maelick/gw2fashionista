use std::time::Duration;

use gw2lib::{ApiError, EndpointError};

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

    pub async fn retry<T, F, Fut>(&self, mut f: F) -> Result<T, EndpointError>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, EndpointError>>,
    {
        for attempt in 0..=self.max_retries {
            match f().await {
                Ok(result) => return Ok(result),
                Err(e) if self.should_retry(&e) && attempt < self.max_retries => {
                    let sleep_duration = self.sleep_duration * (attempt + 1);
                    log::info!("GW2 API request failed with error {}, retrying in {:?}", e, sleep_duration);
                    tokio::time::sleep(sleep_duration).await;
                }
                Err(e) => return Err(e),
            }
        }
        unreachable!()
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