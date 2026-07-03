use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::EnumCount;

use crate::domain::skins::Slot;
use crate::domain::wardrobe_template::WardrobeTemplate;
use crate::domain::wardrobe_template::slot::SlotType;
use crate::models::error::{ModelError, SlotVariant};
use crate::models::skin::Skin;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WardrobeTemplateData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aquabreather: Option<Skin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backpack: Option<Skin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chest: Option<Skin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shoes: Option<Skin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gloves: Option<Skin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Skin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legs: Option<Skin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shoulders: Option<Skin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outfit: Option<Skin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon_aquatic_a: Option<Skin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon_aquatic_b: Option<Skin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon_a1: Option<Skin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon_a2: Option<Skin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon_b1: Option<Skin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon_b2: Option<Skin>,
}

impl WardrobeTemplateData {
    fn from_map(map: &HashMap<SlotType, Slot>) -> Result<Self, ModelError> {
        Ok(WardrobeTemplateData {
            aquabreather: skin_from_map(map, SlotType::Aquabreather)?,
            backpack: dyable_skin_from_map(map, SlotType::Backpack)?,
            chest: dyable_skin_from_map(map, SlotType::Chest)?,
            shoes: dyable_skin_from_map(map, SlotType::Shoes)?,
            gloves: dyable_skin_from_map(map, SlotType::Gloves)?,
            head: dyable_skin_from_map(map, SlotType::Head)?,
            legs: dyable_skin_from_map(map, SlotType::Legs)?,
            shoulders: dyable_skin_from_map(map, SlotType::Shoulders)?,
            outfit: dyable_skin_from_map(map, SlotType::Outfit)?,
            weapon_aquatic_a: skin_from_map(map, SlotType::WeaponAquaticA)?,
            weapon_aquatic_b: skin_from_map(map, SlotType::WeaponAquaticB)?,
            weapon_a1: skin_from_map(map, SlotType::WeaponA1)?,
            weapon_a2: skin_from_map(map, SlotType::WeaponA2)?,
            weapon_b1: skin_from_map(map, SlotType::WeaponB1)?,
            weapon_b2: skin_from_map(map, SlotType::WeaponB2)?,
        })
    }
}

impl From<&WardrobeTemplate> for WardrobeTemplateData {
    fn from(template: &WardrobeTemplate) -> Self {
        let map = template.as_map(false);
        WardrobeTemplateData::from_map(&map).unwrap()
    }
}

impl From<&WardrobeTemplateData> for WardrobeTemplate {
    fn from(template: &WardrobeTemplateData) -> Self {
        let mut slots: HashMap<SlotType, Slot> =
            HashMap::<SlotType, Slot>::with_capacity(SlotType::COUNT);
        insert_slot(&mut slots, &template.aquabreather, SlotType::Aquabreather);
        insert_dyable_slot(&mut slots, &template.backpack, SlotType::Backpack);
        insert_dyable_slot(&mut slots, &template.chest, SlotType::Chest);
        insert_dyable_slot(&mut slots, &template.shoes, SlotType::Shoes);
        insert_dyable_slot(&mut slots, &template.gloves, SlotType::Gloves);
        insert_dyable_slot(&mut slots, &template.head, SlotType::Head);
        insert_dyable_slot(&mut slots, &template.legs, SlotType::Legs);
        insert_dyable_slot(&mut slots, &template.shoulders, SlotType::Shoulders);
        insert_dyable_slot(&mut slots, &template.outfit, SlotType::Outfit);
        insert_slot(
            &mut slots,
            &template.weapon_aquatic_a,
            SlotType::WeaponAquaticA,
        );
        insert_slot(
            &mut slots,
            &template.weapon_aquatic_b,
            SlotType::WeaponAquaticB,
        );
        insert_slot(&mut slots, &template.weapon_a1, SlotType::WeaponA1);
        insert_slot(&mut slots, &template.weapon_a2, SlotType::WeaponA2);
        insert_slot(&mut slots, &template.weapon_b1, SlotType::WeaponB1);
        insert_slot(&mut slots, &template.weapon_b2, SlotType::WeaponB2);
        Self::new(slots)
    }
}

fn skin_from_map(
    map: &HashMap<SlotType, Slot>,
    slot_type: SlotType,
) -> Result<Option<Skin>, ModelError> {
    let res = map.get(&slot_type);
    res.map_or(Ok(None), |slot| match slot {
        Slot::NonDyable { skin, visible } => Ok(Some(Skin {
            id: (*skin).into(),
            name: None,
            dyes: None,
            visible: Some(*visible),
        })),
        Slot::Dyable {
            skin: _,
            visible: _,
            dyes: _,
        } => Err(ModelError::IncorrectSlotVariant {
            slot_type,
            expected: SlotVariant::Dyable,
            found: SlotVariant::NonDyable,
        }),
    })
}

fn dyable_skin_from_map(
    map: &HashMap<SlotType, Slot>,
    slot_type: SlotType,
) -> Result<Option<Skin>, ModelError> {
    let res = map.get(&slot_type);
    res.map_or(Ok(None), |slot| match slot {
        Slot::NonDyable {
            skin: _,
            visible: _,
        } => Err(ModelError::IncorrectSlotVariant {
            slot_type,
            expected: SlotVariant::NonDyable,
            found: SlotVariant::Dyable,
        }),
        Slot::Dyable {
            skin,
            visible,
            dyes,
        } => Ok(Some(Skin {
            id: (*skin).into(),
            name: None,
            dyes: Some((*dyes).into()),
            visible: Some(*visible),
        })),
    })
}

fn insert_slot(slots: &mut HashMap<SlotType, Slot>, skin: &Option<Skin>, slot_type: SlotType) {
    if let Some(skin) = skin {
        slots.insert(slot_type, skin.into());
    }
}

fn insert_dyable_slot(
    slots: &mut HashMap<SlotType, Slot>,
    skin: &Option<Skin>,
    slot_type: SlotType,
) {
    if let Some(skin) = skin {
        slots.insert(slot_type, skin.into());
    }
}
