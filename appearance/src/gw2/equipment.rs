use std::collections::{HashMap, HashSet};

use gw2lib::model::{
    authenticated::characters::{Equip, EquipmentTab, Slot as EquipmentSlot},
    items::ItemId,
};

use gw2fashionista_chatlink::domain::{
    skins::{Appearance, DyeId, Dyes},
    templates::{
        FashionSlot,
        wardrobe::{WardrobeSlot, WardrobeTemplate},
    },
};

#[derive(Clone, Debug)]
pub struct Equipment {
    pub char_name: String,
    pub tab_id: usize,
    pub tab_name: String,
    pub slots: Vec<Equip>,
}

impl Equipment {
    pub fn new(char_name: &str, api_data: &EquipmentTab) -> Self {
        Equipment {
            char_name: char_name.to_string(),
            tab_id: api_data.tab,
            tab_name: api_data.name.clone(),
            slots: api_data.equipment.clone(),
        }
    }

    pub fn with_slots(&self, slots: Vec<Equip>) -> Self {
        Equipment {
            char_name: self.char_name.clone(),
            tab_id: self.tab_id,
            tab_name: self.tab_name.clone(),
            slots,
        }
    }

    pub fn all_item_ids(&self) -> HashSet<ItemId> {
        HashSet::from_iter(
            self.slots
                .iter()
                .filter_map(|s| if s.skin.is_none() { Some(s.id) } else { None }),
        )
    }

    pub fn to_template(&self) -> WardrobeTemplate {
        let mut slots = HashMap::new();
        for equip in &self.slots {
            if let Some(slot) = equip.slot.as_ref().and_then(to_wardrobe_slot) {
                slots.insert(slot, appearance_from(&slot, equip));
            }
        }
        WardrobeTemplate::new(slots)
    }
}

fn to_wardrobe_slot(slot: &EquipmentSlot) -> Option<WardrobeSlot> {
    match slot {
        EquipmentSlot::HelmAquatic => Some(WardrobeSlot::Aquabreather),
        EquipmentSlot::Backpack => Some(WardrobeSlot::Backpack),
        EquipmentSlot::Coat => Some(WardrobeSlot::Chest),
        EquipmentSlot::Boots => Some(WardrobeSlot::Shoes),
        EquipmentSlot::Gloves => Some(WardrobeSlot::Gloves),
        EquipmentSlot::Helm => Some(WardrobeSlot::Head),
        EquipmentSlot::Leggings => Some(WardrobeSlot::Legs),
        EquipmentSlot::Shoulders => Some(WardrobeSlot::Shoulders),
        EquipmentSlot::WeaponAquaticA => Some(WardrobeSlot::WeaponAquaticA),
        EquipmentSlot::WeaponAquaticB => Some(WardrobeSlot::WeaponAquaticB),
        EquipmentSlot::WeaponA1 => Some(WardrobeSlot::WeaponA1),
        EquipmentSlot::WeaponA2 => Some(WardrobeSlot::WeaponA2),
        EquipmentSlot::WeaponB1 => Some(WardrobeSlot::WeaponB1),
        EquipmentSlot::WeaponB2 => Some(WardrobeSlot::WeaponB2),
        _ => None,
    }
}

fn appearance_from(slot: &WardrobeSlot, equip: &Equip) -> Appearance {
    let skin = equip.skin.unwrap_or(0).into();
    if slot.dyeable() {
        Appearance::Dyeable {
            skin,
            visible: true,
            dyes: if let Some(dyes) = equip.dyes.as_ref() {
                dyes_from_iter(dyes.iter().map(DyeId::from))
            } else {
                Dyes::default()
            },
        }
    } else {
        Appearance::NonDyeable {
            skin,
            visible: true,
        }
    }
}

fn dyes_from_iter(mut dyes: impl Iterator<Item = DyeId>) -> Dyes {
    Dyes::new(
        dyes.next().unwrap_or_default(),
        dyes.next().unwrap_or_default(),
        dyes.next().unwrap_or_default(),
        dyes.next().unwrap_or_default(),
    )
}
