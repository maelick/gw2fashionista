use super::error::ChatLinkError;
use super::link_type::ChatLinkType;
use super::fashion_template::FashionTemplate;

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
