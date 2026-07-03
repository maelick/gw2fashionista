use linearize::Linearize;
use serde::{Deserialize, Serialize};

use crate::domain::templates::FashionSlot;

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
