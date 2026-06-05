use std::io::Cursor;

use strum::{EnumCount, EnumIter};
use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt};

use super::error::ChatLinkError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, EnumCount)]
pub enum SkinType {
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

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct SkinVisibility: u16 {
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

impl SkinType {
    pub const fn dyable(self) -> bool {
        {
            matches!(self,
                SkinType::Backpack
                | SkinType::Chest
                | SkinType::Shoes
                | SkinType::Gloves
                | SkinType::Head
                | SkinType::Legs
                | SkinType::Shoulders
                | SkinType::Outfit
            )
        }
    }

    pub const fn always_visible(self) -> bool {
        {
            matches!(self,
                SkinType::Chest
                | SkinType::Shoes
                | SkinType::Legs
            )
        }
    }

    pub const fn visibility(self) -> SkinVisibility {
        match self {
            SkinType::Aquabreather => SkinVisibility::AQUABREATHER,
            SkinType::Backpack => SkinVisibility::BACKPACK,
            SkinType::Chest => SkinVisibility::CHEST,
            SkinType::Shoes => SkinVisibility::SHOES,
            SkinType::Gloves => SkinVisibility::GLOVES,
            SkinType::Head => SkinVisibility::HEAD,
            SkinType::Legs => SkinVisibility::LEGS,
            SkinType::Shoulders => SkinVisibility::SHOULDERS,
            SkinType::Outfit => SkinVisibility::OUTFIT,
            SkinType::WeaponAquaticA => SkinVisibility::WEAPON_AQUATIC_A,
            SkinType::WeaponAquaticB => SkinVisibility::WEAPON_AQUATIC_B,
            SkinType::WeaponA1 => SkinVisibility::WEAPON_A1,
            SkinType::WeaponA2 => SkinVisibility::WEAPON_A2,
            SkinType::WeaponB1 => SkinVisibility::WEAPON_B1,
            SkinType::WeaponB2 => SkinVisibility::WEAPON_B2,
        }
    }
}

impl SkinVisibility {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        if bytes.len() < 2 {
            return Err(ChatLinkError::TruncatedData(bytes.to_vec()));
        }
        let visibility_offset = bytes.len() - 2;
        let mut cursor = Cursor::new(&bytes[visibility_offset..]);
        SkinVisibility::read(&mut cursor)
    }

    pub fn read(cursor: &mut Cursor<&[u8]>) -> Result<Self, ChatLinkError> {
        let visibility_bytes = cursor.read_u16::<LittleEndian>()?;
        SkinVisibility::from_bits(visibility_bytes).ok_or(ChatLinkError::InvalidVisibility(visibility_bytes))
    }
}

impl TryFrom<&[u8]> for SkinVisibility {
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        Self::from_bytes(bytes)
    }
}
