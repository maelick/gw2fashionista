use std::io::Cursor;
use std::collections::HashMap;

use itertools::Itertools;
use byteorder::{LittleEndian, ReadBytesExt};
use strum::IntoEnumIterator;

use super::error::ChatLinkError;
use super::skin_type::{SkinType, SkinVisibility};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FashionTemplate {
    slots: HashMap<SkinType, EquipmentSlot>
}

impl TryFrom<&[u8]> for FashionTemplate {
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        if bytes.len() != 96 {
            return Err(ChatLinkError::TruncatedData(bytes.to_vec()))
        }

        let visibility = SkinVisibility::try_from(bytes)?;
        let mut cursor = Cursor::new(bytes);
        let slots = SkinType::iter()
            .map(|skin_type| get_equipment_slot(&mut cursor, skin_type, visibility))
            .try_collect()?;
        Ok(FashionTemplate{slots})
    }
}

fn get_equipment_slot(cursor: &mut Cursor<&[u8]>, skin_type: SkinType, visibility: SkinVisibility) -> Result<(SkinType, EquipmentSlot), std::io::Error> {
    let skin_id = SkinId::from_cursor(cursor)?;
    let visible =  visibility.contains(skin_type.visibility());
    let skin = Skin::new(skin_type, skin_id, visible);
    Ok((skin_type, if skin_type.dyable() {
        let dyes = Dyes::from_cursor(cursor)?;
        DyableSkin::new(skin, dyes).into()
    } else {
        skin.into()
    }))
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

impl Skin {
    pub fn new(skin_type: SkinType, skin: SkinId, visible: bool) -> Self {
        Self { skin_type, skin, visible }
    }
}

impl DyableSkin {
    pub fn new(skin: Skin, dyes: Dyes) -> Self {
        Self { skin, dyes }
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
