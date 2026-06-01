use super::link_type::ChatLinkType;

#[derive(Debug, thiserror::Error)]
pub enum ChatLinkError {
    #[error("Unsupported chat link type: {0:?}")]
    UnsupportedType(ChatLinkType),

    #[error("Unknown or invalid chat link type header: {0:?}")]
    UnknownType(u16),

    #[error("Invalid base64 string")]
    InvalidBase64(),

    #[error("Truncated data: {0:?}")]
    TruncatedData(Vec<u8>),

    #[error("Invalid payload: {0:?}")]
    InvalidPayload(Vec<u8>),

    #[error("Not implemented")]
    NotImplemented,
}
