use std::collections::{HashMap, HashSet};

use futures::stream::{self, StreamExt, TryStreamExt};
use gw2lib::{
    EndpointError,
    model::{
        authenticated::characters::{Equip, EquipmentTab, Slot as EquipmentSlot},
        items::{Item, ItemId},
        misc::colors::ColorId,
    },
};

use crate::{
    domain::{
        skins::{DyeId, Dyes, SkinId, Slot},
        wardrobe_template::{WardrobeTemplate, slot::SlotType},
    },
    gw2_data::cache,
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

    pub fn all_item_ids(&self) -> HashSet<ItemId> {
        HashSet::from_iter(
            self.slots
                .iter()
                .filter_map(|s| if s.skin.is_none() { Some(s.id) } else { None }),
        )
    }

    pub async fn resolve_default_skins<R: cache::Resolver<Item, ItemId>>(
        self,
        cache: &R,
    ) -> Result<Self, EndpointError> {
        Ok(Equipment {
            char_name: self.char_name.clone(),
            tab_id: self.tab_id,
            tab_name: self.tab_name.clone(),
            slots: self.resolve_slots_default_skins(cache).await?,
        })
    }

    async fn resolve_slots_default_skins<R: cache::Resolver<Item, ItemId>>(
        self,
        cache: &R,
    ) -> Result<Vec<Equip>, EndpointError> {
        stream::iter(self.slots)
            .then(async |s| {
                if s.skin.is_none() {
                    Ok::<_, EndpointError>(Equip {
                        skin: cache.get(s.id).await?.default_skin,
                        ..s
                    })
                } else {
                    Ok(s)
                }
            })
            .try_collect()
            .await
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
            if let Some(Ok(slot)) = equip.slot.as_ref().map(SlotType::try_from) {
                slots.insert(slot, Slot::from((&slot, equip)));
            }
        }
        Self::new(slots)
    }
}

impl TryFrom<&EquipmentSlot> for SlotType {
    type Error = ();

    fn try_from(slot: &EquipmentSlot) -> Result<Self, Self::Error> {
        match slot {
            EquipmentSlot::HelmAquatic => Ok(SlotType::Aquabreather),
            EquipmentSlot::Backpack => Ok(SlotType::Backpack),
            EquipmentSlot::Coat => Ok(SlotType::Chest),
            EquipmentSlot::Boots => Ok(SlotType::Shoes),
            EquipmentSlot::Gloves => Ok(SlotType::Gloves),
            EquipmentSlot::Helm => Ok(SlotType::Head),
            EquipmentSlot::Leggings => Ok(SlotType::Legs),
            EquipmentSlot::Shoulders => Ok(SlotType::Shoulders),
            EquipmentSlot::WeaponAquaticA => Ok(SlotType::WeaponAquaticA),
            EquipmentSlot::WeaponAquaticB => Ok(SlotType::WeaponAquaticB),
            EquipmentSlot::WeaponA1 => Ok(SlotType::WeaponA1),
            EquipmentSlot::WeaponA2 => Ok(SlotType::WeaponA2),
            EquipmentSlot::WeaponB1 => Ok(SlotType::WeaponB1),
            EquipmentSlot::WeaponB2 => Ok(SlotType::WeaponB2),
            _ => Err(()),
        }
    }
}

impl From<(&SlotType, &Equip)> for Slot {
    fn from((slot_type, equip): (&SlotType, &Equip)) -> Self {
        let skin = equip.skin.unwrap_or(0).into();
        if slot_type.dyable() {
            let dyes = equip.dyes.as_ref().map_or(Dyes::default(), Dyes::from);
            Slot::Dyable {
                skin,
                visible: true,
                dyes,
            }
        } else {
            Slot::NonDyable {
                skin,
                visible: true,
            }
        }
    }
}

impl From<&Vec<Option<ColorId>>> for Dyes {
    fn from(dyes: &Vec<Option<ColorId>>) -> Self {
        Dyes::new(
            dyes.first().into(),
            dyes.get(1).into(),
            dyes.get(2).into(),
            dyes.get(3).into(),
        )
    }
}

impl From<Option<&Option<u16>>> for DyeId {
    fn from(dye: Option<&Option<u16>>) -> Self {
        dye.unwrap_or(&None).map(DyeId::from).unwrap_or_default()
    }
}

impl From<u32> for SkinId {
    fn from(id: u32) -> Self {
        Self::new(id as u16)
    }
}
