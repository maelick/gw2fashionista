use std::io::Cursor;
use std::collections::HashMap;

use strum::{EnumCount, IntoEnumIterator};
use byteorder::{LittleEndian, WriteBytesExt};

use crate::domain::error::ChatLinkError;
use slot::{SlotType, Visibility, EquipmentSlot};

const TEMPLATE_PAYLOAD_SIZE: usize = 96;

pub mod slot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WardrobeTemplate {
    slots: [EquipmentSlot; SlotType::COUNT],
}

impl WardrobeTemplate {
    pub fn new(slots: HashMap<SlotType, EquipmentSlot>) -> Self {
        let slots_vec = SlotType::iter().map(|slot_type| {
            match slots.get(&slot_type) {
                Some(slot) => *slot,
                None => EquipmentSlot::empty(slot_type),
            }
        }).collect();
        Self::from_vector(slots_vec)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        if bytes.len() != TEMPLATE_PAYLOAD_SIZE {
            return Err(ChatLinkError::TruncatedData(bytes.to_vec()))
        }

        let visibility = Visibility::from_bytes(bytes)?;
        let mut cursor = Cursor::new(bytes);
        let slots: Result<Vec<_>, _> = SlotType::iter()
            .map(|slot_type| EquipmentSlot::read(&mut cursor, slot_type, visibility))
            .collect();

        Ok(Self::from_vector(slots?))
    }

    fn from_vector(slots: Vec<EquipmentSlot>) -> Self {
        WardrobeTemplate {
            slots: slots.try_into().expect("iterator produced exactly SlotType::COUNT items")
        }
    }

    pub fn get_slot(&self, slot_type: &SlotType) -> &EquipmentSlot {
        &self.slots[slot_type.index()]
    }

    pub fn iter(&self) -> impl Iterator<Item = (SlotType, &EquipmentSlot)> {
        SlotType::iter().map(|slot_type| {
            (slot_type, self.get_slot(&slot_type))
        })
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

impl<'a> IntoIterator for &'a WardrobeTemplate {
    type Item = (SlotType, &'a EquipmentSlot);
    type IntoIter = Box<dyn Iterator<Item = (SlotType, &'a EquipmentSlot)> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
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
