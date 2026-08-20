use gw2fashionista_core::domain::error::ChatLinkError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("fashion not found")]
    NotFound,

    #[error("database constraint violation")]
    Conflict(#[source] sqlx::Error),

    #[error("stored chat link is not valid")]
    InvalidChatLink(#[from] ChatLinkError),

    #[error(transparent)]
    Database(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        if matches!(&err, sqlx::Error::Database(db) if db.is_unique_violation()) {
            Self::Conflict(err)
        } else {
            Self::Database(err)
        }
    }
}
