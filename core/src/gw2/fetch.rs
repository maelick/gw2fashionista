use std::{fmt::Display, hash::Hash, sync::Arc};

use async_trait::async_trait;
use gw2lib::{
    Requester,
    model::{BulkEndpoint, EndpointWithId},
};
use serde::{Serialize, de::DeserializeOwned};

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
    I: Display + Hash + Eq + Clone + Send + Sync + Serialize + DeserializeOwned + 'static,
    Req: Requester<AUTH, false> + Send + Sync,
{
    fn endpoint_name(&self) -> &'static str {
        T::URL
    }

    async fn ids(&self) -> Result<Vec<I>, Error> {
        Ok(Requester::ids::<T, I>(&*self.client).await?)
    }

    async fn many(&self, ids: Vec<I>) -> Result<Vec<T>, Error> {
        Ok(Requester::many::<T, I>(&*self.client, ids).await?)
    }

    async fn single(&self, id: I) -> Result<T, Error> {
        Ok(Requester::single::<T, I>(&*self.client, id).await?)
    }
}
