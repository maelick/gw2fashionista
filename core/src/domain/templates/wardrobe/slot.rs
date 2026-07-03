use std::io::Cursor;

use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt};
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
)]
#[repr(u8)]
#[strum(serialize_all = "snake_case")]
pub enum SlotType {
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

impl SlotType {
    pub const fn visibility(self) -> Visibility {
        match self {
            SlotType::Aquabreather => Visibility::AQUABREATHER,
            SlotType::Backpack => Visibility::BACKPACK,
            SlotType::Chest => Visibility::CHEST,
            SlotType::Shoes => Visibility::SHOES,
            SlotType::Gloves => Visibility::GLOVES,
            SlotType::Head => Visibility::HEAD,
            SlotType::Legs => Visibility::LEGS,
            SlotType::Shoulders => Visibility::SHOULDERS,
            SlotType::Outfit => Visibility::OUTFIT,
            SlotType::WeaponAquaticA => Visibility::WEAPON_AQUATIC_A,
            SlotType::WeaponAquaticB => Visibility::WEAPON_AQUATIC_B,
            SlotType::WeaponA1 => Visibility::WEAPON_A1,
            SlotType::WeaponA2 => Visibility::WEAPON_A2,
            SlotType::WeaponB1 => Visibility::WEAPON_B1,
            SlotType::WeaponB2 => Visibility::WEAPON_B2,
        }
    }
}

impl FashionSlot for SlotType {
    fn dyable(self) -> bool {
        {
            matches!(
                self,
                SlotType::Backpack
                    | SlotType::Chest
                    | SlotType::Shoes
                    | SlotType::Gloves
                    | SlotType::Head
                    | SlotType::Legs
                    | SlotType::Shoulders
                    | SlotType::Outfit
            )
        }
    }

    fn always_visible(self) -> bool {
        matches!(self, SlotType::Chest | SlotType::Shoes | SlotType::Legs)
    }

    fn index(self) -> usize {
        self as usize
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
    pub const fn slots(&self) -> &'static [SlotType] {
        match self {
            EquipmentCategory::Underwater => &[
                SlotType::Aquabreather,
                SlotType::WeaponAquaticA,
                SlotType::WeaponAquaticB,
            ],
            EquipmentCategory::Armors => &[
                SlotType::Aquabreather,
                SlotType::Chest,
                SlotType::Shoes,
                SlotType::Gloves,
                SlotType::Head,
                SlotType::Legs,
                SlotType::Shoulders,
            ],
            EquipmentCategory::Weapons => &[
                SlotType::WeaponAquaticA,
                SlotType::WeaponAquaticB,
                SlotType::WeaponA1,
                SlotType::WeaponA2,
                SlotType::WeaponB1,
                SlotType::WeaponB2,
            ],
        }
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Visibility: u16 {
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

impl Visibility {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        if bytes.len() < 2 {
            return Err(ChatLinkError::TruncatedData(bytes.to_vec()));
        }
        let visibility_offset = bytes.len() - 2;
        let mut cursor = Cursor::new(&bytes[visibility_offset..]);
        Visibility::read(&mut cursor)
    }

    pub fn read(cursor: &mut Cursor<&[u8]>) -> Result<Self, ChatLinkError> {
        let visibility_bytes = cursor.read_u16::<LittleEndian>()?;
        Visibility::from_bits(visibility_bytes)
            .ok_or(ChatLinkError::InvalidVisibility(visibility_bytes))
    }
}

impl TryFrom<&[u8]> for Visibility {
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        Self::from_bytes(bytes)
    }
}
