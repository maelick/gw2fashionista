use std::collections::HashSet;
use std::sync::Arc;

use gw2lib::cache::InMemoryCache;
use gw2lib::rate_limit::BucketRateLimiter;
use gw2lib::{Client, EndpointError, Requester};
use gw2lib::model::{items::{Item, skins::Skin}, misc::colors::Color};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

use crate::domain::chatlink::ChatLink;
use crate::domain::error::ChatLinkError;
use crate::domain::skins::{DyeId, SkinId};
use crate::domain::wardrobe_template::WardrobeTemplate;
use crate::gw2_data::cache::{Cache, Resolver as CacheResolver};
use crate::gw2_data::equipment::Equipment;
use crate::gw2_data::outfit::Outfit;
use crate::gw2_data::retry::Retry;
use crate::models::wardrobe_template::WardrobeTemplateData;
use crate::models::skin;

mod cache;
mod retry;
mod outfit;
pub mod equipment;
pub mod import;

pub struct Resolver<Req>
where
    Req: Requester<false, false>,
{
    items: Cache<Item, u32, Req>,
    skins: Cache<Skin, u32, Req>,
    outfits: cache::Cache<Outfit, u32, Req>,
    colors: Cache<Color, u16, Req>,
    retry: Retry,
}

impl<Req> Resolver<Req>
where
    Req: Requester<false, false>,
{
    pub fn new(req: Req) -> Self {
        let req = Arc::new(req);
        Resolver{
            items: cache::Cache::new(req.clone()),
            skins: cache::Cache::new(req.clone()),
            outfits: cache::Cache::new(req.clone()),
            colors: cache::Cache::new(req.clone()),
            retry: Retry::default(),
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.skins.clear();
        self.colors.clear();
    }

    pub fn skin(&mut self, id: SkinId) -> Result<Skin, EndpointError> {
        self.retry.retry(|| self.skins.get(id.into()))
    }

    pub fn outfit(&mut self, id: SkinId) -> Result<Outfit, EndpointError> {
        self.retry.retry(|| self.outfits.get(id.into()))
    }

    pub fn dye(&mut self, id: DyeId) -> Result<Color, EndpointError> {
        self.retry.retry(|| self.colors.get(id.into()))
    }

    pub fn item(&mut self, id: u32) -> Result<Item, EndpointError> {
        self.retry.retry(|| self.items.get(id))
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
        log::info!("Retrieving skin data");
        self.skins.ensure(skins.into_iter().map(|id| id.into()))?;
        log::info!("Retrieving color data");
        self.colors.ensure(dyes.into_iter().map(|id| id.into()))
    }

    pub fn resolve_equipment(&mut self, equipments: Vec<Equipment>) -> Result<Vec<Equipment>, EndpointError> {
        let mut items = HashSet::new();
        for e in &equipments {
            items.extend(e.all_item_ids().into_iter());
        }
        log::info!("Retrieving item data");
        self.items.ensure(items.into_iter())?;

        equipments.into_iter().map(|e| e.resolve_default_skins(&mut self.items)).collect()
    }

    pub fn resolve_chat_link(&mut self, chat_link: &ChatLink) -> Result<WardrobeTemplateData, ChatLinkError> {
        match chat_link {
            ChatLink::WardrobeTemplate(template) => {
                Ok(self.resolve_wardrobe_template(template))
            },
            _ => Err(ChatLinkError::NotImplemented)
        }
    }

    pub fn resolve_wardrobe_template(&mut self, template: &WardrobeTemplate) -> WardrobeTemplateData {
        let data = template.into();
        self.resolve_wardrobe_template_data(&data)
    }

    pub fn resolve_wardrobe_template_data(&mut self, template: &WardrobeTemplateData) -> WardrobeTemplateData {
        WardrobeTemplateData {
            aquabreather: self.resolve_wardrobe_slot(&template.aquabreather),
            backpack: self.resolve_wardrobe_slot(&template.backpack),
            chest: self.resolve_wardrobe_slot(&template.chest),
            shoes: self.resolve_wardrobe_slot(&template.shoes),
            gloves: self.resolve_wardrobe_slot(&template.gloves),
            head: self.resolve_wardrobe_slot(&template.head),
            legs: self.resolve_wardrobe_slot(&template.legs),
            shoulders: self.resolve_wardrobe_slot(&template.shoulders),
            outfit: self.resolve_outfit(&template.outfit),
            weapon_aquatic_a: self.resolve_wardrobe_slot(&template.weapon_aquatic_a),
            weapon_aquatic_b: self.resolve_wardrobe_slot(&template.weapon_aquatic_b),
            weapon_a1: self.resolve_wardrobe_slot(&template.weapon_a1),
            weapon_a2: self.resolve_wardrobe_slot(&template.weapon_a2),
            weapon_b1: self.resolve_wardrobe_slot(&template.weapon_b1),
            weapon_b2: self.resolve_wardrobe_slot(&template.weapon_b2),
        }
    }

    fn resolve_outfit(&mut self, skin: &Option<skin::Skin>) -> Option<skin::Skin> {
        skin.as_ref().map(|skin| {
            skin::Skin{
                name: self.resolve_outfit_name(skin.id),
                dyes: self.resolve_dyes(&skin.dyes),
                ..*skin
            }
        })
    }

    fn resolve_wardrobe_slot(&mut self, skin: &Option<skin::Skin>) -> Option<skin::Skin> {
        skin.as_ref().map(|skin| {
            skin::Skin{
                name: self.resolve_skin_name(skin.id),
                dyes: self.resolve_dyes(&skin.dyes),
                ..*skin
            }
        })
    }

    fn resolve_outfit_name(&mut self, id: u16) -> Option<String> {
        Some(self.outfit(id.into()).unwrap().name)
    }

    fn resolve_skin_name(&mut self, id: u16) -> Option<String> {
        Some(self.skin(id.into()).unwrap().name)
    }

    fn resolve_dyes(&mut self, dyes: &Option<skin::Dyes>) -> Option<skin::Dyes> {
        dyes.as_ref().map(|(dye1, dye2, dye3, dye4)| {
            (
                self.resolve_dye_name(dye1),
                self.resolve_dye_name(dye2),
                self.resolve_dye_name(dye3),
                self.resolve_dye_name(dye4)
            )
        })
    }

    fn resolve_dye_name(&mut self, dye: &skin::Dye) -> skin::Dye {
        skin::Dye{
            name: Some(self.dye(dye.id.into()).unwrap().name),
            ..*dye
        }
    }
}

impl Default for Resolver<Client<InMemoryCache, BucketRateLimiter, HttpsConnector<HttpConnector>, false>> {
    fn default() -> Self {
        Self::new(Client::default())
    }
}