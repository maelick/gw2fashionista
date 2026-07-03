use std::collections::HashMap;

use linearize::Linearize;
use serde::{Deserialize, Serialize};

use crate::domain::skins::Appearance;
use crate::domain::templates::wardrobe::WardrobeTemplate;
use crate::domain::templates::wardrobe::slot::WardrobeSlot;
use crate::models::error::{AppearanceKind, ModelError};
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
    fn from_map(map: &HashMap<WardrobeSlot, Appearance>) -> Result<Self, ModelError> {
        Ok(WardrobeTemplateData {
            aquabreather: skin_from_map(map, WardrobeSlot::Aquabreather)?,
            backpack: dyeable_skin_from_map(map, WardrobeSlot::Backpack)?,
            chest: dyeable_skin_from_map(map, WardrobeSlot::Chest)?,
            shoes: dyeable_skin_from_map(map, WardrobeSlot::Shoes)?,
            gloves: dyeable_skin_from_map(map, WardrobeSlot::Gloves)?,
            head: dyeable_skin_from_map(map, WardrobeSlot::Head)?,
            legs: dyeable_skin_from_map(map, WardrobeSlot::Legs)?,
            shoulders: dyeable_skin_from_map(map, WardrobeSlot::Shoulders)?,
            outfit: dyeable_skin_from_map(map, WardrobeSlot::Outfit)?,
            weapon_aquatic_a: skin_from_map(map, WardrobeSlot::WeaponAquaticA)?,
            weapon_aquatic_b: skin_from_map(map, WardrobeSlot::WeaponAquaticB)?,
            weapon_a1: skin_from_map(map, WardrobeSlot::WeaponA1)?,
            weapon_a2: skin_from_map(map, WardrobeSlot::WeaponA2)?,
            weapon_b1: skin_from_map(map, WardrobeSlot::WeaponB1)?,
            weapon_b2: skin_from_map(map, WardrobeSlot::WeaponB2)?,
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
        let mut slots: HashMap<WardrobeSlot, Appearance> =
            HashMap::<WardrobeSlot, Appearance>::with_capacity(WardrobeSlot::LENGTH);
        insert_slot(
            &mut slots,
            &template.aquabreather,
            WardrobeSlot::Aquabreather,
        );
        insert_dyeable_slot(&mut slots, &template.backpack, WardrobeSlot::Backpack);
        insert_dyeable_slot(&mut slots, &template.chest, WardrobeSlot::Chest);
        insert_dyeable_slot(&mut slots, &template.shoes, WardrobeSlot::Shoes);
        insert_dyeable_slot(&mut slots, &template.gloves, WardrobeSlot::Gloves);
        insert_dyeable_slot(&mut slots, &template.head, WardrobeSlot::Head);
        insert_dyeable_slot(&mut slots, &template.legs, WardrobeSlot::Legs);
        insert_dyeable_slot(&mut slots, &template.shoulders, WardrobeSlot::Shoulders);
        insert_dyeable_slot(&mut slots, &template.outfit, WardrobeSlot::Outfit);
        insert_slot(
            &mut slots,
            &template.weapon_aquatic_a,
            WardrobeSlot::WeaponAquaticA,
        );
        insert_slot(
            &mut slots,
            &template.weapon_aquatic_b,
            WardrobeSlot::WeaponAquaticB,
        );
        insert_slot(&mut slots, &template.weapon_a1, WardrobeSlot::WeaponA1);
        insert_slot(&mut slots, &template.weapon_a2, WardrobeSlot::WeaponA2);
        insert_slot(&mut slots, &template.weapon_b1, WardrobeSlot::WeaponB1);
        insert_slot(&mut slots, &template.weapon_b2, WardrobeSlot::WeaponB2);
        Self::new(slots)
    }
}

fn skin_from_map(
    map: &HashMap<WardrobeSlot, Appearance>,
    slot: WardrobeSlot,
) -> Result<Option<Skin>, ModelError> {
    let res = map.get(&slot);
    res.map_or(Ok(None), |appearance| match appearance {
        Appearance::NonDyeable { skin, visible } => Ok(Some(Skin {
            id: (*skin).into(),
            name: None,
            dyes: None,
            visible: Some(*visible),
        })),
        Appearance::Dyeable {
            skin: _,
            visible: _,
            dyes: _,
        } => Err(ModelError::IncorrectSlotVariant {
            slot,
            expected: AppearanceKind::Dyeable,
            found: AppearanceKind::NonDyeable,
        }),
    })
}

fn dyeable_skin_from_map(
    map: &HashMap<WardrobeSlot, Appearance>,
    slot: WardrobeSlot,
) -> Result<Option<Skin>, ModelError> {
    let res = map.get(&slot);
    res.map_or(Ok(None), |appearance| match appearance {
        Appearance::NonDyeable {
            skin: _,
            visible: _,
        } => Err(ModelError::IncorrectSlotVariant {
            slot,
            expected: AppearanceKind::NonDyeable,
            found: AppearanceKind::Dyeable,
        }),
        Appearance::Dyeable {
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

fn insert_slot(
    slots: &mut HashMap<WardrobeSlot, Appearance>,
    skin: &Option<Skin>,
    slot: WardrobeSlot,
) {
    if let Some(skin) = skin {
        slots.insert(slot, skin.into());
    }
}

fn insert_dyeable_slot(
    slots: &mut HashMap<WardrobeSlot, Appearance>,
    skin: &Option<Skin>,
    slot: WardrobeSlot,
) {
    if let Some(skin) = skin {
        slots.insert(slot, skin.into());
    }
}
