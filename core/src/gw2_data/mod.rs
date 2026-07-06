use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use futures::stream::{self, StreamExt, TryStreamExt};
use gw2lib::cache::InMemoryCache;
use gw2lib::model::{
    items::{Item, skins::Skin},
    misc::colors::Color,
};
use gw2lib::rate_limit::BucketRateLimiter;
use gw2lib::{Client, EndpointError, Requester};
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

use crate::domain::chatlink::ChatLink;
use crate::domain::error::ChatLinkError;
use crate::domain::skins::{DyeId, SkinId};
use crate::domain::templates::wardrobe::{WardrobeSlot, WardrobeTemplate};
use crate::gw2_data::cache::{Cache, Resolver as CacheResolver};
use crate::gw2_data::equipment::Equipment;
use crate::gw2_data::outfit::Outfit;
use crate::gw2_data::retry::Retry;
use crate::models::skin;
use crate::models::template::WardrobeTemplateData;

mod cache;
pub mod equipment;
pub mod import;
mod outfit;
mod retry;

const DEFAULT_BUFFER_SIZE: usize = 10;

pub struct Resolver<Req>
where
    Req: Requester<false, false> + Send + Sync,
{
    items: Cache<Item, u32, Req>,
    skins: Cache<Skin, u32, Req>,
    outfits: Cache<Outfit, u32, Req>,
    colors: Cache<Color, u16, Req>,
    retry: Retry,
    buffer_size: usize,
}

impl<Req> Resolver<Req>
where
    Req: Requester<false, false> + Send + Sync,
{
    pub fn new(req: Req) -> Self {
        let req = Arc::new(req);
        Resolver {
            items: Cache::new(req.clone()),
            skins: Cache::new(req.clone()),
            outfits: Cache::new(req.clone()),
            colors: Cache::new(req.clone()),
            retry: Retry::default(),
            buffer_size: DEFAULT_BUFFER_SIZE,
        }
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    pub fn clear(&self) {
        self.items.clear();
        self.skins.clear();
        self.colors.clear();
    }

    pub async fn skin(&self, id: SkinId) -> Result<Skin, EndpointError> {
        self.retry.start(|| self.skins.get(id.into())).await
    }

    pub async fn outfit(&self, id: SkinId) -> Result<Outfit, EndpointError> {
        self.retry.start(|| self.outfits.get(id.into())).await
    }

    pub async fn dye(&self, id: DyeId) -> Result<Color, EndpointError> {
        self.retry.start(|| self.colors.get(id.into())).await
    }

    pub async fn item(&self, id: u32) -> Result<Item, EndpointError> {
        self.retry.start(|| self.items.get(id)).await
    }

    pub async fn cache_wardrobe_templates<
        'a,
        Templates: IntoIterator<Item = &'a WardrobeTemplate>,
    >(
        &self,
        templates: Templates,
    ) -> Result<(), EndpointError> {
        let mut skins = HashSet::new();
        let mut dyes = HashSet::new();
        for t in templates {
            skins.extend(t.all_skin_ids());
            dyes.extend(t.all_dye_ids());
        }
        self.fetch_missing_fashion_data(skins, dyes).await
    }

    pub async fn cache_wardrobe_template(
        &self,
        template: &WardrobeTemplate,
    ) -> Result<(), EndpointError> {
        self.fetch_missing_fashion_data(template.all_skin_ids(), template.all_dye_ids())
            .await
    }

    async fn fetch_missing_fashion_data<
        Skins: IntoIterator<Item = SkinId>,
        Dyes: IntoIterator<Item = DyeId>,
    >(
        &self,
        skins: Skins,
        dyes: Dyes,
    ) -> Result<(), EndpointError> {
        tokio::try_join!(
            self.skins
                .ensure(skins.into_iter().map(|id| id.into()).collect()),
            self.colors
                .ensure(dyes.into_iter().map(|id| id.into()).collect()),
        )?;
        Ok(())
    }

    pub async fn resolve_equipment(
        &self,
        equipments: Vec<Equipment>,
    ) -> Result<Vec<Equipment>, EndpointError> {
        let mut items = HashSet::new();
        for e in &equipments {
            items.extend(e.all_item_ids());
        }
        self.items.ensure(items.into_iter().collect()).await?;

        stream::iter(equipments)
            .map(async |e| e.resolve_default_skins(&self.items).await)
            .buffered(self.buffer_size)
            .try_collect()
            .await
    }

    pub async fn resolve_chat_link(
        &self,
        chat_link: &ChatLink,
    ) -> Result<WardrobeTemplateData, ChatLinkError> {
        match chat_link {
            ChatLink::WardrobeTemplate(template) => {
                Ok(self.resolve_wardrobe_template(template).await.unwrap())
            }
            _ => Err(ChatLinkError::NotImplemented),
        }
    }

    pub async fn resolve_wardrobe_template(
        &self,
        template: &WardrobeTemplate,
    ) -> Result<WardrobeTemplateData, EndpointError> {
        let data = template.into();
        self.resolve_wardrobe_template_data(&data).await
    }

    pub async fn resolve_wardrobe_template_data(
        &self,
        template: &WardrobeTemplateData,
    ) -> Result<WardrobeTemplateData, EndpointError> {
        let mut slots = HashMap::with_capacity(template.len());
        for (slot, skin) in template {
            let resolved_skin = match slot {
                WardrobeSlot::Outfit => self.resolve_outfit(Some(skin)).await?,
                _ => self.resolve_wardrobe_slot(Some(skin)).await?,
            };
            if let Some(skin) = resolved_skin {
                slots.insert(*slot, skin);
            }
        }
        Ok(WardrobeTemplateData::new(slots))
    }

    async fn resolve_outfit(
        &self,
        skin: Option<&skin::Skin>,
    ) -> Result<Option<skin::Skin>, EndpointError> {
        if let Some(skin) = skin {
            let (name, dyes) = tokio::try_join!(
                self.resolve_outfit_name(skin.id),
                self.resolve_dyes(&skin.dyes),
            )?;
            Ok(Some(skin::Skin {
                name,
                dyes,
                ..*skin
            }))
        } else {
            Ok(None)
        }
    }

    async fn resolve_wardrobe_slot(
        &self,
        skin: Option<&skin::Skin>,
    ) -> Result<Option<skin::Skin>, EndpointError> {
        if let Some(skin) = skin {
            let (name, dyes) = tokio::try_join!(
                self.resolve_skin_name(skin.id),
                self.resolve_dyes(&skin.dyes),
            )?;
            Ok(Some(skin::Skin {
                name,
                dyes,
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

    async fn resolve_dyes(
        &self,
        dyes: &Option<skin::Dyes>,
    ) -> Result<Option<skin::Dyes>, EndpointError> {
        if let Some((dye1, dye2, dye3, dye4)) = dyes {
            Ok(Some(tokio::try_join!(
                self.resolve_dye_name(dye1),
                self.resolve_dye_name(dye2),
                self.resolve_dye_name(dye3),
                self.resolve_dye_name(dye4),
            )?))
        } else {
            Ok(None)
        }
    }

    async fn resolve_dye_name(&self, dye: &skin::Dye) -> Result<skin::Dye, EndpointError> {
        Ok(skin::Dye {
            name: Some(self.dye(dye.id.into()).await?.name),
            ..*dye
        })
    }
}

impl Default
    for Resolver<Client<InMemoryCache, BucketRateLimiter, HttpsConnector<HttpConnector>, false>>
{
    fn default() -> Self {
        Self::new(Client::default())
    }
}
