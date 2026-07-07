use gw2lib::{ApiError, EndpointError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("resource not found")]
    NotFound,
    #[error("transient GW2 API failure")]
    Transient(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("GW2 API request failed")]
    Permanent(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl From<EndpointError> for Error {
    fn from(err: EndpointError) -> Self {
        // TOOD how to make this nicer?
        // TODO create (private or public + Error as struct?) ErrorKind instead when adding contextual info?
        match err {
            EndpointError::ApiError(ApiError::Other(status, _)) if status.as_u16() == 404 => {
                Error::NotFound
            }
            EndpointError::RateLimiterCrashed(_)
            | EndpointError::RateLimiterBucketExceeded
            | EndpointError::RequestFailed(_)
            | EndpointError::ApiError(ApiError::RateLimited) => Error::Transient(Box::new(err)),
            EndpointError::ApiError(ApiError::Other(status, _)) if status.is_server_error() => {
                Error::Transient(Box::new(err))
            }
            _ => Error::Permanent(Box::new(err)),
        }
    }
}
