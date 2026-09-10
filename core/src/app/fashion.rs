use std::{iter, sync::Arc};

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
        let id = created.id.ok_or(Error::MissingFashionId)?;
        self.fashion_repo
            .ensure_fashion_tags(iter::once(&id), &fashion.tags)
            .await?;
        created.tags.extend_from_slice(&fashion.tags);
        Ok(created)
    }

    pub async fn list(&self) -> Result<Vec<Fashion>> {
        let fashions = self.fashion_repo.list_fashions().await?;
        Ok(fashions)
    }

    pub async fn get_by_name(&self, name: &str, character: Option<&str>) -> Result<Fashion> {
        let fashions = self
            .fashion_repo
            .get_fashion_by_name(name, character)
            .await?;
        Ok(fashions)
    }

    pub async fn get_by_id(&self, id: &uuid::Uuid) -> Result<Fashion> {
        let fashions = self.fashion_repo.get_fashion_by_id(id).await?;
        Ok(fashions)
    }
}
