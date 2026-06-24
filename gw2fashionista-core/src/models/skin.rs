use serde::{Serialize, Deserialize};

use crate::domain::{skins, wardrobe_template::slot::WardrobeSlot};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skin {
    pub id: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dyes: Option<Dyes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
}

pub type Dyes = (Dye, Dye, Dye, Dye);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dye {
    pub id: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl From<&Skin> for WardrobeSlot {
    fn from(skin: &Skin) -> Self {
        match &skin.dyes {
            Some(dyes) => WardrobeSlot::Dyable { skin: skin.id.into(), visible: skin.visible.unwrap_or(true), dyes: dyes.clone().into() },
            None => WardrobeSlot::NonDyable { skin: skin.id.into(), visible: skin.visible.unwrap_or(true) },
        }
    }
}

impl From<Dye> for skins::DyeId {
    fn from(dye: Dye) -> Self {
        dye.id.into()
    }
}

impl From<skins::DyeId> for Dye {
    fn from(dye: skins::DyeId) -> Self {
        Dye {
            id: dye.into(),
            name: None,
        }
    }
}

impl From<Dyes> for skins::Dyes {
    fn from((dye1, dye2, dye3, dye4): Dyes) -> Self {
        skins::Dyes::new(dye1.into(), dye2.into(), dye3.into(), dye4.into())
    }
}

impl From<skins::Dyes> for Dyes {
    fn from(dyes: skins::Dyes) -> Self {
        let (dye1, dye2, dye3, dye4): (skins::DyeId, skins::DyeId, skins::DyeId, skins::DyeId) = dyes.into();
        (dye1.into(), dye2.into(), dye3.into(), dye4.into())
    }
}
