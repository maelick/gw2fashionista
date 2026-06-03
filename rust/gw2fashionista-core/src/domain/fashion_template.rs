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
    pub fn new(id: u16) -> Self {
        SkinId(id)
    }

    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        Ok(SkinId(cursor.read_u16::<LittleEndian>()?))
    }
}

impl From<u16> for SkinId {
    fn from(id: u16) -> Self {
        Self::new(id)
    }
}

impl From<SkinId> for u16 {
    fn from(SkinId(id): SkinId) -> u16 {
        id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DyeId(u16);

impl DyeId {
    pub fn new(id: u16) -> Self {
        DyeId(id)
    }

    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        Ok(DyeId(cursor.read_u16::<LittleEndian>()?))
    }
}

impl From<u16> for DyeId {
    fn from(id: u16) -> Self {
        Self::new(id)
    }
}

impl From<DyeId> for u16 {
    fn from(DyeId(id): DyeId) -> u16 {
        id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dyes(DyeId, DyeId, DyeId, DyeId);

impl Dyes {
    pub fn new(dye1: DyeId, dye2: DyeId, dye3: DyeId, dye4: DyeId) -> Self {
        Dyes(dye1, dye2, dye3, dye4)
    }

    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        Ok(Dyes(
            DyeId::from_cursor(cursor)?,
            DyeId::from_cursor(cursor)?,
            DyeId::from_cursor(cursor)?,
            DyeId::from_cursor(cursor)?,
        ))
    }
}

impl From<(u16, u16, u16, u16)> for Dyes {
    fn from((id1, id2, id3, id4): (u16, u16, u16, u16)) -> Self {
        Self::new(id1.into(), id2.into(), id3.into(), id4.into())
    }
}

impl From<(DyeId, DyeId, DyeId, DyeId)> for Dyes {
    fn from((dye1, dye2, dye3, dye4): (DyeId, DyeId, DyeId, DyeId)) -> Self {
        Self::new(dye1, dye2, dye3, dye4)
    }
}

impl From<Dyes> for (DyeId, DyeId, DyeId, DyeId) {
    fn from(Dyes(dye1, dye2, dye3, dye4): Dyes) -> (DyeId, DyeId, DyeId, DyeId) {
        (dye1, dye2, dye3, dye4)
    }
}

impl From<Dyes> for (u16, u16, u16, u16) {
    fn from(Dyes(dye1, dye2, dye3, dye4): Dyes) -> (u16, u16, u16, u16) {
        (dye1.into(), dye2.into(), dye3.into(), dye4.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Skin {
    pub skin_type: SkinType,
    pub skin: SkinId,
    pub visible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DyableSkin {
    pub skin: Skin,
    pub dyes: Dyes,
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
