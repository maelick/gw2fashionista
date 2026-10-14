use serde::{Deserialize, Serialize};

use gw2fashionista_chatlink::templates::appearance::{self, Appearance, DyeId};

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

impl From<&Skin> for Appearance {
    fn from(skin: &Skin) -> Self {
        match &skin.dyes {
            Some(dyes) => Appearance::Dyeable {
                skin: skin.id.into(),
                visible: skin.visible.unwrap_or(true),
                dyes: dyes_from_model(dyes.clone()),
            },
            None => Appearance::NonDyeable {
                skin: skin.id.into(),
                visible: skin.visible.unwrap_or(true),
            },
        }
    }
}

impl From<&Appearance> for Skin {
    fn from(appearance: &Appearance) -> Self {
        match appearance {
            Appearance::NonDyeable { skin, visible } => Skin {
                id: (*skin).into(),
                name: None,
                dyes: None,
                visible: Some(*visible),
            },
            Appearance::Dyeable {
                skin,
                visible,
                dyes,
            } => Skin {
                id: (*skin).into(),
                name: None,
                dyes: Some(model_from_dyes(*dyes)),
                visible: Some(*visible),
            },
        }
    }
}

impl From<Dye> for DyeId {
    fn from(dye: Dye) -> Self {
        dye.id.into()
    }
}

impl From<DyeId> for Dye {
    fn from(dye: DyeId) -> Self {
        Dye {
            id: dye.into(),
            name: None,
        }
    }
}

fn dyes_from_model((dye1, dye2, dye3, dye4): Dyes) -> appearance::Dyes {
    appearance::Dyes::new(dye1.into(), dye2.into(), dye3.into(), dye4.into())
}

fn model_from_dyes(dyes: appearance::Dyes) -> Dyes {
    let (dye1, dye2, dye3, dye4): (DyeId, DyeId, DyeId, DyeId) = dyes.into();
    (dye1.into(), dye2.into(), dye3.into(), dye4.into())
}
