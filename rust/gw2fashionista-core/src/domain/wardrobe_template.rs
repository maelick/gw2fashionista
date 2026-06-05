use std::io::Cursor;
use std::collections::HashMap;

use strum::{EnumCount, IntoEnumIterator};
use byteorder::{LittleEndian, WriteBytesExt};

use super::error::ChatLinkError;
use slot::{SlotType, Visibility};
use super::skins::{SkinId, Dyes};

const TEMPLATE_PAYLOAD_SIZE: usize = 96;

pub mod slot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WardrobeTemplate {
    slots: HashMap<SlotType, EquipmentSlot>
}

impl WardrobeTemplate {
    pub fn new(slots: HashMap<SlotType, EquipmentSlot>) -> Self {
        WardrobeTemplate { slots }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        if bytes.len() != TEMPLATE_PAYLOAD_SIZE {
            return Err(ChatLinkError::TruncatedData(bytes.to_vec()))
        }

        let visibility = Visibility::from_bytes(bytes)?;
        let mut cursor = Cursor::new(bytes);
        let mut slots = HashMap::with_capacity(SlotType::COUNT);

        for slot_type in SlotType::iter() {
            let slot = EquipmentSlot::read(&mut cursor, slot_type, visibility)?;
            slots.insert(slot_type, slot);
        }

        Ok(WardrobeTemplate{slots})
    }

    pub fn get_slot(&self, slot_type: SlotType) -> Option<&EquipmentSlot> {
        self.slots.get(&slot_type)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&SlotType, &EquipmentSlot)> {
        self.slots.iter()
    }

    fn visibility(&self) -> Visibility {
        self.iter()
            .filter(|(slot_type, slot)| slot_type.always_visible() || slot.is_visible())
            .map(|(slot_type, _)| slot_type.visibility())
            .fold(Visibility::empty(), |acc, v| acc | v)
    }

    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::with_capacity(TEMPLATE_PAYLOAD_SIZE);

        for (_, slot) in self {
            slot.serialize(&mut buffer)?;
        }

        let visibility = self.visibility();
        buffer.write_u16::<LittleEndian>(visibility.bits())?;
        Ok(buffer)
    }
}

impl IntoIterator for WardrobeTemplate {
    type Item = (SlotType, EquipmentSlot);
    type IntoIter = std::collections::hash_map::IntoIter<SlotType, EquipmentSlot>;

    fn into_iter(self) -> Self::IntoIter {
        self.slots.into_iter()
    }
}

impl<'a> IntoIterator for &'a WardrobeTemplate {
    type Item = (&'a SlotType, &'a EquipmentSlot);
    type IntoIter = std::collections::hash_map::Iter<'a, SlotType, EquipmentSlot>;

    fn into_iter(self) -> Self::IntoIter {
        self.slots.iter()
    }
}

impl TryFrom<&[u8]> for WardrobeTemplate {
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&WardrobeTemplate> for Vec<u8> {
    type Error = std::io::Error;

    fn try_from(template: &WardrobeTemplate) -> Result<Self, std::io::Error> {
        template.serialize()
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

    fn read(cursor: &mut Cursor<&[u8]>, slot_type: SlotType, visibility: Visibility) -> Result<Self, std::io::Error> {
        let skin = SkinId::from_cursor(cursor)?;
        let visible =  visibility.contains(slot_type.visibility());
        if slot_type.dyable() {
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
