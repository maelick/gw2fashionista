use std::fmt::Display;
use std::hash::Hash;
use std::sync::Arc;
use tokio::sync::Mutex;

use dashmap::DashMap;
use gw2lib::model::{BulkEndpoint, EndpointWithId};
use gw2lib::{Requester, EndpointError};
use serde::Serialize;
use serde::de::DeserializeOwned;
use async_trait::async_trait;

#[async_trait]
pub trait Resolver<T, I>
where
    T: DeserializeOwned + Serialize + Clone + Send + Sync + EndpointWithId<IdType = I> + BulkEndpoint + 'static,
    I: Display + DeserializeOwned + Serialize + Hash + Clone + Send + Sync + Eq + Copy + 'static,
{
    fn clear(&self);
    async fn ensure(&self, ids: Vec<I>) -> Result<(), EndpointError>;
    async fn get(&self, id: I) -> Result<T, EndpointError>;
    async fn get_many(&self, ids: Vec<I>) -> Result<Vec<T>, EndpointError>;
    async fn get_all(&self) -> Result<Vec<T>, EndpointError>;
}

pub struct Cache<T, I, Req> {
    client: Arc<Req>,
    _ids: Mutex<Vec<I>>,
    items: DashMap<I, T>,
    item_type: String,
}

impl<T, I, Req> Cache<T, I, Req>
where
    Req: Requester<false, false>,
    T: DeserializeOwned + Serialize + Clone + Send + Sync + EndpointWithId<IdType = I> + BulkEndpoint + 'static,
    I: Display + DeserializeOwned + Serialize + Hash + Clone + Send + Sync + Eq + Copy + 'static,
{
    pub fn new(client: Arc<Req>, item_type: &str) -> Self {
        Cache {
            client,
            _ids: Mutex::new(Vec::new()),
            items: DashMap::new(),
            item_type: item_type.to_string(),
        }
    }

    async fn _fetch_ids(&self) -> Result<Vec<I>, EndpointError> {
        Requester::ids::<T, I>(&*self.client).await
    }

    async fn fetch_many(&self, ids: Vec<I>) -> Result<Vec<T>, EndpointError> {
        Requester::many::<T, I>(&*self.client, ids).await
    }

    async fn fetch_single(&self, id: I) -> Result<T, EndpointError> {
        Requester::single::<T, I>(&*self.client, id).await
    }
}

#[async_trait]
impl<T, I, Req> Resolver<T, I> for Cache<T, I, Req>
where
    Req: Requester<false, false> + Send + Sync,
    T: DeserializeOwned + Serialize + Clone + Send + Sync + EndpointWithId<IdType = I> + BulkEndpoint + 'static,
    I: Display + DeserializeOwned + Serialize + Hash + Clone + Send + Sync + Eq + Copy + 'static,
{
    fn clear(&self) {
        self.items.clear()
    }

    async fn ensure(&self, ids: Vec<I>) -> Result<(), EndpointError> {
        log::info!("Retrieving {} data", self.item_type);
        let ids: Vec<_> = ids.into_iter().filter(|id| !self.items.contains_key(id)).collect();
        log::info!("Retrieving {} missing {}s from GW2 API", ids.len(), self.item_type);
        let items = self.fetch_many(ids.clone()).await?;
        for (id, item) in ids.into_iter().zip(items) {
            self.items.insert(id, item);
        }
        Ok(())
    }

    async fn get(&self, id: I) -> Result<T, EndpointError> {
        if !self.items.contains_key(&id) {
            self.items.insert(id, self.fetch_single(id).await?);
        }
        Ok(self.items.get(&id).unwrap().clone())
    }

    async fn get_many(&self, ids: Vec<I>) -> Result<Vec<T>, EndpointError> {
        self.ensure(ids.clone()).await?;
        let items = ids.iter().filter_map(|id| self.items.get(id).map(|guard| guard.clone()));
        Ok(items.collect())
    }

    async fn get_all(&self) -> Result<Vec<T>, EndpointError> {
        let mut ids = self._ids.lock().await;
        if ids.is_empty() {
            let new_ids = self._fetch_ids().await?;
            *ids = new_ids;
        }
        self.get_many(ids.clone()).await
    }
}
