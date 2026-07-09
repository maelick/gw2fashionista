use gw2lib::model::{BulkEndpoint, Endpoint, EndpointWithId, misc::colors::ColorId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct SkiffDyeSlot {
    pub color_id: ColorId,
    pub material: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Skiff {
    pub id: u32,
    pub name: String,
    pub icon: String,
    pub dye_slots: Vec<SkiffDyeSlot>,
}

impl Endpoint for Skiff {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/skiffs";
    const VERSION: &'static str = "2025-08-29T01:00:00.000Z";
}
impl EndpointWithId for Skiff {
    type IdType = u32;
}
impl BulkEndpoint for Skiff {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
