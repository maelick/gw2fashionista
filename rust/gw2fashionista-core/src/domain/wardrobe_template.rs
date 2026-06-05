use std::io::Cursor;
use std::collections::HashMap;

use strum::{EnumCount, IntoEnumIterator};
use byteorder::{LittleEndian, WriteBytesExt};

use super::error::ChatLinkError;
use super::skin_type::{SkinType, SkinVisibility};
use super::skins::{SkinId, Dyes};

const TEMPLATE_PAYLOAD_SIZE: usize = 96;

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

    pub fn iter(&self) -> impl Iterator<Item = (&SkinType, &EquipmentSlot)> {
        self.slots.iter()
    }

    fn visibility(&self) -> SkinVisibility {
        self.iter()
            .filter(|(skin_type, slot)| skin_type.always_visible() || slot.is_visible())
            .map(|(skin_type, _)| skin_type.visibility())
            .fold(SkinVisibility::empty(), |acc, v| acc | v)
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
        if bytes.len() != TEMPLATE_PAYLOAD_SIZE {
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

impl TryFrom<&WardrobeTemplate> for Vec<u8> {
    type Error = std::io::Error;

    fn try_from(template: &WardrobeTemplate) -> Result<Self, std::io::Error> {
        let mut buffer = Vec::with_capacity(TEMPLATE_PAYLOAD_SIZE);

        for (_, slot) in template {
            slot.serialize(&mut buffer)?;
        }

        let visibility = template.visibility();
        buffer.write_u16::<LittleEndian>(visibility.bits())?;
        Ok(buffer)
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
    pub fn is_visible(self) -> bool {
        match self {
            EquipmentSlot::NonDyable { skin: _, visible } | EquipmentSlot::Dyable { skin: _, visible, dyes: _ } => {
                visible
            }
        }
    }

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

    fn serialize(&self, buffer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
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
