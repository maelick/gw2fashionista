use super::link_type::ChatLinkType;

#[derive(Debug, thiserror::Error)]
pub enum ChatLinkError {
    #[error("Unsupported chat link type: {0:?}")]
    UnsupportedType(ChatLinkType),

    #[error("Unknown or invalid chat link type header: {0:?}")]
    UnknownType(u8),

    #[error("String does not look like a chat link")]
    InvalidString,

    #[error("Invalid base64 string")]
    InvalidBase64(#[from] base64::DecodeError),

    #[error("Truncated data: {0:?}")]
    TruncatedData(Vec<u8>),

    #[error("Invalid payload: {0:?}")]
    InvalidPayload(#[from] std::io::Error),

    #[error("Invalid visibility bytes: {0:?}")]
    InvalidVisibility(u16),

    #[error("Empty payload")]
    EmptyPayload,

    #[error("Not implemented")]
    NotImplemented,
}
