use async_trait::async_trait;

pub type Result<T> = std::result::Result<T, super::Error<Error>>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid value: {message}")]
    Invalid { message: String },
}

#[async_trait]
pub trait Repository {
    async fn gw2_api_key(&self) -> Result<String>;
}
