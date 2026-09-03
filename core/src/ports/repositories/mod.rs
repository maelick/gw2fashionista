mod fashion;

pub use fashion::{
    Error as FashionError, Repository as FashionRepository, Result as FashionResult,
};

#[derive(Debug, thiserror::Error)]
pub enum Error<E> {
    #[error("not found")]
    NotFound,

    #[error(transparent)]
    Backend(Box<dyn std::error::Error + Send + Sync>),

    #[error(transparent)]
    Repository(#[from] E),
}
