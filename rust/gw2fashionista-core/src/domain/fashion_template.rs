use super::error::ChatLinkError;

#[derive(Debug)]
pub struct FashionTemplate {

}

impl TryFrom<Vec<u8>> for FashionTemplate {
    type Error = ChatLinkError;

    fn try_from(_: Vec<u8>) -> Result<Self, ChatLinkError> {
        return Err(ChatLinkError::NotImplemented)
    }
}

impl From<FashionTemplate> for Vec<u8> {
    fn from(_: FashionTemplate) -> Vec<u8> {
        return Vec::new()
    }
}
