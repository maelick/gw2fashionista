use std::{iter, sync::Arc};

use futures::TryStreamExt;
use futures::stream::FuturesOrdered;

use crate::{
    domain::fashion::Fashion,
    ports::repositories::{self, FashionError, FashionRepository},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The repository returned a fashion template without id")] // should not be possible
    MissingFashionId,
    #[error(transparent)]
    Repository(#[from] repositories::Error<FashionError>),
}

pub struct Service<R>
where
    R: FashionRepository,
{
    fashion_repo: Arc<R>,
}

impl<R> Service<R>
where
    R: FashionRepository,
{
    pub fn new(fashion_repo: Arc<R>) -> Self {
        Self { fashion_repo }
    }

    pub async fn create(&self, fashion: &Fashion) -> Result<Fashion> {
        let mut created = self.fashion_repo.insert_fashion(fashion).await?;
        let id = created.id.as_ref().ok_or(Error::MissingFashionId)?;
        self.fashion_repo
            .ensure_fashion_tags(iter::once(id), &fashion.tags)
            .await?;
        created.tags.extend_from_slice(&fashion.tags);
        Ok(created)
    }

    pub async fn list(&self) -> Result<Vec<Fashion>> {
        FuturesOrdered::from_iter(
            self.fashion_repo
                .list_fashions()
                .await?
                .into_iter()
                .map(async |f| self.retrieve_fashion_tags(f).await),
        )
        .try_collect()
        .await
    }

    pub async fn set(&self, fashion: &Fashion) -> Result<Fashion> {
        let fashion = self.resolve_id(fashion).await?;
        let mut updated = self.fashion_repo.update_fashion(&fashion).await?;
        let id = updated.id.as_ref().ok_or(Error::MissingFashionId)?;
        self.untag(iter::once(id)).await?;
        self.fashion_repo
            .ensure_fashion_tags(iter::once(id), &fashion.tags)
            .await?;
        updated.tags = fashion.tags.clone();
        Ok(updated)
    }

    pub async fn patch(&self, fashion: &Fashion) -> Result<Fashion> {
        let existing = if fashion.id.is_some() {
            self.get_by_id(fashion.id.as_ref().unwrap()).await?
        } else {
            self.get_by_name(&fashion.name, fashion.character.as_deref())
                .await?
        };
        let patched = existing.patch(fashion);

        let mut updated = self.fashion_repo.update_fashion(&patched).await?;
        let id = updated.id.as_ref().ok_or(Error::MissingFashionId)?;

        self.fashion_repo
            .ensure_fashion_tags(iter::once(id), &fashion.tags)
            .await?;
        updated.tags = patched.tags;
        Ok(updated)
    }

    pub async fn get_by_name(&self, name: &str, character: Option<&str>) -> Result<Fashion> {
        let fashions = self
            .fashion_repo
            .get_fashion_by_name(name, character)
            .await?;
        Ok(self.retrieve_fashion_tags(fashions).await?)
    }

    pub async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Fashion> {
        let fashions = self.fashion_repo.get_fashion_by_id(id).await?;
        Ok(self.retrieve_fashion_tags(fashions).await?)
    }

    pub async fn untag(
        &self,
        fashion_ids: impl IntoIterator<Item = &uuid::Uuid> + Send,
    ) -> Result<()> {
        self.fashion_repo
            .remove_all_fashion_tags(fashion_ids)
            .await?;
        Ok(())
    }

    async fn resolve_id(&self, fashion: &Fashion) -> Result<Fashion> {
        if fashion.id.is_none() {
            let id = self
                .get_by_name(&fashion.name, fashion.character.as_deref())
                .await?
                .id
                .ok_or(Error::MissingFashionId)?;
            Ok(fashion.clone().with_id(id))
        } else {
            Ok(fashion.clone())
        }
    }

    async fn retrieve_fashion_tags(&self, mut fashion: Fashion) -> Result<Fashion> {
        let fashion_id = fashion.id.as_ref().ok_or(Error::MissingFashionId)?;
        let tags = self.fashion_repo.get_fashion_tags(fashion_id).await?;
        fashion.tags = tags;
        Ok(fashion)
    }
}
