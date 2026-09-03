use gw2fashionista_chatlink::ChatLinkError;
use gw2fashionista_core::ports::repositories;
use sqlx::types::uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("not found")]
    NotFound,

    #[error("id is not a valid UUID")]
    InvalidId(#[from] uuid::Error),

    #[error("database constraint violation")]
    Conflict(#[source] sqlx::Error),

    #[error("stored chat link is not valid")]
    InvalidChatLink(#[from] ChatLinkError),

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

impl From<Error> for repositories::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::NotFound => Self::NotFound,
            Error::InvalidId(error) => Self::InvalidId(error),
            Error::Conflict(error) => Self::Conflict {
                message: error.to_string(),
            },
            Error::InvalidChatLink(error) => Self::InvalidChatLink(error),
            Error::Database(error) => Self::Backend(Box::new(error)),
        }
    }
}
