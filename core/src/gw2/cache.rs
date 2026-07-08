use std::fmt::Debug;
use std::hash::Hash;
use tokio::sync::Mutex;

use dashmap::DashMap;

use crate::gw2::error::Error;
use crate::gw2::fetch::Fetch;

pub struct Cache<T, I> {
    client: Box<dyn Fetch<T, I> + Send + Sync + 'static>,
    ids: Mutex<Vec<I>>,
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
            ids: Mutex::new(Vec::new()),
            items: DashMap::new(),
        }
    }

    pub fn clear(&self) {
        self.items.clear()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, fields(endpoint = self.client.endpoint_name())))]
    pub async fn ensure(&self, ids: Vec<I>) -> Result<(), Error> {
        let ids: Vec<_> = ids
            .into_iter()
            .filter(|id| !self.items.contains_key(id))
            .collect();

        if !ids.is_empty() {
            #[cfg(feature = "tracing")]
            tracing::info!(message = "Retrieving missing data from GW2 API", ?ids);

            let items = self.client.many(&ids).await?;
            for (id, item) in ids.into_iter().zip(items) {
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

    pub async fn get_many(&self, ids: &[I]) -> Result<Vec<T>, Error> {
        self.ensure(ids.to_vec()).await?;
        let items = ids
            .iter()
            .filter_map(|id| self.items.get(id).map(|guard| guard.clone()));
        Ok(items.collect())
    }

    pub async fn get_all(&self) -> Result<Vec<T>, Error> {
        let mut ids = self.ids.lock().await;
        if ids.is_empty() {
            let new_ids = self.client.ids().await?;
            *ids = new_ids;
        }
        self.get_many(&ids).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gw2::fetch::MockFetch;
    use mockall::predicate;

    #[tokio::test]
    async fn test_no_retry_on_cached() {
        let mut mock = MockFetch::new();
        mock_single(&mut mock, 42);

        let cache = Cache::new(Box::new(mock));
        let result = cache.get(42).await.unwrap();
        assert_eq!(result, "Item 42");

        let result = cache.get(42).await.unwrap();
        assert_eq!(result, "Item 42");
    }

    #[tokio::test]
    async fn test_ensure() {
        let mut mock = MockFetch::new();
        mock_many(&mut mock, vec![42], vec!["Item 42".to_string()]);

        let cache = Cache::new(Box::new(mock));

        cache.ensure(vec![42]).await.unwrap();

        let result = cache.get(42).await.unwrap();
        assert_eq!(result, "Item 42");
    }

    #[tokio::test]
    async fn test_ensure_already_cached() {
        let mut mock = MockFetch::new();
        mock_single(&mut mock, 42);

        let cache = Cache::new(Box::new(mock));
        let result = cache.get(42).await.unwrap();
        assert_eq!(result, "Item 42");

        cache.ensure(vec![42]).await.unwrap();

        let result = cache.get(42).await.unwrap();
        assert_eq!(result, "Item 42");
    }

    #[tokio::test]
    async fn test_ensure_many_with_missing() {
        let mut mock = MockFetch::new();
        mock_many(&mut mock, vec![1, 42, 101010], vec!["Item 42".to_string()]);
        mock_single(&mut mock, 101010);

        let cache = Cache::new(Box::new(mock));
        cache.ensure(vec![1, 42, 101010]).await.unwrap();

        let result = cache.get(42).await.unwrap();
        assert_eq!(result, "Item 42");

        let result = cache.get(101010).await.unwrap();
        assert_eq!(result, "Item 101010");
    }

    fn mock_single(mock: &mut MockFetch<String, u32>, id: u32) {
        mock.expect_single()
            .with(predicate::eq(id))
            .times(1)
            .returning(move |_| Ok(format!("Item {}", id)));
    }

    fn mock_many(mock: &mut MockFetch<String, u32>, ids: Vec<u32>, result: Vec<String>) {
        mock.expect_many()
            .with(predicate::eq(ids))
            .times(1)
            .returning(move |_| Ok(result.clone()));
    }
}
