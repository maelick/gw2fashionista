use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::io::Cursor;

use byteorder::{LittleEndian, WriteBytesExt};
use strum::{EnumCount, IntoEnumIterator};

use crate::domain::error::ChatLinkError;
use crate::domain::skins::{Appearance, DyeId, SkinId};
use crate::domain::templates::{FashionSlot, SlotFilter};
use slot::{WardrobeSlot, WardrobeVisibility};

const TEMPLATE_PAYLOAD_SIZE: usize = 96;

pub mod slot;

#[derive(Clone, PartialEq, Eq)]
pub struct WardrobeTemplate {
    slots: [Appearance; WardrobeSlot::COUNT],
}

impl WardrobeTemplate {
    pub fn new(slots: HashMap<WardrobeSlot, Appearance>) -> Self {
        let slots_vec = WardrobeSlot::iter()
            .map(|slot| match slots.get(&slot) {
                Some(slot) => *slot,
                None => Appearance::empty(slot.dyable()),
            })
            .collect();
        Self::from_vector(slots_vec)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        if bytes.len() != TEMPLATE_PAYLOAD_SIZE {
            return Err(ChatLinkError::TruncatedData(bytes.to_vec()));
        }

        let visibility = WardrobeVisibility::from_bytes(bytes)?;
        let mut cursor = Cursor::new(bytes);
        let slots: Result<Vec<_>, _> = WardrobeSlot::iter()
            .map(|slot| {
                Appearance::read(
                    &mut cursor,
                    slot.dyable(),
                    visibility.contains(slot.visibility()),
                )
            })
            .collect();

        Ok(Self::from_vector(slots?))
    }

    fn from_vector(slots: Vec<Appearance>) -> Self {
        WardrobeTemplate {
            slots: slots
                .try_into()
                .expect("iterator produced exactly SlotType::COUNT items"),
        }
    }

    pub fn get_slot(&self, slot: &WardrobeSlot) -> &Appearance {
        &self.slots[slot.index()]
    }

    pub fn iter(&self) -> impl Iterator<Item = (WardrobeSlot, &Appearance)> {
        WardrobeSlot::iter().map(|slot| (slot, self.get_slot(&slot)))
    }

    fn visibility(&self) -> WardrobeVisibility {
        self.iter()
            .filter(|(slot, appearance)| slot.always_visible() || appearance.is_visible())
            .map(|(slot, _)| slot.visibility())
            .fold(WardrobeVisibility::empty(), |acc, v| acc | v)
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

    pub fn as_map(&self, include_empty: bool) -> HashMap<WardrobeSlot, Appearance> {
        let mut slots = HashMap::with_capacity(WardrobeSlot::COUNT);
        for (slot, appearance) in self {
            if include_empty || !appearance.is_empty() {
                slots.insert(slot, *appearance);
            }
        }
        slots
    }

    pub fn filter(&self, filter: &SlotFilter<WardrobeSlot>) -> Self {
        let mut filtered = self.as_map(true);
        filtered.retain(|slot, _| filter.contains(slot));
        Self::new(filtered)
    }

    pub fn merge(&self, other: &Self, ignore_skin: bool, ignore_dies: bool) -> Self {
        let mut slots = self.as_map(false);
        for slot in WardrobeSlot::iter() {
            let merged = self.get_slot(&slot).merge(
                other.get_slot(&slot),
                ignore_skin,
                ignore_dies,
            );
            slots.insert(slot, merged);
        }
        Self::new(slots)
    }

    pub fn all_skin_ids(&self) -> HashSet<SkinId> {
        HashSet::from_iter(self.iter().filter_map(|(slot, appearance)| match slot {
            WardrobeSlot::Outfit => None,
            _ => Some(appearance.skin()).filter(|skin| !skin.is_empty()),
        }))
    }

    pub fn all_dye_ids(&self) -> HashSet<DyeId> {
        let dyes = self
            .iter()
            .filter_map(|(_, appearance)| appearance.dyes())
            .flat_map(|dyes| dyes.into_iter());
        HashSet::from_iter(dyes)
    }
}

impl<'a> IntoIterator for &'a WardrobeTemplate {
    type Item = (WardrobeSlot, &'a Appearance);
    type IntoIter = Box<dyn Iterator<Item = (WardrobeSlot, &'a Appearance)> + 'a>;

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
            .entries(WardrobeSlot::iter().map(|slot| (slot, self.get_slot(&slot))))
            .finish()
    }
}
