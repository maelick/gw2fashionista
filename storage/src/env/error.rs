use std::env;

use gw2fashionista_core::ports::repositories::{self, SecretError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("not found")]
    NotFound,

    #[error("not valid unicode")]
    NotUnicode,

    #[error("empy value")]
    Empty,

    #[error(transparent)]
    Env(env::VarError),
}

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Self {
        match &err {
            env::VarError::NotPresent => Self::NotFound,
            env::VarError::NotUnicode(_) => Self::NotUnicode,
        }
    }
}

impl From<Error> for repositories::Error<SecretError> {
    fn from(err: Error) -> Self {
        match err {
            Error::NotFound => Self::NotFound,
            Error::NotUnicode => Self::Repository(SecretError::Invalid {
                message: "not unicode".to_string(),
            }),
            Error::Empty => Self::Repository(SecretError::Invalid {
                message: "empty".to_string(),
            }),
            Error::Env(error) => Self::Backend(Box::new(error)),
        }
    }
}
