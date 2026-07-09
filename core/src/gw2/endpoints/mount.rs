use gw2lib::model::{BulkEndpoint, Endpoint, EndpointWithId, misc::colors::ColorId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct MountDyeSlot {
    pub color_id: ColorId,
    pub material: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct MountSkin {
    pub id: u32,
    pub name: String,
    pub icon: String,
    pub dye_slots: Vec<MountDyeSlot>,
    pub mount_guid: String,
}

impl Endpoint for MountSkin {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/mounts/skins";
    const VERSION: &'static str = "2025-08-29T01:00:00.000Z";
}
impl EndpointWithId for MountSkin {
    type IdType = u32;
}
impl BulkEndpoint for MountSkin {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
