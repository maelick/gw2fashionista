use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    sync::Arc,
};

use async_trait::async_trait;
use gw2lib::{
    Requester,
    model::{BulkEndpoint, EndpointWithId},
};
use serde::{Serialize, de::DeserializeOwned};
use tokio_retry::RetryIf;
use tokio_retry::strategy::{ExponentialBackoff, jitter};

use crate::gw2::error::Error;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Fetch<T, I>
where
    T: Sync,
    I: Send + Sync,
{
    fn endpoint_name(&self) -> &'static str;

    async fn ids(&self) -> Result<Vec<I>, Error>;

    /// Returns a map of successfully retrieved objects.
    /// If some (or all) are missing, the method returns a partial result instead of an ErrorKind::NotFound.
    async fn many(&self, ids: &[I]) -> Result<HashMap<I, T>, Error>;

    async fn single(&self, id: I) -> Result<T, Error>;
}

pub struct Gw2LibFetcher<Req, const AUTH: bool> {
    client: Arc<Req>,
}

impl<Req, const AUTH: bool> Gw2LibFetcher<Req, AUTH> {
    pub fn new(client: Arc<Req>) -> Self {
        Gw2LibFetcher { client }
    }
}

#[async_trait]
impl<T, I, Req, const AUTH: bool> Fetch<T, I> for Gw2LibFetcher<Req, AUTH>
where
    T: EndpointWithId<IdType = I>
        + BulkEndpoint
        + Clone
        + Send
        + Sync
        + Serialize
        + DeserializeOwned
        + 'static,
    I: Display + Debug + Hash + Eq + Clone + Send + Sync + Serialize + DeserializeOwned + 'static,
    Req: Requester<AUTH, false> + Send + Sync,
{
    fn endpoint_name(&self) -> &'static str {
        T::URL
    }

    async fn ids(&self) -> Result<Vec<I>, Error> {
        Requester::ids::<T, I>(&*self.client)
            .await
            .map_err(|e| Error::from_gw2lib(T::URL, "ids()".to_string(), e))
    }

    async fn many(&self, ids: &[I]) -> Result<HashMap<I, T>, Error> {
        let result = Requester::many::<T, I>(&*self.client, ids.to_vec())
            .await
            .map_err(|e| Error::from_gw2lib(T::URL, format!("many(ids={ids:?})"), e));
        match result {
            Ok(objects) => Ok(objects.into_iter().map(|o| (o.id().clone(), o)).collect()),
            Err(e) => {
                if e.is_not_found() {
                    Ok(HashMap::new())
                } else {
                    Err(e)
                }
            }
        }
    }

    async fn single(&self, id: I) -> Result<T, Error> {
        Requester::single::<T, I>(&*self.client, id.clone())
            .await
            .map_err(|e| Error::from_gw2lib(T::URL, format!("single(id={id:?})"), e))
    }
}

impl<Req, const AUTH: bool> Clone for Gw2LibFetcher<Req, AUTH> {
    fn clone(&self) -> Self {
        Gw2LibFetcher {
            client: self.client.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Retry<F> {
    inner: F,
    max_retries: usize,
    sleep_millis: u64,
}

impl<F> Retry<F> {
    pub const DEFAULT_MAX_RETRIES: usize = 10;
    pub const DEFAULT_SLEEP_MILLIS: u64 = 1000;

    pub fn new(inner: F) -> Self {
        Retry {
            inner,
            max_retries: Self::DEFAULT_MAX_RETRIES,
            sleep_millis: Self::DEFAULT_SLEEP_MILLIS,
        }
    }

    pub fn with_max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_sleep_millis(mut self, sleep_millis: u64) -> Self {
        self.sleep_millis = sleep_millis;
        self
    }

    pub async fn start<T, A, Fut>(&self, action: A) -> Result<T, Error>
    where
        A: FnMut() -> Fut,
        Fut: Future<Output = Result<T, Error>>,
    {
        let retries = ExponentialBackoff::from_millis(self.sleep_millis)
            .map(jitter)
            .take(self.max_retries);
        RetryIf::start(retries, action, Error::is_transient).await
    }
}

#[async_trait]
impl<T, I, F> Fetch<T, I> for Retry<F>
where
    T: Sync + 'static,
    I: Clone + Send + Sync + 'static,
    F: Fetch<T, I> + Send + Sync,
{
    fn endpoint_name(&self) -> &'static str {
        self.inner.endpoint_name()
    }

    async fn ids(&self) -> Result<Vec<I>, Error> {
        self.start(|| self.inner.ids()).await
    }

    async fn many(&self, ids: &[I]) -> Result<HashMap<I, T>, Error> {
        self.start(|| self.inner.many(ids)).await
    }

    async fn single(&self, id: I) -> Result<T, Error> {
        self.start(|| self.inner.single(id.clone())).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gw2lib::{ApiError, EndpointError};
    use hyper::StatusCode;
    use mockall::predicate;

    #[tokio::test(start_paused = true)]
    async fn test_no_retry_on_success() {
        let mut mock = MockFetch::new();
        mock_single_with_api_response(&mut mock, 200, 1);

        let retry = Retry::new(mock);
        let result = retry.single(42).await.unwrap();
        assert_eq!(result, "peekaboo");
    }

    #[tokio::test(start_paused = true)]
    async fn test_retry_on_500_error() {
        let mut mock = MockFetch::new();
        mock_single_with_api_response(&mut mock, 500, 1);
        mock_single_with_api_response(&mut mock, 200, 1);

        let retry = Retry::new(mock);
        let result = retry.single(42).await.unwrap();
        assert_eq!(result, "peekaboo");
    }

    #[tokio::test(start_paused = true)]
    async fn test_retry_on_500_error_with_max_retries() {
        let mut mock = MockFetch::new();
        mock_single_with_api_response(&mut mock, 500, 1);
        mock_single_with_api_response(&mut mock, 200, 1);

        let retry = Retry::new(mock).with_max_retries(1);
        let result = retry.single(42).await.unwrap();
        assert_eq!(result, "peekaboo");
    }

    #[tokio::test(start_paused = true)]
    async fn test_retry_on_500_error_with_sleep_millis() {
        let mut mock = MockFetch::new();
        mock_single_with_api_response(&mut mock, 500, 10);
        mock_single_with_api_response(&mut mock, 200, 1);

        let retry = Retry::new(mock).with_sleep_millis(10000);
        let result = retry.single(42).await.unwrap();
        assert_eq!(result, "peekaboo");
    }

    #[tokio::test(start_paused = true)]
    async fn test_retry_on_500_error_with_max_retries_reached() {
        let mut mock = MockFetch::new();
        mock_single_with_api_response(&mut mock, 500, 2);

        let retry = Retry::new(mock).with_max_retries(1);
        let result = retry.single(42).await.unwrap_err();
        assert!(result.is_transient());
    }

    #[tokio::test(start_paused = true)]
    async fn test_retry_on_404_error() {
        let mut mock = MockFetch::new();
        mock_single_with_api_response(&mut mock, 404, 1);

        let retry = Retry::new(mock);
        let result = retry.single(42).await.unwrap_err();
        assert!(result.is_not_found());
    }

    #[tokio::test(start_paused = true)]
    async fn test_retry_on_500_and_404_error() {
        let mut mock = MockFetch::new();
        mock_single_with_api_response(&mut mock, 500, 1);
        mock_single_with_api_response(&mut mock, 404, 1);

        let retry = Retry::new(mock);
        let result = retry.single(42).await.unwrap_err();
        assert!(result.is_not_found());
    }

    #[tokio::test(start_paused = true)]
    async fn test_retry_on_permanent_error() {
        let mut mock: MockFetch<String, u32> = MockFetch::new();
        mock.expect_single()
            .with(predicate::eq(42))
            .times(1)
            .returning(|_| {
                Err(Error::from_gw2lib(
                    "peekaboo",
                    "id=42".to_string(),
                    EndpointError::NotAuthenticated,
                ))
            });

        let retry = Retry::new(mock);
        let result = retry.single(42).await.unwrap_err();
        assert!(!result.is_transient());
    }

    fn mock_single_with_api_response<'a>(
        mock: &'a mut MockFetch<String, u32>,
        status: u16,
        times: usize,
    ) {
        mock.expect_single()
            .with(predicate::eq(42))
            .times(times)
            .returning(move |_| {
                if status == 200 {
                    Ok("peekaboo".to_string())
                } else if status >= 400 && status < 600 {
                    Err(build_api_error(
                        "peekaboo",
                        "id=42",
                        status,
                        "an error occured",
                    ))
                } else {
                    panic!("unsupported mocked status: {:?}", status)
                }
            });
    }

    fn build_api_error(
        endpoint: &'static str,
        request: &str,
        status: u16,
        error_msg: &str,
    ) -> Error {
        Error::from_gw2lib(
            endpoint,
            request.to_string(),
            EndpointError::ApiError(ApiError::Other(
                StatusCode::from_u16(status).unwrap(),
                error_msg.to_string(),
            )),
        )
    }
}
