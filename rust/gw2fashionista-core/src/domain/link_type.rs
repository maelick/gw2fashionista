use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::domain::error::ChatLinkError;

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
    Achivement = 0x0E,
    WardrobeTemplate = 0x0F,
}
