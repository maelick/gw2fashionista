use std::fmt::Display;
use std::sync::LazyLock;

use regex::Regex;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::domain::error::ChatLinkError;
use crate::domain::templates::travel::TravelTemplate;
use crate::domain::templates::wardrobe::WardrobeTemplate;

const BASE64_RE: &str = r"[-A-Za-z0-9+/]*={0,3}";

static CHAT_LINK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = format!(r"^\[?&?({})\]?$", BASE64_RE);
    Regex::new(&pattern).unwrap()
});

#[derive(IntoPrimitive, TryFromPrimitive, Debug, Copy, Clone)]
#[num_enum(error_type(name = ChatLinkError, constructor = ChatLinkError::UnknownType))]
#[repr(u8)]
pub enum ChatLinkType {
    Coin = 0x01,
    Item = 0x02,
    NPCText = 0x03,
    MapLink = 0x04,
    PvPGame = 0x05,
    Skill = 0x06,
    Trait = 0x07,
    User = 0x08,
    Recipe = 0x09,
    Wardrobe = 0x0A,
    Outfit = 0x0B,
    WvWObjective = 0x0C,
    BuildTemplate = 0x0D,
    Achievement = 0x0E,
    WardrobeTemplate = 0x0F,
    TravelTemplate = 0x10,
}

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
    Achievement,
    WardrobeTemplate(WardrobeTemplate),
    TravelTemplate(TravelTemplate),
}

impl ChatLink {
    pub fn from_string(raw_chat_link: &str) -> Result<Self, ChatLinkError> {
        let serialized = SerializedChatLink::try_from(raw_chat_link)?;
        Self::from_serialized(&serialized)
    }

    pub fn from_serialized(serialized: &SerializedChatLink) -> Result<Self, ChatLinkError> {
        match serialized.link_type {
            ChatLinkType::WardrobeTemplate => {
                let template = WardrobeTemplate::try_from(serialized.bytes.as_slice())?;
                Ok(Self::WardrobeTemplate(template))
            }
            ChatLinkType::TravelTemplate => {
                let template = TravelTemplate::try_from(serialized.bytes.as_slice())?;
                Ok(Self::TravelTemplate(template))
            }
            _ => Err(ChatLinkError::UnsupportedType(serialized.link_type)),
        }
    }

    pub fn to_string(&self) -> Result<String, ChatLinkError> {
        let serialized = SerializedChatLink::from_chat_link(self)?;
        Ok(serialized.to_string())
    }

    pub fn link_type(&self) -> ChatLinkType {
        match self {
            ChatLink::Coin => ChatLinkType::Coin,
            ChatLink::Item => ChatLinkType::Item,
            ChatLink::NPCText => ChatLinkType::NPCText,
            ChatLink::MapLink => ChatLinkType::MapLink,
            ChatLink::PvPGame => ChatLinkType::PvPGame,
            ChatLink::Skill => ChatLinkType::Skill,
            ChatLink::Trait => ChatLinkType::Trait,
            ChatLink::User => ChatLinkType::User,
            ChatLink::Recipe => ChatLinkType::Recipe,
            ChatLink::Wardrobe => ChatLinkType::Wardrobe,
            ChatLink::Outfit => ChatLinkType::Outfit,
            ChatLink::WvWObjective => ChatLinkType::WvWObjective,
            ChatLink::BuildTemplate => ChatLinkType::BuildTemplate,
            ChatLink::Achievement => ChatLinkType::Achievement,
            ChatLink::WardrobeTemplate(_) => ChatLinkType::WardrobeTemplate,
            ChatLink::TravelTemplate(_) => ChatLinkType::TravelTemplate,
        }
    }
}

#[derive(Debug)]
pub struct SerializedChatLink {
    link_type: ChatLinkType,
    bytes: Vec<u8>,
}

impl SerializedChatLink {
    pub fn new(link_type: ChatLinkType, bytes: Vec<u8>) -> Self {
        SerializedChatLink { link_type, bytes }
    }

    pub fn from_chat_link(chat_link: &ChatLink) -> Result<Self, ChatLinkError> {
        match chat_link {
            ChatLink::WardrobeTemplate(template) => {
                let bytes = template.serialize()?;
                Ok(Self::new(ChatLinkType::WardrobeTemplate, bytes))
            }
            ChatLink::TravelTemplate(template) => {
                let bytes = template.serialize()?;
                Ok(Self::new(ChatLinkType::TravelTemplate, bytes))
            }
            _ => Err(ChatLinkError::NotImplemented),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        let (header, payload) = bytes.split_first().ok_or(ChatLinkError::EmptyPayload)?;
        let link_type = ChatLinkType::try_from(*header)?;
        Ok(Self::new(link_type, payload.to_vec()))
    }

    pub fn from_string(raw_chat_link: &str) -> Result<Self, ChatLinkError> {
        let caps = CHAT_LINK_REGEX
            .captures(raw_chat_link)
            .ok_or(ChatLinkError::InvalidString)?;
        let base64_str = caps.get(1).ok_or(ChatLinkError::InvalidString)?.as_str();
        let decoded = BASE64.decode(base64_str)?;
        Self::from_bytes(decoded.as_slice())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.bytes.len() + 1);
        bytes.push(self.link_type.into());
        bytes.extend_from_slice(&self.bytes);
        bytes
    }
}

impl Display for SerializedChatLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.to_bytes();
        let b64_encoded = BASE64.encode(bytes);
        write!(f, "[&{}]", b64_encoded)
    }
}

impl TryFrom<ChatLink> for WardrobeTemplate {
    type Error = ChatLinkError;

    fn try_from(link: ChatLink) -> Result<Self, Self::Error> {
        match link {
            ChatLink::WardrobeTemplate(template) => Ok(template),
            _ => Err(ChatLinkError::UnsupportedType(link.link_type())),
        }
    }
}

impl TryFrom<ChatLink> for TravelTemplate {
    type Error = ChatLinkError;

    fn try_from(link: ChatLink) -> Result<Self, Self::Error> {
        match link {
            ChatLink::TravelTemplate(template) => Ok(template),
            _ => Err(ChatLinkError::UnsupportedType(link.link_type())),
        }
    }
}

impl TryFrom<&str> for ChatLink {
    type Error = ChatLinkError;

    fn try_from(raw_chat_link: &str) -> Result<Self, ChatLinkError> {
        Self::from_string(raw_chat_link)
    }
}

impl TryFrom<&ChatLink> for String {
    type Error = ChatLinkError;

    fn try_from(chat_link: &ChatLink) -> Result<Self, ChatLinkError> {
        chat_link.to_string()
    }
}

impl TryFrom<SerializedChatLink> for ChatLink {
    type Error = ChatLinkError;

    fn try_from(serialized: SerializedChatLink) -> Result<Self, ChatLinkError> {
        Self::from_serialized(&serialized)
    }
}

impl TryFrom<&ChatLink> for SerializedChatLink {
    type Error = ChatLinkError;

    fn try_from(chat_link: &ChatLink) -> Result<Self, ChatLinkError> {
        Self::from_chat_link(chat_link)
    }
}

impl TryFrom<&[u8]> for SerializedChatLink {
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&str> for SerializedChatLink {
    type Error = ChatLinkError;

    fn try_from(raw_chat_link: &str) -> Result<Self, ChatLinkError> {
        Self::from_string(raw_chat_link)
    }
}

impl From<SerializedChatLink> for Vec<u8> {
    fn from(chat_link: SerializedChatLink) -> Self {
        chat_link.to_bytes()
    }
}

impl From<SerializedChatLink> for String {
    fn from(chat_link: SerializedChatLink) -> Self {
        chat_link.to_string()
    }
}
