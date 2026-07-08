use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use async_trait::async_trait;

use crate::gw2::{cache::Cache, error::Error, named::Named};

#[async_trait]
pub trait Lookup<I> {
    async fn ensure(&self, ids: Vec<I>) -> Result<(), Error>;

    async fn resolve_name(&self, id: I) -> Result<Option<String>, Error>;

    fn clear(&self);
}

pub struct StaticLookup<T, I> {
    items: HashMap<I, T>,
}

impl<T, I: Hash + Eq> StaticLookup<T, I> {
    pub fn new(items: impl IntoIterator<Item = (I, T)>) -> Self {
        StaticLookup {
            items: items.into_iter().collect(),
        }
    }
}

#[async_trait]
impl<T, I> Lookup<I> for StaticLookup<T, I>
where
    T: Named + Clone + Send + Sync,
    I: Debug + Hash + Eq + Clone + Copy + Send + Sync,
{
    async fn ensure(&self, _ids: Vec<I>) -> Result<(), Error> {
        Ok(())
    }

    async fn resolve_name(&self, id: I) -> Result<Option<String>, Error> {
        Ok(self.items.get(&id).map(|obj| obj.name().to_string()))
    }

    fn clear(&self) {}
}

#[async_trait]
impl<T, I> Lookup<I> for Cache<T, I>
where
    T: Named + Clone + Send + Sync + 'static,
    I: Debug + Hash + Eq + Clone + Copy + Send + Sync + 'static,
{
    async fn ensure(&self, ids: Vec<I>) -> Result<(), Error> {
        Cache::ensure(self, ids).await
    }

    async fn resolve_name(&self, id: I) -> Result<Option<String>, Error> {
        Cache::resolve_name(self, id).await
    }

    fn clear(&self) {
        Cache::clear(self)
    }
}
