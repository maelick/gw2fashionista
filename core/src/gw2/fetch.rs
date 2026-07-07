use std::{
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

#[async_trait]
pub trait Fetch<T, I> {
    fn endpoint_name(&self) -> &'static str;

    async fn ids(&self) -> Result<Vec<I>, Error>;

    async fn many(&self, ids: Vec<I>) -> Result<Vec<T>, Error>;

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

    async fn many(&self, ids: Vec<I>) -> Result<Vec<T>, Error> {
        Requester::many::<T, I>(&*self.client, ids.clone())
            .await
            .map_err(|e| Error::from_gw2lib(T::URL, format!("many(ids={ids:?})"), e))
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
    T: 'static,
    I: Clone + Send + Sync + 'static,
    F: Fetch<T, I> + Send + Sync,
{
    fn endpoint_name(&self) -> &'static str {
        self.inner.endpoint_name()
    }

    async fn ids(&self) -> Result<Vec<I>, Error> {
        self.start(|| self.inner.ids()).await
    }

    async fn many(&self, ids: Vec<I>) -> Result<Vec<T>, Error> {
        self.start(|| self.inner.many(ids.clone())).await
    }

    async fn single(&self, id: I) -> Result<T, Error> {
        self.start(|| self.inner.single(id.clone())).await
    }
}
