use std::collections::HashSet;
use std::io::Cursor;

use strum::{EnumCount, EnumIter, IntoEnumIterator};
use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::domain::error::ChatLinkError;
use crate::domain::skins::{SkinId, Dyes};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, EnumCount)]
#[repr(u8)]
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
    pub const fn dyable(self) -> bool {
        {
            matches!(self,
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

    pub const fn always_visible(self) -> bool {
        {
            matches!(self,
                SlotType::Chest
                | SlotType::Shoes
                | SlotType::Legs
            )
        }
    }

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

    pub fn index(self) -> usize {
        self as usize
    }
}

pub type SlotFilter = HashSet<SlotType>;

pub trait SlotFilterExt {
    fn all() -> Self;

    fn invert(&mut self);
    fn remove_all<I: IntoIterator<Item = &'static SlotType>>(&mut self, slots: I);
    fn filter<I: IntoIterator<Item = &'static SlotType>>(&mut self, slots: I);
    fn no_weapons(&mut self);
    fn no_armors(&mut self);
    fn no_backpack(&mut self);
    fn no_outfit(&mut self);
    fn no_underwater(&mut self);
    fn only_underwater(&mut self);
}

const WEAPONS: &[SlotType] = &[
    SlotType::WeaponAquaticA,
    SlotType::WeaponAquaticB,
    SlotType::WeaponA1,
    SlotType::WeaponA2,
    SlotType::WeaponB1,
    SlotType::WeaponB2,
];
const ARMORS: &[SlotType] = &[
    SlotType::Aquabreather,
    SlotType::Chest,
    SlotType::Shoes,
    SlotType::Gloves,
    SlotType::Head,
    SlotType::Legs,
    SlotType::Shoulders,
];
const UNDERWATER: &[SlotType] = &[
    SlotType::Aquabreather,
    SlotType::WeaponAquaticA,
    SlotType::WeaponAquaticB,
];

impl SlotFilterExt for SlotFilter {
    fn all() -> Self {
        SlotFilter::from_iter(SlotType::iter())
    }

    fn invert(&mut self) {
        *self = Self::all().difference(self).map(|s| *s).collect()
    }

    fn filter<I: IntoIterator<Item = &'static SlotType>>(&mut self, slots: I) {
        let slots = Self::from_iter(slots.into_iter().copied());
        self.retain(|s| slots.contains(s))
    }

    fn remove_all<I: IntoIterator<Item = &'static SlotType>>(&mut self, slots: I) {
        for s in slots {
            self.remove(s);
        }
    }

    fn no_weapons(&mut self) {
        self.remove_all(WEAPONS)
    }

    fn no_armors(&mut self) {
        self.remove_all(ARMORS)
    }

    fn no_backpack(&mut self) {
        self.remove(&SlotType::Backpack);
    }

    fn no_outfit(&mut self) {
        self.remove(&SlotType::Outfit);
    }

    fn no_underwater(&mut self) {
        self.remove_all(UNDERWATER)
    }

    fn only_underwater(&mut self) {
        self.filter(UNDERWATER)
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
        Visibility::from_bits(visibility_bytes).ok_or(ChatLinkError::InvalidVisibility(visibility_bytes))
    }
}

impl TryFrom<&[u8]> for Visibility {
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        Self::from_bytes(bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquipmentSlot {
    NonDyable {
        skin: SkinId,
        visible: bool,
    },
    Dyable {
        skin: SkinId,
        visible: bool,
        dyes: Dyes,
    },
}

impl EquipmentSlot {
    pub fn empty(dyable: bool) -> Self {
        if dyable {
            Self::Dyable { skin: SkinId::default(), visible: true, dyes: Dyes::default() }
        } else {
            Self::NonDyable { skin: SkinId::default(), visible: true }
        }
    }

    pub fn skin(self) -> SkinId {
        match self {
            EquipmentSlot::NonDyable { skin, visible: _ } | EquipmentSlot::Dyable { skin, visible: _, dyes: _ } => {
                skin
            }
        }
    }

    pub fn is_visible(self) -> bool {
        match self {
            EquipmentSlot::NonDyable { skin: _, visible } | EquipmentSlot::Dyable { skin: _, visible, dyes: _ } => {
                visible
            }
        }
    }

    pub fn dyes(self) -> Option<Dyes> {
        match self {
            EquipmentSlot::Dyable { skin: _, visible: _, dyes } => Some(dyes),
            EquipmentSlot::NonDyable { skin: _, visible: _ } => None,
        }
    }

    pub fn is_empty(self) -> bool {
        match self {
            EquipmentSlot::NonDyable { skin, visible: _ } => skin.is_empty(),
            EquipmentSlot::Dyable { skin, visible: _, dyes } => skin.is_empty() && dyes.is_empty(),
        }
    }

    pub fn merge(&self, other: &EquipmentSlot, ignore_skin: bool, ignore_dies: bool) -> EquipmentSlot {
        if other.is_empty() || (ignore_skin && ignore_dies) {
            *self
        } else if ignore_skin {
            match other {
                EquipmentSlot::NonDyable { skin: _, visible: _ } => EquipmentSlot::NonDyable { skin: self.skin(), visible: self.is_visible() },
                EquipmentSlot::Dyable { skin: _, visible: _, dyes } => EquipmentSlot::Dyable { skin: self.skin(), visible: self.is_visible(), dyes: *dyes },
            }
        } else if ignore_dies {
            match self {
                EquipmentSlot::NonDyable { skin: _, visible: _ } => EquipmentSlot::NonDyable { skin: other.skin(), visible: other.is_visible() },
                EquipmentSlot::Dyable { skin: _, visible: _, dyes } => EquipmentSlot::Dyable { skin: other.skin(), visible: other.is_visible(), dyes: *dyes },
            }
        } else {
            *other
        }
    }

    pub fn read(cursor: &mut Cursor<&[u8]>, dyable: bool, visible: bool) -> Result<Self, std::io::Error> {
        let skin = SkinId::from_cursor(cursor)?;
        if dyable {
            let dyes = Dyes::from_cursor(cursor)?;
            Ok(Self::Dyable { skin, visible, dyes })
        } else {
            Ok(Self::NonDyable { skin, visible })
        }
    }

    pub fn serialize(&self, buffer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        match self {
            EquipmentSlot::NonDyable { skin, visible: _ } => {
                buffer.write_u16::<LittleEndian>((*skin).into())?;
            },
            EquipmentSlot::Dyable { skin, visible: _, dyes } => {
                let (dye1, dye2, dye3, dye4) = (*dyes).into();
                buffer.write_u16::<LittleEndian>((*skin).into())?;
                buffer.write_u16::<LittleEndian>(dye1)?;
                buffer.write_u16::<LittleEndian>(dye2)?;
                buffer.write_u16::<LittleEndian>(dye3)?;
                buffer.write_u16::<LittleEndian>(dye4)?;
            },
        }
        Ok(())
    }
}
