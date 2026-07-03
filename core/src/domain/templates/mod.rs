use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
};

use linearize::{Linearize, StaticMap};
use strum::IntoEnumIterator;

use crate::domain::skins::{Appearance, DyeId};

pub mod wardrobe;

pub type SlotFilter<S> = HashSet<S>;

pub trait FashionSlot: Eq + Hash + Copy + IntoEnumIterator + Linearize + fmt::Debug {
    fn dyable(self) -> bool;
    fn always_visible(self) -> bool;
}

#[derive(Clone, PartialEq, Eq)]
pub struct Template<S: FashionSlot> {
    slots: StaticMap<S, Appearance>,
}

impl<S: FashionSlot> Template<S> {
    pub fn new(slots: HashMap<S, Appearance>) -> Self {
        Template {
            slots: StaticMap::from_fn(|slot| match slots.get(&slot) {
                Some(slot) => *slot,
                None => Appearance::empty(slot.dyable()),
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
        for (slot, appearance) in self.slots.iter() {
            let merged = appearance.merge(other.get_slot(&slot), ignore_skin, ignore_dyes);
            slots.insert(slot, merged);
        }
        Self::new(slots)
    }

    pub fn all_dye_ids(&self) -> HashSet<DyeId> {
        let dyes = self
            .iter()
            .filter_map(|(_, appearance)| appearance.dyes())
            .flat_map(|dyes| dyes.into_iter());
        HashSet::from_iter(dyes)
    }
}

impl<'a, S: FashionSlot> IntoIterator for &'a Template<S> {
    type Item = (S, &'a Appearance);
    type IntoIter = <&'a StaticMap<S, Appearance> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.slots.iter()
    }
}

impl<S: FashionSlot> fmt::Debug for Template<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.slots.iter()).finish()
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
        SlotFilter::from_iter(S::iter())
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
