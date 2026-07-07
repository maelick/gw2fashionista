use gw2lib::{ApiError, EndpointError};
use hyper::StatusCode;
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
#[strum(serialize_all = "lowercase")]
pub enum ErrorKind {
    NotFound,
    Transient,
    Permanent,
}

impl Error {
    pub(crate) fn from_gw2lib(endpoint: &'static str, request: String, err: EndpointError) -> Self {
        Error {
            kind: classify(&err),
            endpoint,
            request,
            source: Some(Box::new(err)),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn is_not_found(&self) -> bool {
        self.kind == ErrorKind::NotFound
    }

    pub fn is_transient(&self) -> bool {
        self.kind == ErrorKind::Transient
    }
}

fn classify(err: &EndpointError) -> ErrorKind {
    use ApiError as A;
    use EndpointError as E;
    match err {
        E::ApiError(A::Other(status, _)) => classify_status(*status),
        E::ApiError(A::RateLimited)
        | E::RateLimiterCrashed(_)
        | E::RateLimiterBucketExceeded
        | E::RequestFailed(_) => ErrorKind::Transient,
        _ => ErrorKind::Permanent,
    }
}

fn classify_status(status: StatusCode) -> ErrorKind {
    if status == StatusCode::NOT_FOUND {
        ErrorKind::NotFound
    } else if status.is_server_error() {
        ErrorKind::Transient
    } else {
        ErrorKind::Permanent
    }
}
