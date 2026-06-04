use std::io::Cursor;
use std::collections::HashMap;

use strum::{EnumCount, IntoEnumIterator};

use super::error::ChatLinkError;
use super::skin_type::{SkinType, SkinVisibility};
use super::skins::{SkinId, Dyes};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WardrobeTemplate {
    slots: HashMap<SkinType, EquipmentSlot>
}

impl WardrobeTemplate {
    pub fn new(slots: HashMap<SkinType, EquipmentSlot>) -> Self {
        WardrobeTemplate { slots }
    }

    pub fn get_slot(&self, skin_type: SkinType) -> Option<&EquipmentSlot> {
        self.slots.get(&skin_type)
    }

    pub fn iter(&self) -> impl Iterator<Item = &EquipmentSlot> {
        self.slots.values()
    }
}

impl IntoIterator for WardrobeTemplate {
    type Item = (SkinType, EquipmentSlot);
    type IntoIter = std::collections::hash_map::IntoIter<SkinType, EquipmentSlot>;

    fn into_iter(self) -> Self::IntoIter {
        self.slots.into_iter()
    }
}

impl<'a> IntoIterator for &'a WardrobeTemplate {
    type Item = (&'a SkinType, &'a EquipmentSlot);
    type IntoIter = std::collections::hash_map::Iter<'a, SkinType, EquipmentSlot>;

    fn into_iter(self) -> Self::IntoIter {
        self.slots.iter()
    }
}

impl TryFrom<&[u8]> for WardrobeTemplate {
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

        Ok(WardrobeTemplate{slots})
    }
}

impl From<WardrobeTemplate> for Vec<u8> {
    fn from(_: WardrobeTemplate) -> Vec<u8> {
        return Vec::new() // TODO implement
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
    fn from_cursor(cursor: &mut Cursor<&[u8]>, skin_type: SkinType, visibility: SkinVisibility) -> Result<Self, std::io::Error> {
        let skin = SkinId::from_cursor(cursor)?;
        let visible =  visibility.contains(skin_type.visibility());
        if skin_type.dyable() {
            let dyes = Dyes::from_cursor(cursor)?;
            Ok(Self::Dyable { skin, visible, dyes })
        } else {
            Ok(Self::NonDyable { skin, visible })
        }
    }
}
