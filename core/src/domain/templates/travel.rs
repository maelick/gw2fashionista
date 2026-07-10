use linearize::Linearize;
use serde::{Deserialize, Serialize};

use crate::domain::templates::{FashionSlot, Template};

pub type TravelTemplate = Template<TravelSlot>;

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
#[serde(rename_all = "snake_case")]
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

    fn kind(self) -> super::FashionSlotKind {
        match self {
            TravelSlot::Glider => super::FashionSlotKind::Glider,
            TravelSlot::Doorway => super::FashionSlotKind::Doorway,
            TravelSlot::Skiff => super::FashionSlotKind::Skiff,
            TravelSlot::Jackal
            | TravelSlot::Griffon
            | TravelSlot::Springer
            | TravelSlot::Skimmer
            | TravelSlot::Raptor
            | TravelSlot::Beetle
            | TravelSlot::Warclaw
            | TravelSlot::Skyscale
            | TravelSlot::Turtle => super::FashionSlotKind::Mount,
        }
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    Serialize,
    Deserialize,
    strum_macros::EnumString,
    strum_macros::Display,
    Linearize,
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
    use crate::domain::error::ChatLinkError;
    use std::assert_matches;

    #[test]
    fn test_payload_size() {
        assert_eq!(TravelTemplate::payload_size(), 122)
    }

    #[test]
    fn test_invalid_visibility() {
        let bytes = &[0xFF, 0xFF];
        let result = TravelTemplate::read_visibility(bytes);
        assert_matches!(result, Err(ChatLinkError::InvalidVisibility(_)))
    }
}
