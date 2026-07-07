use gw2lib::model::{BulkEndpoint, Endpoint, EndpointWithId, items::ItemId, misc::colors::ColorId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Glider {
    pub id: u32,
    pub unlock_items: Vec<ItemId>,
    pub order: u32,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub default_dyes: Vec<ColorId>,
}

impl Endpoint for Glider {
    const AUTHENTICATED: bool = false;
    const LOCALE: bool = true;
    const URL: &'static str = "v2/gliders";
    const VERSION: &'static str = "2025-08-29T01:00:00.000Z";
}
impl EndpointWithId for Glider {
    type IdType = u32;
}
impl BulkEndpoint for Glider {
    const ALL: bool = true;

    fn id(&self) -> &Self::IdType {
        &self.id
    }
}
