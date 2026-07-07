use std::fmt::Debug;
use std::hash::Hash;
use tokio::sync::Mutex;

use async_trait::async_trait;
use dashmap::DashMap;

use crate::gw2::error::Error;
use crate::gw2::fetch::Fetch;

#[async_trait]
pub trait Resolver<T, I> {
    fn clear(&self);
    async fn ensure(&self, ids: Vec<I>) -> Result<(), Error>;
    async fn get(&self, id: I) -> Result<T, Error>;
    async fn get_many(&self, ids: Vec<I>) -> Result<Vec<T>, Error>;
    async fn get_all(&self) -> Result<Vec<T>, Error>;
}

pub struct Cache<T, I> {
    client: Box<dyn Fetch<T, I> + Send + Sync + 'static>,
    ids: Mutex<Vec<I>>,
    items: DashMap<I, T>,
}

impl<T, I> Cache<T, I>
where
    T: Clone + Send + Sync + 'static,
    I: Hash + Eq + Clone + Send + Sync + 'static,
{
    pub fn new(client: Box<dyn Fetch<T, I> + Send + Sync + 'static>) -> Self {
        Cache {
            client,
            ids: Mutex::new(Vec::new()),
            items: DashMap::new(),
        }
    }
}

#[async_trait]
impl<T, I> Resolver<T, I> for Cache<T, I>
where
    T: Clone + Send + Sync + 'static,
    I: Debug + Hash + Eq + Clone + Copy + Send + Sync + 'static,
{
    fn clear(&self) {
        self.items.clear()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(endpoint = self.client.endpoint_name())))]
    async fn ensure(&self, ids: Vec<I>) -> Result<(), Error> {
        let ids: Vec<_> = ids
            .into_iter()
            .filter(|id| !self.items.contains_key(id))
            .collect();

        if !ids.is_empty() {
            #[cfg(feature = "tracing")]
            tracing::info!(message = "Retrieving missing data from GW2 API", ?ids);

            let items = self.client.many(ids.clone()).await?;
            for (id, item) in ids.into_iter().zip(items) {
                self.items.insert(id, item);
            }
        }
        Ok(())
    }

    async fn get(&self, id: I) -> Result<T, Error> {
        if !self.items.contains_key(&id) {
            self.items.insert(id, self.client.single(id).await?);
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
        let mut ids = self.ids.lock().await;
        if ids.is_empty() {
            let new_ids = self.client.ids().await?;
            *ids = new_ids;
        }
        self.get_many(ids.clone()).await
    }
}
