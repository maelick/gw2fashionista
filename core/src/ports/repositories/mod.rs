use crate::filters::StringFilters;
use crate::{fashion::Fashion, tag::Tag};
use async_trait::async_trait;
use gw2fashionista_chatlink::ChatLinkError;
use uuid;

#[async_trait]
pub trait FashionRepository {
    async fn insert_fashion(&self, fashion: &Fashion) -> Result<Fashion>;

    async fn update_fashion(&self, fashion: &Fashion) -> Result<Fashion>;

    async fn get_fashion_by_id(&self, id: &uuid::Uuid) -> Result<Fashion>;

    async fn get_fashion_by_name(&self, name: &str, character: Option<&str>) -> Result<Fashion>;

    async fn list_fashions(&self) -> Result<Vec<Fashion>>;

    async fn upsert_tag(&self, name: &str) -> Result<Option<Tag>>;

    async fn ensure_tag(&self, name: &str) -> Result<Tag>;

    async fn rename_tag(&self, from: &str, to: &str) -> Result<Tag>;

    async fn get_tag_by_id(&self, id: &uuid::Uuid) -> Result<Tag>;

    async fn get_tag_by_name(&self, name: &str) -> Result<Tag>;

    async fn list_tags(&self, filters: StringFilters) -> Result<Vec<Tag>>;

    async fn replace_tags(
        &self,
        tags: impl IntoIterator<Item: Into<String>, IntoIter: Send> + Send,
        with: &str,
    ) -> Result<()>;

    async fn clean_tags(&self) -> Result<()>;

    async fn get_fashion_tags(&self, fashion_id: &uuid::Uuid) -> Result<Vec<String>>;

    async fn ensure_fashion_tags(
        &self,
        fashion_ids: impl IntoIterator<Item = &uuid::Uuid> + Send,
        tags: impl IntoIterator<Item: Into<String>, IntoIter: Send> + Send,
    ) -> Result<()>;

    async fn remove_fashion_tags(
        &self,
        fashion_ids: impl IntoIterator<Item = &uuid::Uuid> + Send,
        tags: impl IntoIterator<Item: Into<String>, IntoIter: Send> + Send,
    ) -> Result<()>;
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("not found")]
    NotFound,

    #[error("id is not a valid UUID")]
    InvalidId(#[from] uuid::Error),

    #[error("database constraint violation: {message}")]
    Conflict { message: String },

    #[error("stored chat link is not valid")]
    InvalidChatLink(#[from] ChatLinkError),

    #[error(transparent)]
    Backend(Box<dyn std::error::Error + Send + Sync>),
}
