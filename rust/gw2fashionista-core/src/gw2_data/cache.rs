use std::fmt::Display;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

use gw2lib::model::{BulkEndpoint, EndpointWithId};
use gw2lib::{Requester, EndpointError};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct Cache<T, I, Req> {
    client: Arc<Req>,
    _ids: Vec<I>,
    items: HashMap<I, T>,
}

impl<T: Clone, I: Clone + Hash + Eq + Copy, Req: Requester<false, false>> Cache<T, I, Req>
where
    Req: Requester<false, false>,
    T: DeserializeOwned + Serialize + Clone + Send + Sync + EndpointWithId<IdType = I> + BulkEndpoint + 'static,
    I: Display + DeserializeOwned + Serialize + Hash + Clone + Send + Sync + Eq + 'static,
{
    pub fn new(client: Arc<Req>) -> Self {
        Cache {
            client,
            _ids: Vec::new(),
            items: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.items.drain();
    }

    pub fn ensure<Ids: IntoIterator<Item=I>>(&mut self, ids: Ids) -> Result<(), EndpointError> {
        let ids: Vec<_> = ids.into_iter().filter(|id| !self.items.contains_key(id)).collect();
        let items = self.fetch_many(ids.clone())?;
        let items = ids.into_iter().zip(items);
        self.items.extend(items);
        Ok(())
    }

    pub fn get(&mut self, id: I) -> Result<T, EndpointError> {
        if !self.items.contains_key(&id) {
            self.items.insert(id, self.fetch_single(id)?);
        }
        Ok(self.items.get(&id).unwrap().clone())
    }

    pub fn _get_many(&mut self, ids: Vec<I>) -> Result<Vec<T>, EndpointError> {
        self.ensure(ids.clone())?;
        let items = ids.iter().filter_map(|id| self.items.get(id).cloned());
        Ok(items.collect())
    }

    pub fn _get_all(&mut self) -> Result<Vec<T>, EndpointError> {
        if self._ids.is_empty() {
            self._ids = self._fetch_ids()?;
        }
        self._get_many(self._ids.clone())
    }

    fn _fetch_ids(&self) -> Result<Vec<I>, EndpointError> {
        Requester::ids::<T, I>(&*self.client)
    }

    fn fetch_many(&self, ids: Vec<I>) -> Result<Vec<T>, EndpointError> {
        Requester::many::<T, I>(&*self.client, ids)
    }

    fn fetch_single(&self, id: I) -> Result<T, EndpointError> {
        Requester::single::<T, I>(&*self.client, id)
    }
}
