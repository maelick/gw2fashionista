use regex::Regex;
use once_cell::sync::Lazy;
use base64::Engine;

use super::error::ChatLinkError;
use super::link_type::ChatLinkType;
use super::fashion_template::FashionTemplate;

const BASE64_RE: &str = r"[-A-Za-z0-9+/]*={0,3}";

static CHAT_LINK_REGEX: Lazy<Regex> = Lazy::new(|| {
    let pattern = format!(r"^\[?&?({})\]?$", BASE64_RE);
    Regex::new(&pattern).unwrap()
});

#[derive(Debug)]
pub enum ChatLink {
    Coin,
    Item,
    NPCText,
    MapLink,
    PvPGame,
    Skill,
    Trait,
    User,
    Recipe,
    Wardrobe,
    Outfit,
    WvWObjective,
    BuildTemplate,
    Achivement,
    WardrobeTemplate(FashionTemplate),
}

#[derive(Debug)]
pub struct SerializedChatLink {
    link_type: ChatLinkType,
    bytes: Vec<u8>,
}

impl TryFrom<&str> for ChatLink {
    type Error = ChatLinkError;

    fn try_from(raw_chat_link: &str) -> Result<Self, ChatLinkError> {
        let serialized = SerializedChatLink::try_from(raw_chat_link)?;
        serialized.try_into()
    }
}


impl TryFrom<SerializedChatLink> for ChatLink {
    type Error = ChatLinkError;

    fn try_from(serialized: SerializedChatLink) -> Result<Self, ChatLinkError> {
        match serialized.link_type {
            ChatLinkType::WardrobeTemplate => {
                let template = FashionTemplate::try_from(serialized.bytes)?;
                Ok(ChatLink::WardrobeTemplate(template))
            }
            _ => Err(ChatLinkError::UnsupportedType(serialized.link_type)),
        }
    }
}

impl SerializedChatLink {
    pub fn new(link_type: ChatLinkType, bytes: Vec<u8>) -> Self {
        return SerializedChatLink { link_type, bytes }
    }
}

impl TryFrom<ChatLink> for SerializedChatLink {
    type Error = ChatLinkError;

    fn try_from(chat_link: ChatLink) -> Result<Self, ChatLinkError> {
        match chat_link {
            ChatLink::WardrobeTemplate(template) => {
                let bytes = template.into();
                return Ok(SerializedChatLink::new(ChatLinkType::WardrobeTemplate, bytes))
            }
            _ => Err(ChatLinkError::NotImplemented),
        }
    }
}

impl TryFrom<Vec<u8>> for SerializedChatLink {
    type Error = ChatLinkError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, ChatLinkError> {
        let (header, payload) = bytes
            .split_first()
            .ok_or(ChatLinkError::EmptyPayload)?;
        let link_type = ChatLinkType::try_from(*header).map_err(|err| ChatLinkError::UnknownType(err.number))?;
        Ok(SerializedChatLink::new(link_type, payload.to_vec()))
    }
}

impl TryFrom<&str> for SerializedChatLink {
    type Error = ChatLinkError;

    fn try_from(raw_chat_link: &str) -> Result<Self, ChatLinkError> {
        let caps = CHAT_LINK_REGEX.captures(raw_chat_link).ok_or(ChatLinkError::InvalidString)?;
        let base64_str = caps.get(1).ok_or(ChatLinkError::InvalidString)?.as_str();
        let decoded = base64::engine::general_purpose::STANDARD.decode(base64_str);
        decoded.map_err(ChatLinkError::InvalidBase64)?.try_into()
    }
}
