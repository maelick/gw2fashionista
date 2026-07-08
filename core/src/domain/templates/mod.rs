use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
    io::Cursor,
};

use byteorder::{LittleEndian, WriteBytesExt};
use linearize::{Linearize, LinearizeExt, StaticMap, static_map};
use serde::Serialize;

use crate::domain::{
    error::ChatLinkError,
    skins::{Appearance, DyeId, SkinId},
};

pub mod travel;
pub mod wardrobe;

pub type SlotFilter<S> = HashSet<S>;

#[derive(Debug, Eq, PartialEq, Hash, Linearize)]
pub enum FashionSlotKind {
    Equipment,
    Outfit,
    Mount,
    Glider,
    Skiff,
    Doorway,
}

pub trait FashionSlot: Eq + Hash + Copy + Linearize + Serialize + fmt::Debug + 'static {
    fn dyeable(self) -> bool;
    fn always_visible(self) -> bool;

    fn visibility_bit(self) -> u16 {
        1 << self.linearize()
    }

    fn visibility_mask() -> u16 {
        const { assert!(Self::LENGTH <= 16) }
        ((1u32 << Self::LENGTH) - 1) as u16
    }

    fn is_visible(self, visibility: u16) -> bool {
        (visibility & self.visibility_bit()) != 0
    }

    fn kind(self) -> FashionSlotKind;
}

#[derive(Clone, PartialEq, Eq)]
pub struct Template<S: FashionSlot> {
    slots: StaticMap<S, Appearance>,
}

impl<S: FashionSlot> Template<S> {
    pub fn new(slots: HashMap<S, Appearance>) -> Self {
        Template {
            slots: StaticMap::from_fn(|slot| {
                slots
                    .get(&slot)
                    .copied()
                    .unwrap_or_else(|| Appearance::empty(slot.dyeable()))
            }),
        }
    }

    pub fn get_slot(&self, slot: &S) -> &Appearance {
        &self.slots[slot]
    }

    pub fn iter(&self) -> impl Iterator<Item = (S, &Appearance)> {
        self.slots.iter()
    }

    pub fn as_map(&self, include_empty: bool) -> HashMap<S, Appearance> {
        let mut slots = HashMap::with_capacity(S::LENGTH);
        for (slot, appearance) in self {
            if include_empty || !appearance.is_empty() {
                slots.insert(slot, *appearance);
            }
        }
        slots
    }

    pub fn filter(&self, filter: &SlotFilter<S>) -> Self {
        let mut filtered = self.as_map(true);
        filtered.retain(|slot, _| filter.contains(slot));
        Self::new(filtered)
    }

    pub fn merge(&self, other: &Self, ignore_skin: bool, ignore_dyes: bool) -> Self {
        let mut slots = self.as_map(false);
        for (slot, appearance) in self.iter() {
            let merged = appearance.merge(other.get_slot(&slot), ignore_skin, ignore_dyes);
            slots.insert(slot, merged);
        }
        Self::new(slots)
    }

    pub fn all_skin_ids(&self) -> HashMap<FashionSlotKind, HashSet<SkinId>> {
        let mut skins = HashMap::<FashionSlotKind, HashSet<SkinId>>::new();
        for (slot, appearance) in self {
            skins
                .entry(slot.kind())
                .or_default()
                .insert(appearance.skin());
        }
        skins
    }

    pub fn all_dye_ids(&self) -> HashSet<DyeId> {
        let dyes = self
            .iter()
            .filter_map(|(_, appearance)| appearance.dyes())
            .flat_map(|dyes| dyes.into_iter());
        HashSet::from_iter(dyes)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        if bytes.len() != Self::payload_size() {
            return Err(ChatLinkError::TruncatedData(bytes.to_vec()));
        }

        let visibility = Self::read_visibility(bytes)?;
        let mut cursor = Cursor::new(bytes);

        Ok(Self {
            slots: static_map! {
                slot => Appearance::read(
                    &mut cursor,
                    slot.dyeable(),
                    slot.is_visible(visibility),
                )?
            },
        })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::with_capacity(Self::payload_size());
        for (_, slot) in self {
            slot.serialize(&mut buffer)?;
        }
        buffer.write_u16::<LittleEndian>(self.visibility())?;
        Ok(buffer)
    }

    fn read_visibility(bytes: &[u8]) -> Result<u16, ChatLinkError> {
        let tail: &[u8; 2] = bytes.last_chunk().expect("caller validated payload size");
        let visibility = u16::from_le_bytes(*tail);
        if (visibility & !S::visibility_mask()) == 0 {
            Ok(visibility)
        } else {
            Err(ChatLinkError::InvalidVisibility(visibility))
        }
    }

    fn visibility(&self) -> u16 {
        self.iter()
            .filter(|(slot, a)| slot.always_visible() || a.is_visible())
            .fold(0, |acc, (slot, _)| acc | slot.visibility_bit())
    }

    fn payload_size() -> usize {
        S::variants()
            .map(|slot| Appearance::encoded_size(slot.dyeable()))
            .sum::<usize>()
            + 2
    }
}

impl<'a, S: FashionSlot> IntoIterator for &'a Template<S> {
    type Item = (S, &'a Appearance);
    type IntoIter = <&'a StaticMap<S, Appearance> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.slots.iter()
    }
}

impl<S: FashionSlot> TryFrom<&[u8]> for Template<S> {
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        Self::from_bytes(bytes)
    }
}

impl<S: FashionSlot> TryFrom<&Template<S>> for Vec<u8> {
    type Error = std::io::Error;

    fn try_from(template: &Template<S>) -> Result<Self, std::io::Error> {
        template.serialize()
    }
}

impl<S: FashionSlot> fmt::Debug for Template<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

pub trait SlotFilterExt<S>
where
    S: FashionSlot,
{
    fn all() -> Self;

    fn invert(&mut self);

    fn remove_all<I>(&mut self, slots: I)
    where
        I: IntoIterator,
        I::Item: Borrow<S>;

    fn retain_all<I>(&mut self, slots: I)
    where
        I: IntoIterator,
        I::Item: Borrow<S>;
}

impl<S> SlotFilterExt<S> for SlotFilter<S>
where
    S: FashionSlot,
{
    fn all() -> Self {
        SlotFilter::from_iter(S::variants())
    }

    fn invert(&mut self) {
        *self = Self::all().difference(self).copied().collect()
    }

    fn retain_all<I>(&mut self, slots: I)
    where
        I: IntoIterator,
        I::Item: Borrow<S>,
    {
        let slots = Self::from_iter(slots.into_iter().map(|s| *s.borrow()));
        self.retain(|s| slots.contains(s))
    }

    fn remove_all<I>(&mut self, slots: I)
    where
        I: IntoIterator,
        I::Item: Borrow<S>,
    {
        for s in slots {
            self.remove(s.borrow());
        }
    }
}
