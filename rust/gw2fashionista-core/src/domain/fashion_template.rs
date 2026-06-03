use std::io::Cursor;
use std::collections::HashMap;

use byteorder::{LittleEndian, ReadBytesExt};
use strum::{EnumCount, IntoEnumIterator};

use super::error::ChatLinkError;
use super::skin_type::{SkinType, SkinVisibility};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FashionTemplate {
    slots: HashMap<SkinType, EquipmentSlot>
}

impl FashionTemplate {
    pub fn new(slots: HashMap<SkinType, EquipmentSlot>) -> Self {
        FashionTemplate { slots }
    }

    pub fn get_slot(&self, skin_type: SkinType) -> Option<&EquipmentSlot> {
        self.slots.get(&skin_type)
    }

    pub fn iter(&self) -> impl Iterator<Item = &EquipmentSlot> {
        self.slots.values()
    }
}

impl IntoIterator for FashionTemplate {
    type Item = EquipmentSlot;
    type IntoIter = std::collections::hash_map::IntoValues<SkinType, EquipmentSlot>;

    fn into_iter(self) -> Self::IntoIter {
        self.slots.into_values()
    }
}

impl<'a> IntoIterator for &'a FashionTemplate {
    type Item = &'a EquipmentSlot;
    type IntoIter = std::collections::hash_map::Values<'a, SkinType, EquipmentSlot>;

    fn into_iter(self) -> Self::IntoIter {
        self.slots.values()
    }
}

impl TryFrom<&[u8]> for FashionTemplate {
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        if bytes.len() != 96 {
            return Err(ChatLinkError::TruncatedData(bytes.to_vec()))
        }

        let visibility = SkinVisibility::try_from(bytes)?;
        let mut cursor = Cursor::new(bytes);
        let mut slots = HashMap::with_capacity(SkinType::COUNT);

        for skin_type in SkinType::iter() {
            let slot = EquipmentSlot::from_cursor(&mut cursor, skin_type, visibility)?;
            slots.insert(skin_type, slot);
        }

        Ok(FashionTemplate{slots})
    }
}

impl From<FashionTemplate> for Vec<u8> {
    fn from(_: FashionTemplate) -> Vec<u8> {
        return Vec::new() // TODO implement
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkinId(u16);

impl SkinId {
    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        Ok(SkinId(cursor.read_u16::<LittleEndian>()?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DyeId(u16);

impl DyeId {
    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        Ok(DyeId(cursor.read_u16::<LittleEndian>()?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dyes(DyeId, DyeId, DyeId, DyeId);

impl Dyes {
    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        Ok(Dyes(
            DyeId::from_cursor(cursor)?,
            DyeId::from_cursor(cursor)?,
            DyeId::from_cursor(cursor)?,
            DyeId::from_cursor(cursor)?,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Skin {
    skin_type: SkinType,
    skin: SkinId,
    visible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DyableSkin {
    skin: Skin,
    dyes: Dyes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquipmentSlot {
    Skin(Skin),
    DyableSkin(DyableSkin),
}

impl EquipmentSlot {
    fn from_cursor(cursor: &mut Cursor<&[u8]>, skin_type: SkinType, visibility: SkinVisibility) -> Result<Self, std::io::Error> {
        if skin_type.dyable() {
            Ok(DyableSkin::from_cursor(cursor, skin_type, visibility)?.into())
        } else {
            Ok(Skin::from_cursor(cursor, skin_type, visibility)?.into())
        }
    }
}

impl Skin {
    pub fn new(skin_type: SkinType, skin: SkinId, visible: bool) -> Self {
        Self { skin_type, skin, visible }
    }

    fn from_cursor(cursor: &mut Cursor<&[u8]>, skin_type: SkinType, visibility: SkinVisibility) -> Result<Self, std::io::Error> {
        let skin_id = SkinId::from_cursor(cursor)?;
        let visible =  visibility.contains(skin_type.visibility());
        Ok(Skin::new(skin_type, skin_id, visible))
    }
}

impl DyableSkin {
    pub fn new(skin: Skin, dyes: Dyes) -> Self {
        Self { skin, dyes }
    }

    fn from_cursor(cursor: &mut Cursor<&[u8]>, skin_type: SkinType, visibility: SkinVisibility) -> Result<Self, std::io::Error> {
        let skin = Skin::from_cursor(cursor, skin_type, visibility)?;
        let dyes = Dyes::from_cursor(cursor)?;
        Ok(DyableSkin::new(skin, dyes).into())
    }
}

impl From<Skin> for EquipmentSlot {
    fn from(skin: Skin) -> EquipmentSlot {
        return EquipmentSlot::Skin(skin)
    }
}

impl From<DyableSkin> for EquipmentSlot {
    fn from(skin: DyableSkin) -> EquipmentSlot {
        return EquipmentSlot::DyableSkin(skin)
    }
}
