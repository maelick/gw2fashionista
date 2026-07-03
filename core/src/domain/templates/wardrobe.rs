use std::collections::HashSet;

use linearize::Linearize;
use serde::{Deserialize, Serialize};

use crate::domain::skins::SkinId;
use crate::domain::templates::{FashionSlot, Template};

pub type WardrobeTemplate = Template<WardrobeSlot>;

impl WardrobeTemplate {
    pub fn all_skin_ids(&self) -> HashSet<SkinId> {
        HashSet::from_iter(self.iter().filter_map(|(slot, appearance)| match slot {
            WardrobeSlot::Outfit => None,
            _ => Some(appearance.skin()).filter(|skin| !skin.is_empty()),
        }))
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
pub enum WardrobeSlot {
    Aquabreather,
    Backpack,
    Chest,
    Shoes,
    Gloves,
    Head,
    Legs,
    Shoulders,
    Outfit,
    WeaponAquaticA,
    WeaponAquaticB,
    WeaponA1,
    WeaponA2,
    WeaponB1,
    WeaponB2,
}

impl FashionSlot for WardrobeSlot {
    fn dyeable(self) -> bool {
        {
            matches!(
                self,
                WardrobeSlot::Backpack
                    | WardrobeSlot::Chest
                    | WardrobeSlot::Shoes
                    | WardrobeSlot::Gloves
                    | WardrobeSlot::Head
                    | WardrobeSlot::Legs
                    | WardrobeSlot::Shoulders
                    | WardrobeSlot::Outfit
            )
        }
    }

    fn always_visible(self) -> bool {
        matches!(
            self,
            WardrobeSlot::Chest | WardrobeSlot::Shoes | WardrobeSlot::Legs
        )
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
pub enum EquipmentCategory {
    Underwater,
    Armors,
    Weapons,
}

impl EquipmentCategory {
    pub const fn slots(&self) -> &'static [WardrobeSlot] {
        match self {
            EquipmentCategory::Underwater => &[
                WardrobeSlot::Aquabreather,
                WardrobeSlot::WeaponAquaticA,
                WardrobeSlot::WeaponAquaticB,
            ],
            EquipmentCategory::Armors => &[
                WardrobeSlot::Aquabreather,
                WardrobeSlot::Chest,
                WardrobeSlot::Shoes,
                WardrobeSlot::Gloves,
                WardrobeSlot::Head,
                WardrobeSlot::Legs,
                WardrobeSlot::Shoulders,
            ],
            EquipmentCategory::Weapons => &[
                WardrobeSlot::WeaponAquaticA,
                WardrobeSlot::WeaponAquaticB,
                WardrobeSlot::WeaponA1,
                WardrobeSlot::WeaponA2,
                WardrobeSlot::WeaponB1,
                WardrobeSlot::WeaponB2,
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
        assert_eq!(WardrobeTemplate::payload_size(), 96)
    }

    #[test]
    fn test_invalid_visibility() {
        let bytes = &[0xFF, 0xFF];
        let result = WardrobeTemplate::read_visibility(bytes);
        assert_matches!(result, Err(ChatLinkError::InvalidVisibility(_)))
    }
}
