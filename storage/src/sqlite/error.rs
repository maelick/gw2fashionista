use gw2fashionista_core::ports::repositories::{self, FashionError, FashionValidationError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("not found")]
    NotFound,

    #[error(transparent)]
    Validation(#[from] FashionValidationError),

    #[error("database constraint violation")]
    Conflict(#[source] sqlx::Error),

    #[error(transparent)]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => Self::Conflict(err),
            sqlx::Error::RowNotFound => Self::NotFound,
            _ => Self::Database(err),
        }
    }
}

impl From<Error> for repositories::Error<FashionError> {
    fn from(err: Error) -> Self {
        match err {
            Error::NotFound => Self::NotFound,
            Error::Validation(error) => Self::Repository(FashionError::Validation(error)),
            Error::Conflict(error) => Self::Repository(FashionError::Conflict {
                message: error.to_string(),
            }),
            Error::Database(error) => Self::Backend(Box::new(error)),
        }
    }
}
