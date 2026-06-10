use std::collections::HashMap;

use gw2lib::model::{authenticated::characters::{Equip, EquipmentTab, Slot}, misc::colors::ColorId};

use crate::domain::{skins::{DyeId, Dyes}, wardrobe_template::{WardrobeTemplate, slot::{EquipmentSlot, SlotType}}};

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
}

impl From<&Equipment> for WardrobeTemplate {
    fn from(equipment: &Equipment) -> Self {
        (&equipment.slots).into()
    }
}

impl From<&Vec<Equip>> for WardrobeTemplate {
    fn from(equipment: &Vec<Equip>) -> Self {
        let mut slots = HashMap::new();
        for equip in equipment {
            if let Some(slot) = convert_slot(&equip.slot) {
                slots.insert(slot, convert_equip(&slot, equip));
            }
        }
        Self::new(slots)
    }
}

fn convert_slot(slot: &Option<Slot>) -> Option<SlotType> {
    match slot {
        Some(Slot::HelmAquatic) => Some(SlotType::Aquabreather),
        Some(Slot::Backpack) => Some(SlotType::Backpack),
        Some(Slot::Coat) => Some(SlotType::Chest),
        Some(Slot::Boots) => Some(SlotType::Shoes),
        Some(Slot::Gloves) => Some(SlotType::Gloves),
        Some(Slot::Helm) => Some(SlotType::Head),
        Some(Slot::Leggings) => Some(SlotType::Legs),
        Some(Slot::Shoulders) => Some(SlotType::Shoulders),
        Some(Slot::WeaponAquaticA) => Some(SlotType::WeaponAquaticA),
        Some(Slot::WeaponAquaticB) => Some(SlotType::WeaponAquaticB),
        Some(Slot::WeaponA1) => Some(SlotType::WeaponA1),
        Some(Slot::WeaponA2) => Some(SlotType::WeaponA2),
        Some(Slot::WeaponB1) => Some(SlotType::WeaponB1),
        Some(Slot::WeaponB2) => Some(SlotType::WeaponB2),
        _ => None,
    }
}

fn convert_equip(slot_type: &SlotType, equip: &Equip) -> EquipmentSlot {
    let skin_id = equip.skin.unwrap_or(0) as u16;
    if slot_type.dyable() {
        EquipmentSlot::Dyable { skin: skin_id.into(), visible: true, dyes: convert_dyes(&equip.dyes) }
    } else {
        EquipmentSlot::NonDyable { skin: skin_id.into(), visible: true }
    }
}

fn convert_dyes(dyes: &Option<Vec<Option<ColorId>>>) -> Dyes {
    dyes.clone().map_or(Dyes::default(), |dyes| {
        let dye1 = convert_dye(dyes.get(0));
        let dye2 = convert_dye(dyes.get(1));
        let dye3 = convert_dye(dyes.get(2));
        let dye4 = convert_dye(dyes.get(3));
        Dyes::new(dye1, dye2, dye3, dye4)
    })
}

fn convert_dye(dye: Option<&Option<u16>>) -> DyeId {
    if let Some(dye) = dye {
        dye.map_or(DyeId::default(), |d| d.into())
    } else {
        DyeId::default()
    }
}
