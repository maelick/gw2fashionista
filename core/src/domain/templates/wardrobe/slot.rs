use std::io::Cursor;

use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt};
use linearize::Linearize;
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter};

use crate::domain::{error::ChatLinkError, templates::FashionSlot};

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    EnumIter,
    EnumCount,
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

impl WardrobeSlot {
    pub const fn visibility(self) -> WardrobeVisibility {
        match self {
            WardrobeSlot::Aquabreather => WardrobeVisibility::AQUABREATHER,
            WardrobeSlot::Backpack => WardrobeVisibility::BACKPACK,
            WardrobeSlot::Chest => WardrobeVisibility::CHEST,
            WardrobeSlot::Shoes => WardrobeVisibility::SHOES,
            WardrobeSlot::Gloves => WardrobeVisibility::GLOVES,
            WardrobeSlot::Head => WardrobeVisibility::HEAD,
            WardrobeSlot::Legs => WardrobeVisibility::LEGS,
            WardrobeSlot::Shoulders => WardrobeVisibility::SHOULDERS,
            WardrobeSlot::Outfit => WardrobeVisibility::OUTFIT,
            WardrobeSlot::WeaponAquaticA => WardrobeVisibility::WEAPON_AQUATIC_A,
            WardrobeSlot::WeaponAquaticB => WardrobeVisibility::WEAPON_AQUATIC_B,
            WardrobeSlot::WeaponA1 => WardrobeVisibility::WEAPON_A1,
            WardrobeSlot::WeaponA2 => WardrobeVisibility::WEAPON_A2,
            WardrobeSlot::WeaponB1 => WardrobeVisibility::WEAPON_B1,
            WardrobeSlot::WeaponB2 => WardrobeVisibility::WEAPON_B2,
        }
    }
}

impl FashionSlot for WardrobeSlot {
    fn dyable(self) -> bool {
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
    EnumIter,
    strum_macros::EnumString,
    strum_macros::Display,
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

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct WardrobeVisibility: u16 {
        const AQUABREATHER = 1 << 0;
        const BACKPACK = 1 << 1;
        const CHEST = 1 << 2;
        const SHOES = 1 << 3;
        const GLOVES = 1 << 4;
        const HEAD = 1 << 5;
        const LEGS = 1 << 6;
        const SHOULDERS = 1 << 7;
        const OUTFIT = 1 << 8;
        const WEAPON_AQUATIC_A = 1 << 9;
        const WEAPON_AQUATIC_B = 1 << 10;
        const WEAPON_A1 = 1 << 11;
        const WEAPON_A2 = 1 << 12;
        const WEAPON_B1 = 1 << 13;
        const WEAPON_B2 = 1 << 14;
    }
}

impl WardrobeVisibility {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        if bytes.len() < 2 {
            return Err(ChatLinkError::TruncatedData(bytes.to_vec()));
        }
        let visibility_offset = bytes.len() - 2;
        let mut cursor = Cursor::new(&bytes[visibility_offset..]);
        WardrobeVisibility::read(&mut cursor)
    }

    pub fn read(cursor: &mut Cursor<&[u8]>) -> Result<Self, ChatLinkError> {
        let visibility_bytes = cursor.read_u16::<LittleEndian>()?;
        WardrobeVisibility::from_bits(visibility_bytes)
            .ok_or(ChatLinkError::InvalidVisibility(visibility_bytes))
    }
}

impl TryFrom<&[u8]> for WardrobeVisibility {
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        Self::from_bytes(bytes)
    }
}
