use std::fmt::Debug;
use std::hash::Hash;

use dashmap::DashMap;

use crate::gw2::error::Error;
use crate::gw2::fetch::Fetch;
use crate::gw2::named::Named;

pub struct Cache<T, I> {
    client: Box<dyn Fetch<T, I> + Send + Sync + 'static>,
    items: DashMap<I, T>,
}

impl<T, I> Cache<T, I>
where
    T: Clone + Send + Sync + 'static,
    I: Debug + Hash + Eq + Clone + Copy + Send + Sync + 'static,
{
    pub fn new(client: Box<dyn Fetch<T, I> + Send + Sync + 'static>) -> Self {
        Cache {
            client,
            items: DashMap::new(),
        }
    }

    pub fn clear(&self) {
        self.items.clear()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(endpoint = self.client.endpoint_name())))]
    pub async fn ensure<J: Into<I>>(&self, ids: impl IntoIterator<Item = J>) -> Result<(), Error> {
        let ids: Vec<_> = ids
            .into_iter()
            .map(|id| id.into())
            .filter(|id| !self.items.contains_key(id))
            .collect();

        if !ids.is_empty() {
            #[cfg(feature = "tracing")]
            tracing::info!(message = "Retrieving missing data from GW2 API", ?ids);

            let items = self.client.many(&ids).await?;
            for (id, item) in items {
                self.items.insert(id, item);
            }
        }
        Ok(())
    }

    pub async fn get(&self, id: I) -> Result<T, Error> {
        if !self.items.contains_key(&id) {
            self.items.insert(id, self.client.single(id).await?);
        }
        Ok(self.items.get(&id).unwrap().clone())
    }
}

impl<T, I> Cache<T, I>
where
    T: Named + Clone + Send + Sync + 'static,
    I: Debug + Hash + Eq + Clone + Copy + Send + Sync + 'static,
{
    pub async fn resolve_name(&self, id: I) -> Result<Option<String>, Error> {
        match self.get(id).await {
            Ok(object) => Ok(Some(object.name().to_string())),
            Err(err) if err.is_not_found() => {
                #[cfg(feature = "tracing")]
                tracing::warn!(
                    message = "could not resolve object",
                    endpoint = self.client.endpoint_name(),
                    id = ?id
                );
                Ok(None)
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;
    use std::collections::HashMap;

    use super::*;
    use crate::gw2::{error::ErrorKind, fetch::MockFetch};

    const ITEM_ID: u32 = 42;

    #[tokio::test]
    async fn test_no_retry_on_cached() {
        let mut mock = MockFetch::new();
        mock_single(&mut mock, ITEM_ID);

        let cache = Cache::new(Box::new(mock));
        let result = cache.get(ITEM_ID).await.unwrap();
        assert_eq!(result, "Item 42");

        let result = cache.get(ITEM_ID).await.unwrap();
        assert_eq!(result, "Item 42");
    }

    #[tokio::test]
    async fn test_ensure() {
        let mut mock = MockFetch::new();
        mock_many(
            &mut mock,
            vec![ITEM_ID],
            HashMap::from([(ITEM_ID, "Item 42".to_string())]),
        );

        let cache = Cache::new(Box::new(mock));

        cache.ensure(vec![ITEM_ID]).await.unwrap();

        let result = cache.get(ITEM_ID).await.unwrap();
        assert_eq!(result, "Item 42");
    }

    #[tokio::test]
    async fn test_ensure_already_cached() {
        let mut mock = MockFetch::new();
        mock_single(&mut mock, ITEM_ID);

        let cache = Cache::new(Box::new(mock));
        let result = cache.get(ITEM_ID).await.unwrap();
        assert_eq!(result, "Item 42");

        cache.ensure(vec![ITEM_ID]).await.unwrap();

        let result = cache.get(ITEM_ID).await.unwrap();
        assert_eq!(result, "Item 42");
    }

    #[tokio::test]
    async fn test_ensure_many_with_missing() {
        let mut mock = MockFetch::new();
        mock_many(
            &mut mock,
            vec![1, ITEM_ID, 101010],
            HashMap::from([(ITEM_ID, "Item 42".to_string())]),
        );
        mock.expect_single()
            .with(predicate::eq(101010))
            .times(1)
            .returning(|_| Err(ErrorKind::NotFound.into()));

        let cache = Cache::new(Box::new(mock));
        cache.ensure(vec![1, ITEM_ID, 101010]).await.unwrap();

        let result = cache.get(ITEM_ID).await.unwrap();
        assert_eq!(result, "Item 42");

        let result = cache.get(101010).await.unwrap_err();
        assert!(result.is_not_found());
    }

    fn mock_single(mock: &mut MockFetch<String, u32>, id: u32) {
        mock.expect_single()
            .with(predicate::eq(id))
            .times(1)
            .returning(move |_| Ok(format!("Item {}", id)));
    }

    fn mock_many(mock: &mut MockFetch<String, u32>, ids: Vec<u32>, result: HashMap<u32, String>) {
        mock.expect_many()
            .with(predicate::eq(ids))
            .times(1)
            .returning(move |_| Ok(result.clone()));
    }
}
