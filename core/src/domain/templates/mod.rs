use std::{borrow::Borrow, collections::HashSet, hash::Hash};

use strum::IntoEnumIterator;

pub mod wardrobe;

pub type SlotFilter<S> = HashSet<S>;

pub trait FashionSlot: Eq + Hash + Copy + IntoEnumIterator {
    fn dyable(self) -> bool;
    fn always_visible(self) -> bool;
    fn index(self) -> usize;
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
