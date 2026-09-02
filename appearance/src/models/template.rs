use std::collections::HashMap;
use std::collections::hash_map::Iter;

use serde::{Deserialize, Serialize};

use crate::models::skin::Skin;
use gw2fashionista_chatlink::domain::skins::Appearance;
use gw2fashionista_chatlink::domain::templates::travel::TravelSlot;
use gw2fashionista_chatlink::domain::templates::wardrobe::WardrobeSlot;
use gw2fashionista_chatlink::domain::templates::{FashionSlot, Template};

pub type WardrobeTemplateData = TemplateData<WardrobeSlot>;
pub type TravelTemplateData = TemplateData<TravelSlot>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TemplateData<S: FashionSlot> {
    slots: HashMap<S, Skin>,
}

impl<S: FashionSlot> TemplateData<S> {
    pub fn new(slots: HashMap<S, Skin>) -> Self {
        TemplateData { slots }
    }

    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    pub fn get(&self, slot: &S) -> Option<&Skin> {
        self.slots.get(slot)
    }
}

impl<'a, S: FashionSlot> IntoIterator for &'a TemplateData<S> {
    type Item = (&'a S, &'a Skin);
    type IntoIter = Iter<'a, S, Skin>;

    fn into_iter(self) -> Self::IntoIter {
        self.slots.iter()
    }
}

impl<S: FashionSlot> From<&Template<S>> for TemplateData<S> {
    fn from(template: &Template<S>) -> Self {
        let mut slots = HashMap::new();
        for (slot, appearance) in template.as_map(false) {
            slots.insert(slot, (&appearance).into());
        }
        Self::new(slots)
    }
}

impl<S: FashionSlot> From<&TemplateData<S>> for Template<S> {
    fn from(template: &TemplateData<S>) -> Self {
        let mut slots: HashMap<S, Appearance> = HashMap::<S, Appearance>::with_capacity(S::LENGTH);
        for (slot, skin) in &template.slots {
            slots.insert(*slot, skin.into());
        }
        Self::new(slots)
    }
}
