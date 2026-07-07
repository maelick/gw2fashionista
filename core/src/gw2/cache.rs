use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::Mutex;

use async_trait::async_trait;
use dashmap::DashMap;
use gw2lib::Requester;
use gw2lib::model::{BulkEndpoint, EndpointWithId};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::gw2::error::Error;

#[async_trait]
pub trait Resolver<T, I>
where
    T: DeserializeOwned
        + Serialize
        + Clone
        + Send
        + Sync
        + EndpointWithId<IdType = I>
        + BulkEndpoint
        + 'static,
    I: Display
        + Debug
        + DeserializeOwned
        + Serialize
        + Hash
        + Clone
        + Send
        + Sync
        + Eq
        + Copy
        + 'static,
{
    fn clear(&self);
    async fn ensure(&self, ids: Vec<I>) -> Result<(), Error>;
    async fn get(&self, id: I) -> Result<T, Error>;
    async fn get_many(&self, ids: Vec<I>) -> Result<Vec<T>, Error>;
    async fn get_all(&self) -> Result<Vec<T>, Error>;
}

pub struct Cache<T, I, Req> {
    client: Arc<Req>,
    _ids: Mutex<Vec<I>>,
    items: DashMap<I, T>,
}

impl<T, I, Req> Cache<T, I, Req>
where
    Req: Requester<false, false>,
    T: DeserializeOwned
        + Serialize
        + Clone
        + Send
        + Sync
        + EndpointWithId<IdType = I>
        + BulkEndpoint
        + 'static,
    I: Display
        + Debug
        + DeserializeOwned
        + Serialize
        + Hash
        + Clone
        + Send
        + Sync
        + Eq
        + Copy
        + 'static,
{
    pub fn new(client: Arc<Req>) -> Self {
        Cache {
            client,
            _ids: Mutex::new(Vec::new()),
            items: DashMap::new(),
        }
    }

    async fn _fetch_ids(&self) -> Result<Vec<I>, Error> {
        Ok(Requester::ids::<T, I>(&*self.client).await?)
    }

    async fn fetch_many(&self, ids: Vec<I>) -> Result<Vec<T>, Error> {
        Ok(Requester::many::<T, I>(&*self.client, ids).await?)
    }

    async fn fetch_single(&self, id: I) -> Result<T, Error> {
        Ok(Requester::single::<T, I>(&*self.client, id).await?)
    }
}

#[async_trait]
impl<T, I, Req> Resolver<T, I> for Cache<T, I, Req>
where
    Req: Requester<false, false> + Send + Sync,
    T: DeserializeOwned
        + Serialize
        + Clone
        + Send
        + Sync
        + EndpointWithId<IdType = I>
        + BulkEndpoint
        + 'static,
    I: Display
        + Debug
        + DeserializeOwned
        + Serialize
        + Hash
        + Clone
        + Send
        + Sync
        + Eq
        + Copy
        + 'static,
{
    fn clear(&self) {
        self.items.clear()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(endpoint = %T::URL)))]
    async fn ensure(&self, ids: Vec<I>) -> Result<(), Error> {
        let ids: Vec<_> = ids
            .into_iter()
            .filter(|id| !self.items.contains_key(id))
            .collect();

        if !ids.is_empty() {
            #[cfg(feature = "tracing")]
            tracing::info!(message = "Retrieving missing data from GW2 API", ?ids);

            let items = self.fetch_many(ids.clone()).await?;
            for (id, item) in ids.into_iter().zip(items) {
                self.items.insert(id, item);
            }
        }
        Ok(())
    }

    async fn get(&self, id: I) -> Result<T, Error> {
        if !self.items.contains_key(&id) {
            self.items.insert(id, self.fetch_single(id).await?);
        }
        Ok(self.items.get(&id).unwrap().clone())
    }

    async fn get_many(&self, ids: Vec<I>) -> Result<Vec<T>, Error> {
        self.ensure(ids.clone()).await?;
        let items = ids
            .iter()
            .filter_map(|id| self.items.get(id).map(|guard| guard.clone()));
        Ok(items.collect())
    }

    async fn get_all(&self) -> Result<Vec<T>, Error> {
        let mut ids = self._ids.lock().await;
        if ids.is_empty() {
            let new_ids = self._fetch_ids().await?;
            *ids = new_ids;
        }
        self.get_many(ids.clone()).await
    }
}
