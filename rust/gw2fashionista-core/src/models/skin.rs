use serde::{Serialize, Deserialize};

use crate::domain::wardrobe_template::slot::EquipmentSlot;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skin {
    pub id: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DyableSkin {
    pub id: u16,
    pub dyes: (u16, u16, u16, u16),
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

impl From<&Skin> for EquipmentSlot {
    fn from(skin: &Skin) -> Self {
        EquipmentSlot::NonDyable { skin: skin.id.into(), visible: skin.visible.unwrap_or(true) }
    }
}

impl From<&DyableSkin> for EquipmentSlot {
    fn from(skin: &DyableSkin) -> Self {
        EquipmentSlot::Dyable { skin: skin.id.into(), visible: skin.visible.unwrap_or(true), dyes: skin.dyes.into() }
    }
}
