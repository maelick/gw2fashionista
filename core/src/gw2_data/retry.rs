use tokio_retry::RetryIf;
use tokio_retry::strategy::{ExponentialBackoff, jitter};

use gw2lib::{ApiError, EndpointError};

pub struct Retry {
    max_retries: usize,
    sleep_millis: u64,
}

impl Retry {
    pub fn new(max_retries: usize, sleep_millis: u64) -> Self {
        Retry {
            max_retries,
            sleep_millis,
        }
    }

    pub async fn start<T, F, Fut>(&self, action: F) -> Result<T, EndpointError>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, EndpointError>>,
    {
        let retries = ExponentialBackoff::from_millis(self.sleep_millis)
            .map(jitter)
            .take(self.max_retries);
        RetryIf::start(retries, action, retryable).await
    }
}

impl Default for Retry {
    fn default() -> Self {
        Self::new(10, 1000)
    }
}

fn retryable(e: &EndpointError) -> bool {
    match e {
        EndpointError::ApiError(ApiError::Other(status, _)) => status.is_server_error(),
        EndpointError::RateLimiterCrashed(_)
        | EndpointError::RateLimiterBucketExceeded
        | EndpointError::RequestFailed(_)
        | EndpointError::ApiError(ApiError::RateLimited) => true,
        _ => false,
    }
}
