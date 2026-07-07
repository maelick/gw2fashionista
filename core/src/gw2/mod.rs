use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use futures::stream::{self, StreamExt, TryStreamExt};
use gw2lib::model::{
    items::{Item, skins::Skin},
    misc::colors::Color,
};
use gw2lib::{Client, Requester};

use crate::domain::skins::{DyeId, SkinId};
use crate::domain::templates::wardrobe::WardrobeTemplate;
use crate::domain::templates::{FashionSlot, FashionSlotKind};
use crate::gw2::cache::{Cache, Resolver as CacheResolver};
use crate::gw2::endpoints::glider::Glider;
use crate::gw2::endpoints::mount::MountSkin;
use crate::gw2::endpoints::outfit::Outfit;
use crate::gw2::endpoints::skiff::Skiff;
use crate::gw2::equipment::Equipment;
use crate::gw2::error::Error;
use crate::gw2::fetch::Gw2LibFetcher;
use crate::gw2::retry::Retry;
use crate::models::skin;
use crate::models::template::TemplateData;

mod cache;
pub mod endpoints;
pub mod equipment;
pub mod error;
pub mod fetch;
pub mod import;
mod retry;

const DEFAULT_BUFFER_SIZE: usize = 10;

pub struct Resolver {
    items: Cache<Item, u32>,
    skins: Cache<Skin, u32>,
    outfits: Cache<Outfit, u32>,
    colors: Cache<Color, u16>,
    mounts: Cache<MountSkin, u32>,
    gliders: Cache<Glider, u32>,
    skiffs: Cache<Skiff, u32>,
    retry: Retry,
    buffer_size: usize,
}

impl Resolver {
    pub fn new<Req>(req: Req) -> Self
    where
        Req: Requester<false, false> + Send + Sync + 'static,
    {
        let req = Arc::new(req);
        Resolver {
            items: Cache::new(Box::new(Gw2LibFetcher::new(req.clone()))),
            skins: Cache::new(Box::new(Gw2LibFetcher::new(req.clone()))),
            outfits: Cache::new(Box::new(Gw2LibFetcher::new(req.clone()))),
            colors: Cache::new(Box::new(Gw2LibFetcher::new(req.clone()))),
            mounts: Cache::new(Box::new(Gw2LibFetcher::new(req.clone()))),
            gliders: Cache::new(Box::new(Gw2LibFetcher::new(req.clone()))),
            skiffs: Cache::new(Box::new(Gw2LibFetcher::new(req.clone()))),
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

    pub async fn skin(&self, id: SkinId) -> Result<Skin, Error> {
        self.retry.start(|| self.skins.get(id.into())).await
    }

    pub async fn outfit(&self, id: SkinId) -> Result<Outfit, Error> {
        self.retry.start(|| self.outfits.get(id.into())).await
    }

    pub async fn dye(&self, id: DyeId) -> Result<Color, Error> {
        self.retry.start(|| self.colors.get(id.into())).await
    }

    pub async fn item(&self, id: u32) -> Result<Item, Error> {
        self.retry.start(|| self.items.get(id)).await
    }

    pub async fn mount(&self, id: SkinId) -> Result<MountSkin, Error> {
        self.retry.start(|| self.mounts.get(id.into())).await
    }

    pub async fn glider(&self, id: SkinId) -> Result<Glider, Error> {
        self.retry.start(|| self.gliders.get(id.into())).await
    }

    pub async fn skiff(&self, id: SkinId) -> Result<Skiff, Error> {
        self.retry.start(|| self.skiffs.get(id.into())).await
    }

    pub async fn cache_wardrobe_templates<
        'a,
        Templates: IntoIterator<Item = &'a WardrobeTemplate>,
    >(
        &self,
        templates: Templates,
    ) -> Result<(), Error> {
        let mut skins = HashSet::new();
        let mut dyes = HashSet::new();
        for t in templates {
            skins.extend(t.all_skin_ids());
            dyes.extend(t.all_dye_ids());
        }
        self.fetch_missing_fashion_data(skins, dyes).await
    }

    pub async fn cache_wardrobe_template(&self, template: &WardrobeTemplate) -> Result<(), Error> {
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
    ) -> Result<(), Error> {
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
    ) -> Result<Vec<Equipment>, Error> {
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

    pub async fn resolve_template<S: FashionSlot>(
        &self,
        template: &TemplateData<S>,
    ) -> Result<TemplateData<S>, Error> {
        let mut slots = HashMap::with_capacity(template.len());
        for (slot, skin) in template {
            slots.insert(
                *slot,
                match slot.kind() {
                    FashionSlotKind::Equipment => self.resolve_wardrobe_skin(skin).await?,
                    FashionSlotKind::Outfit => self.resolve_outfit(skin).await?,
                    FashionSlotKind::Mount => self.resolve_mount(skin).await?,
                    FashionSlotKind::Glider => self.resolve_glider(skin).await?,
                    FashionSlotKind::Skiff => self.resolve_skiff(skin).await?,
                    FashionSlotKind::Doorway => self.resolve_doorway(skin).await?,
                },
            );
        }
        Ok(TemplateData::new(slots))
    }

    async fn resolve_outfit(&self, skin: &skin::Skin) -> Result<skin::Skin, Error> {
        let (name, dyes) = tokio::try_join!(
            self.resolve_outfit_name(skin.id),
            self.resolve_dyes(&skin.dyes),
        )?;
        Ok(skin::Skin {
            name,
            dyes,
            ..*skin
        })
    }

    async fn resolve_wardrobe_skin(&self, skin: &skin::Skin) -> Result<skin::Skin, Error> {
        let (name, dyes) = tokio::try_join!(
            self.resolve_skin_name(skin.id),
            self.resolve_dyes(&skin.dyes),
        )?;
        Ok(skin::Skin {
            name,
            dyes,
            ..*skin
        })
    }

    async fn resolve_mount(&self, skin: &skin::Skin) -> Result<skin::Skin, Error> {
        let (name, dyes) = tokio::try_join!(
            self.resolve_mount_name(skin.id),
            self.resolve_dyes(&skin.dyes),
        )?;
        Ok(skin::Skin {
            name,
            dyes,
            ..*skin
        })
    }

    async fn resolve_glider(&self, skin: &skin::Skin) -> Result<skin::Skin, Error> {
        let (name, dyes) = tokio::try_join!(
            self.resolve_glider_name(skin.id),
            self.resolve_dyes(&skin.dyes),
        )?;
        Ok(skin::Skin {
            name,
            dyes,
            ..*skin
        })
    }

    async fn resolve_skiff(&self, skin: &skin::Skin) -> Result<skin::Skin, Error> {
        let (name, dyes) = tokio::try_join!(
            self.resolve_skiff_name(skin.id),
            self.resolve_dyes(&skin.dyes),
        )?;
        Ok(skin::Skin {
            name,
            dyes,
            ..*skin
        })
    }

    async fn resolve_doorway(&self, skin: &skin::Skin) -> Result<skin::Skin, Error> {
        let (name, dyes) = tokio::try_join!(
            self.resolve_doorway_name(skin.id),
            self.resolve_dyes(&skin.dyes),
        )?;
        Ok(skin::Skin {
            name,
            dyes,
            ..*skin
        })
    }

    async fn resolve_outfit_name(&self, id: u16) -> Result<Option<String>, Error> {
        match self.outfit(id.into()).await {
            Ok(outfit) => Ok(Some(outfit.name)),
            Err(err) if is_not_found(&err) => {
                tracing::warn!(message = "could not resolve outfit", id = id);
                Ok(Some("Unknown".to_owned()))
            }
            Err(err) => Err(err),
        }
    }

    async fn resolve_skin_name(&self, id: u16) -> Result<Option<String>, Error> {
        match self.skin(id.into()).await {
            Ok(skin) => Ok(Some(skin.name)),
            Err(err) if is_not_found(&err) => {
                tracing::warn!(message = "could not resolve skin", id = id);
                Ok(Some("Unknown".to_owned()))
            }
            Err(err) => Err(err),
        }
    }

    async fn resolve_mount_name(&self, id: u16) -> Result<Option<String>, Error> {
        match self.mount(id.into()).await {
            Ok(mount) => Ok(Some(mount.name)),
            Err(err) if is_not_found(&err) => {
                tracing::warn!(message = "could not resolve mount skin", id = id);
                Ok(Some("Unknown".to_owned()))
            }
            Err(err) => Err(err),
        }
    }

    async fn resolve_glider_name(&self, id: u16) -> Result<Option<String>, Error> {
        match self.glider(id.into()).await {
            Ok(glider) => Ok(Some(glider.name)),
            Err(err) if is_not_found(&err) => {
                tracing::warn!(message = "could not resolve glider", id = id);
                Ok(Some("Unknown".to_owned()))
            }
            Err(err) => Err(err),
        }
    }

    async fn resolve_skiff_name(&self, id: u16) -> Result<Option<String>, Error> {
        match self.skiff(id.into()).await {
            Ok(skiff) => Ok(Some(skiff.name)),
            Err(err) if is_not_found(&err) => {
                tracing::warn!(message = "could not resolve skiff", id = id);
                Ok(Some("Unknown".to_owned()))
            }
            Err(err) => Err(err),
        }
    }

    async fn resolve_doorway_name(&self, _id: u16) -> Result<Option<String>, Error> {
        Ok(Some("Unknown".to_owned()))
    }

    async fn resolve_dyes(&self, dyes: &Option<skin::Dyes>) -> Result<Option<skin::Dyes>, Error> {
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

    async fn resolve_dye_name(&self, dye: &skin::Dye) -> Result<skin::Dye, Error> {
        let name = match self.dye(dye.id.into()).await {
            Ok(color) => Ok(color.name),
            Err(err) if is_not_found(&err) => {
                tracing::warn!(message = "could not resolve dye color", id = dye.id);
                Ok("Unknown".to_owned())
            }
            Err(err) => Err(err),
        }?;
        Ok(skin::Dye {
            name: Some(name),
            ..*dye
        })
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new(Client::default())
    }
}

fn is_not_found(err: &Error) -> bool {
    match err {
        Error::NotFound => true,
        _ => false,
    }
}
