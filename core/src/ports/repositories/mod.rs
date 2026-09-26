mod fashion;
mod secret;

pub use fashion::{
    Error as FashionError, Repository as FashionRepository, Result as FashionResult,
    ValidationError as FashionValidationError,
};
pub use secret::{Error as SecretError, Repository as SecretRepository, Result as SecretResult};

#[derive(Debug, thiserror::Error)]
pub enum Error<E> {
    #[error("not found")]
    NotFound,

    #[error(transparent)]
    Backend(Box<dyn std::error::Error + Send + Sync>),

    #[error(transparent)]
    Repository(#[from] E),
}
