use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io::Cursor;

use byteorder::{LittleEndian, WriteBytesExt};
use strum::{EnumCount, IntoEnumIterator};

use crate::domain::error::ChatLinkError;
use crate::domain::skins::{DyeId, SkinId, Slot};
use slot::{SlotFilter, SlotType, Visibility};

const TEMPLATE_PAYLOAD_SIZE: usize = 96;

pub mod slot;

#[derive(Clone, PartialEq, Eq)]
pub struct WardrobeTemplate {
    slots: [Slot; SlotType::COUNT],
}

impl WardrobeTemplate {
    pub fn new(slots: HashMap<SlotType, Slot>) -> Self {
        let slots_vec = SlotType::iter()
            .map(|slot_type| match slots.get(&slot_type) {
                Some(slot) => *slot,
                None => Slot::empty(slot_type.dyable()),
            })
            .collect();
        Self::from_vector(slots_vec)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        if bytes.len() != TEMPLATE_PAYLOAD_SIZE {
            return Err(ChatLinkError::TruncatedData(bytes.to_vec()));
        }

        let visibility = Visibility::from_bytes(bytes)?;
        let mut cursor = Cursor::new(bytes);
        let slots: Result<Vec<_>, _> = SlotType::iter()
            .map(|slot_type| {
                Slot::read(
                    &mut cursor,
                    slot_type.dyable(),
                    visibility.contains(slot_type.visibility()),
                )
            })
            .collect();

        Ok(Self::from_vector(slots?))
    }

    fn from_vector(slots: Vec<Slot>) -> Self {
        WardrobeTemplate {
            slots: slots
                .try_into()
                .expect("iterator produced exactly SlotType::COUNT items"),
        }
    }

    pub fn get_slot(&self, slot_type: &SlotType) -> &Slot {
        &self.slots[slot_type.index()]
    }

    pub fn iter(&self) -> impl Iterator<Item = (SlotType, &Slot)> {
        SlotType::iter().map(|slot_type| (slot_type, self.get_slot(&slot_type)))
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

    pub fn as_map(&self, include_empty: bool) -> HashMap<SlotType, Slot> {
        let mut slots = HashMap::with_capacity(SlotType::COUNT);
        for (slot_type, slot) in self {
            if include_empty || !slot.is_empty() {
                slots.insert(slot_type, *slot);
            }
        }
        slots
    }

    pub fn filter(&self, filter: &SlotFilter) -> Self {
        let mut filtered = self.as_map(true);
        filtered.retain(|slot_type, _| filter.contains(slot_type));
        Self::new(filtered)
    }

    pub fn merge(&self, other: &Self, ignore_skin: bool, ignore_dies: bool) -> Self {
        let mut slots = self.as_map(false);
        for slot_type in SlotType::iter() {
            let merged = self.get_slot(&slot_type).merge(
                other.get_slot(&slot_type),
                ignore_skin,
                ignore_dies,
            );
            slots.insert(slot_type, merged);
        }
        Self::new(slots)
    }

    pub fn all_skin_ids(&self) -> HashSet<SkinId> {
        HashSet::from_iter(self.iter().filter_map(|(slot_type, slot)| match slot_type {
            SlotType::Outfit => None,
            _ => Some(slot.skin()).filter(|skin| !skin.is_empty()),
        }))
    }

    pub fn all_dye_ids(&self) -> HashSet<DyeId> {
        let dyes = self
            .iter()
            .filter_map(|(_, slot)| slot.dyes())
            .flat_map(|dyes| dyes.into_iter());
        HashSet::from_iter(dyes)
    }
}

impl<'a> IntoIterator for &'a WardrobeTemplate {
    type Item = (SlotType, &'a Slot);
    type IntoIter = Box<dyn Iterator<Item = (SlotType, &'a Slot)> + 'a>;

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

impl fmt::Debug for WardrobeTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(SlotType::iter().map(|slot| (slot, self.get_slot(&slot))))
            .finish()
    }
}
