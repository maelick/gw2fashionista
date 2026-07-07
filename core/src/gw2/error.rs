use gw2lib::{ApiError, EndpointError};
use strum_macros::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[error("GET {endpoint} {request}: {kind}")]
pub struct Error {
    kind: ErrorKind,
    endpoint: &'static str,
    request: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    NotFound,
    Transient,
    Permanent,
}

impl Error {
    pub(crate) fn from_gw2lib(endpoint: &'static str, request: String, err: EndpointError) -> Self {
        Error {
            kind: (&err).into(),
            endpoint,
            request,
            source: Some(Box::new(err)),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn is_not_found(&self) -> bool {
        matches!(self.kind, ErrorKind::NotFound)
    }

    pub fn is_transient(&self) -> bool {
        matches!(self.kind, ErrorKind::Transient)
    }
}

impl From<&EndpointError> for ErrorKind {
    fn from(err: &EndpointError) -> Self {
        // TOOD how to make this nicer?
        match err {
            EndpointError::ApiError(ApiError::Other(status, _)) if status.as_u16() == 404 => {
                ErrorKind::NotFound
            }
            EndpointError::RateLimiterCrashed(_)
            | EndpointError::RateLimiterBucketExceeded
            | EndpointError::RequestFailed(_)
            | EndpointError::ApiError(ApiError::RateLimited) => ErrorKind::Transient,
            EndpointError::ApiError(ApiError::Other(status, _)) if status.is_server_error() => {
                ErrorKind::Transient
            }
            _ => ErrorKind::Permanent,
        }
    }
}
