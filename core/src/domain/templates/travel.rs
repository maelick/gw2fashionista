use std::collections::HashSet;

use linearize::Linearize;
use serde::{Deserialize, Serialize};

use crate::domain::error::ChatLinkError;
use crate::domain::skins::SkinId;
use crate::domain::templates::{FashionSlot, Template};

pub type TravelTemplate = Template<TravelSlot>;

impl TravelTemplate {
    pub fn all_mount_ids(&self) -> HashSet<SkinId> {
        HashSet::from_iter(self.iter().filter_map(|(slot, appearance)| {
            if slot.is_mount() {
                None
            } else {
                Some(appearance.skin()).filter(|skin| !skin.is_empty())
            }
        }))
    }
}

impl TryFrom<&[u8]> for TravelTemplate {
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&TravelTemplate> for Vec<u8> {
    type Error = std::io::Error;

    fn try_from(template: &TravelTemplate) -> Result<Self, std::io::Error> {
        template.serialize()
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    strum_macros::EnumString,
    strum_macros::Display,
    Linearize,
)]
#[repr(u8)]
#[strum(serialize_all = "snake_case")]
pub enum TravelSlot {
    Glider,
    Doorway,
    Jackal,
    Griffon,
    Springer,
    Skimmer,
    Raptor,
    Beetle,
    Warclaw,
    Skyscale,
    Skiff,
    Turtle,
}

impl FashionSlot for TravelSlot {
    fn dyeable(self) -> bool {
        true
    }

    fn always_visible(self) -> bool {
        true
    }
}

impl TravelSlot {
    pub fn is_mount(self) -> bool {
        match self {
            TravelSlot::Jackal
            | TravelSlot::Griffon
            | TravelSlot::Springer
            | TravelSlot::Skimmer
            | TravelSlot::Raptor
            | TravelSlot::Beetle
            | TravelSlot::Warclaw
            | TravelSlot::Skyscale
            | TravelSlot::Turtle => true,
            _ => false,
        }
    }
}

#[derive(
    Debug, Copy, Clone, Serialize, Deserialize, strum_macros::EnumString, strum_macros::Display,
)]
#[strum(serialize_all = "snake_case")]
pub enum TravelCategory {
    Mount,
}

impl TravelCategory {
    pub const fn slots(&self) -> &'static [TravelSlot] {
        match self {
            TravelCategory::Mount => &[
                TravelSlot::Jackal,
                TravelSlot::Griffon,
                TravelSlot::Springer,
                TravelSlot::Skimmer,
                TravelSlot::Raptor,
                TravelSlot::Beetle,
                TravelSlot::Warclaw,
                TravelSlot::Skyscale,
                TravelSlot::Turtle,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload_size() {
        assert_eq!(TravelTemplate::payload_size(), 122)
    }
}
