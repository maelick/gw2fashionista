use std::collections::HashSet;
use std::sync::Arc;

use gw2lib::{EndpointError, Requester};
use gw2lib::model::{items::{Item, skins::Skin}, misc::colors::Color};

use crate::domain::skins::{DyeId, SkinId};
use crate::domain::wardrobe_template::WardrobeTemplate;

mod cache;

pub struct GW2DataClient<Req>
where
    Req: Requester<false, false>,
{
    items: cache::Cache<Item, u32, Req>,
    skins: cache::Cache<Skin, u32, Req>,
    colors: cache::Cache<Color, u16, Req>,
    // outfits: cache::Cache<Outfit, u32, Req>,
}

impl<Req> GW2DataClient<Req>
where
    Req: Requester<false, false>,
{
    pub fn new(req: Req) -> Self {
        let req = Arc::new(req);
        GW2DataClient{
            items: cache::Cache::new(req.clone()),
            skins: cache::Cache::new(req.clone()),
            colors: cache::Cache::new(req.clone()),
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.skins.clear();
        self.colors.clear();
    }

    pub fn skin(&mut self, id: SkinId) -> Result<Skin, EndpointError> {
        self.skins.get(id.into())
    }

    pub fn dye(&mut self, id: DyeId) -> Result<Item, EndpointError> {
        self.items.get(id.into())
    }

    pub fn item(&mut self, id: u32) -> Result<Item, EndpointError> {
        self.items.get(id)
    }

    pub fn cache_wardrobe_templates<'a, Templates: IntoIterator<Item=&'a WardrobeTemplate>>(&mut self, templates: Templates) -> Result<(), EndpointError> {
        let mut skins = HashSet::new();
        let mut dyes = HashSet::new();
        for t in templates {
            skins.extend(t.all_skin_ids().into_iter());
            dyes.extend(t.all_dye_ids().into_iter());
        }
        self.fetch_missing_fashion_data(skins, dyes)
    }

    pub fn cache_wardrobe_template(&mut self, template: &WardrobeTemplate) -> Result<(), EndpointError> {
        self.fetch_missing_fashion_data(template.all_skin_ids(), template.all_dye_ids())
    }

    fn fetch_missing_fashion_data<Skins: IntoIterator<Item=SkinId>, Dyes:IntoIterator<Item=DyeId>>(&mut self, skins: Skins, dyes: Dyes) -> Result<(), EndpointError> {
        self.skins.ensure(skins.into_iter().map(|id| id.into()))?;
        self.colors.ensure(dyes.into_iter().map(|id| id.into()))
    }
}
