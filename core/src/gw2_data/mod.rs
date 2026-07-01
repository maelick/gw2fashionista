use std::collections::HashSet;
use std::sync::Arc;

use futures::stream::{self, StreamExt, TryStreamExt};
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
    Req: Requester<false, false> + Send + Sync,
{
    items: Cache<Item, u32, Req>,
    skins: Cache<Skin, u32, Req>,
    outfits: Cache<Outfit, u32, Req>,
    colors: Cache<Color, u16, Req>,
    retry: Retry,
}

impl<Req> Resolver<Req>
where
    Req: Requester<false, false> + Send + Sync,
{
    pub fn new(req: Req) -> Self {
        let req = Arc::new(req);
        Resolver{
            items: Cache::new(req.clone(), "item"),
            skins: Cache::new(req.clone(), "skin"),
            outfits: Cache::new(req.clone(), "outfit"),
            colors: Cache::new(req.clone(), "color"),
            retry: Retry::default(),
        }
    }

    pub fn clear(&self) {
        self.items.clear();
        self.skins.clear();
        self.colors.clear();
    }

    pub async fn skin(&self, id: SkinId) -> Result<Skin, EndpointError> {
        self.retry.retry(async || self.skins.get(id.into()).await).await
    }

    pub async fn outfit(&self, id: SkinId) -> Result<Outfit, EndpointError> {
        self.retry.retry(async || self.outfits.get(id.into()).await).await
    }

    pub async fn dye(&self, id: DyeId) -> Result<Color, EndpointError> {
        self.retry.retry(async || self.colors.get(id.into()).await).await
    }

    pub async fn item(&self, id: u32) -> Result<Item, EndpointError> {
        self.retry.retry(async || self.items.get(id).await).await
    }

    pub async fn cache_wardrobe_templates<'a, Templates: IntoIterator<Item=&'a WardrobeTemplate>>(&self, templates: Templates) -> Result<(), EndpointError> {
        let mut skins = HashSet::new();
        let mut dyes = HashSet::new();
        for t in templates {
            skins.extend(t.all_skin_ids().into_iter());
            dyes.extend(t.all_dye_ids().into_iter());
        }
        self.fetch_missing_fashion_data(skins, dyes).await
    }

    pub async fn cache_wardrobe_template(&self, template: &WardrobeTemplate) -> Result<(), EndpointError> {
        self.fetch_missing_fashion_data(template.all_skin_ids(), template.all_dye_ids()).await
    }

    async fn fetch_missing_fashion_data<Skins: IntoIterator<Item=SkinId>, Dyes:IntoIterator<Item=DyeId>>(&self, skins: Skins, dyes: Dyes) -> Result<(), EndpointError> {
        self.skins.ensure(skins.into_iter().map(|id| id.into()).collect()).await?;
        self.colors.ensure(dyes.into_iter().map(|id| id.into()).collect()).await
    }

    pub async fn resolve_equipment(&self, equipments: Vec<Equipment>) -> Result<Vec<Equipment>, EndpointError> {
        let mut items = HashSet::new();
        for e in &equipments {
            items.extend(e.all_item_ids().into_iter());
        }
        log::info!("Retrieving item data");
        self.items.ensure(items.into_iter().collect()).await?;

        stream::iter(equipments).then(async |e| e.resolve_default_skins(&self.items).await).try_collect().await
    }

    pub async fn resolve_chat_link(&self, chat_link: &ChatLink) -> Result<WardrobeTemplateData, ChatLinkError> {
        match chat_link {
            ChatLink::WardrobeTemplate(template) => {
                Ok(self.resolve_wardrobe_template(template).await.unwrap())
            },
            _ => Err(ChatLinkError::NotImplemented)
        }
    }

    pub async fn resolve_wardrobe_template(&self, template: &WardrobeTemplate) -> Result<WardrobeTemplateData, EndpointError> {
        let data = template.into();
        self.resolve_wardrobe_template_data(&data).await
    }

    pub async fn resolve_wardrobe_template_data(&self, template: &WardrobeTemplateData) -> Result<WardrobeTemplateData, EndpointError> {
        Ok(WardrobeTemplateData {
            aquabreather: self.resolve_wardrobe_slot(&template.aquabreather).await?,
            backpack: self.resolve_wardrobe_slot(&template.backpack).await?,
            chest: self.resolve_wardrobe_slot(&template.chest).await?,
            shoes: self.resolve_wardrobe_slot(&template.shoes).await?,
            gloves: self.resolve_wardrobe_slot(&template.gloves).await?,
            head: self.resolve_wardrobe_slot(&template.head).await?,
            legs: self.resolve_wardrobe_slot(&template.legs).await?,
            shoulders: self.resolve_wardrobe_slot(&template.shoulders).await?,
            outfit: self.resolve_outfit(&template.outfit).await?,
            weapon_aquatic_a: self.resolve_wardrobe_slot(&template.weapon_aquatic_a).await?,
            weapon_aquatic_b: self.resolve_wardrobe_slot(&template.weapon_aquatic_b).await?,
            weapon_a1: self.resolve_wardrobe_slot(&template.weapon_a1).await?,
            weapon_a2: self.resolve_wardrobe_slot(&template.weapon_a2).await?,
            weapon_b1: self.resolve_wardrobe_slot(&template.weapon_b1).await?,
            weapon_b2: self.resolve_wardrobe_slot(&template.weapon_b2).await?,
        })
    }

    async fn resolve_outfit(&self, skin: &Option<skin::Skin>) -> Result<Option<skin::Skin>, EndpointError> {
        if let Some(skin) = skin {
            Ok(Some(skin::Skin{
                name: self.resolve_outfit_name(skin.id).await?,
                dyes: self.resolve_dyes(&skin.dyes).await?,
                ..*skin
            }))
        } else {
            Ok(None)
        }
    }

    async fn resolve_wardrobe_slot(&self, skin: &Option<skin::Skin>) -> Result<Option<skin::Skin>, EndpointError> {
        if let Some(skin) = skin {
            Ok(Some(skin::Skin{
                name: self.resolve_skin_name(skin.id).await?,
                dyes: self.resolve_dyes(&skin.dyes).await?,
                ..*skin
            }))
        } else {
            Ok(None)
        }
    }

    async fn resolve_outfit_name(&self, id: u16) -> Result<Option<String>, EndpointError> {
        Ok(Some(self.outfit(id.into()).await?.name))
    }

    async fn resolve_skin_name(&self, id: u16) -> Result<Option<String>, EndpointError> {
        Ok(Some(self.skin(id.into()).await?.name))
    }

    async fn resolve_dyes(&self, dyes: &Option<skin::Dyes>) -> Result<Option<skin::Dyes>, EndpointError> {
        if let Some((dye1, dye2, dye3, dye4)) = dyes {
            Ok(Some((
                self.resolve_dye_name(dye1).await?,
                self.resolve_dye_name(dye2).await?,
                self.resolve_dye_name(dye3).await?,
                self.resolve_dye_name(dye4).await?
            )))
        } else {
            Ok(None)
        }
    }

    async fn resolve_dye_name(&self, dye: &skin::Dye) -> Result<skin::Dye, EndpointError> {
        Ok(skin::Dye{
            name: Some(self.dye(dye.id.into()).await?.name),
            ..*dye
        })
    }
}

impl Default for Resolver<Client<InMemoryCache, BucketRateLimiter, HttpsConnector<HttpConnector>, false>> {
    fn default() -> Self {
        Self::new(Client::default())
    }
}
