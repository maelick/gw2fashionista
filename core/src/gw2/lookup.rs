use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use async_trait::async_trait;

use crate::gw2::{cache::Cache, error::Error, named::Named};

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait Lookup<I>
where
    I: Send + Sync + 'static,
{
    async fn ensure(&self, ids: Vec<I>) -> Result<(), Error>;

    async fn resolve_name(&self, id: I) -> Result<Option<String>, Error>;

    fn clear(&self);

    fn or<L>(self, fallback: L) -> Fallback<Self, L>
    where
        Self: Sized,
        L: Lookup<I> + 'static,
    {
        Fallback::new(self, fallback)
    }
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
    T: Named + Clone + Send + Sync + 'static,
    I: Debug + Hash + Eq + Clone + Copy + Send + Sync + 'static,
{
    async fn ensure(&self, _ids: Vec<I>) -> Result<(), Error> {
        Ok(())
    }

    async fn resolve_name(&self, id: I) -> Result<Option<String>, Error> {
        Ok(self.items.get(&id).map(|obj| obj.name().to_string()))
    }

    fn clear(&self) {}
}

pub struct Fallback<L1, L2> {
    primary: L1,
    fallback: L2,
}

impl<L1, L2> Fallback<L1, L2> {
    pub fn new(primary: L1, fallback: L2) -> Self {
        Fallback { primary, fallback }
    }
}

#[async_trait]
impl<I, L1, L2> Lookup<I> for Fallback<L1, L2>
where
    I: Clone + Send + Sync + 'static,
    L1: Lookup<I> + Send + Sync,
    L2: Lookup<I> + Send + Sync,
{
    async fn ensure(&self, ids: Vec<I>) -> Result<(), Error> {
        self.primary.ensure(ids.clone()).await?;
        self.fallback.ensure(ids).await
    }

    async fn resolve_name(&self, id: I) -> Result<Option<String>, Error> {
        match self.primary.resolve_name(id.clone()).await? {
            Some(name) => Ok(Some(name)),
            None => self.fallback.resolve_name(id).await,
        }
    }

    fn clear(&self) {
        self.primary.clear();
        self.fallback.clear();
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate;

    use crate::gw2::{
        error::ErrorKind,
        lookup::{Lookup, MockLookup, StaticLookup},
        named::StaticName,
    };

    #[tokio::test]
    async fn test_fallback_without_errors() {
        let l1 = StaticLookup::new([(
            1,
            StaticName {
                id: 1,
                name: "Name 1 from primary".to_string(),
            },
        )]);

        let l2 = StaticLookup::new([
            (
                1,
                StaticName {
                    id: 1,
                    name: "Name 1 from fallback".to_string(),
                },
            ),
            (
                2,
                StaticName {
                    id: 2,
                    name: "Name 2 from fallback".to_string(),
                },
            ),
        ]);

        let lookup = l1.or(l2);
        assert_eq!(
            lookup.resolve_name(1).await.unwrap().unwrap(),
            "Name 1 from primary"
        );
        assert_eq!(
            lookup.resolve_name(2).await.unwrap().unwrap(),
            "Name 2 from fallback"
        );
        assert_eq!(lookup.resolve_name(3).await.unwrap(), None);
    }

    #[tokio::test]
    async fn test_fallback_with_errors() {
        let items = [(
            1,
            StaticName {
                id: 1,
                name: "Name 1".to_string(),
            },
        )];

        let lookup = StaticLookup::new(items.clone()).or(MockLookup::new());
        assert_eq!(lookup.resolve_name(1).await.unwrap().unwrap(), "Name 1");

        let mut mock = MockLookup::new();
        mock.expect_resolve_name()
            .with(predicate::eq(1))
            .returning(|_| Err(ErrorKind::Transient.into()));
        let lookup = Fallback::new(mock, StaticLookup::new(items));

        assert!(lookup.resolve_name(1).await.is_err())
    }
}
