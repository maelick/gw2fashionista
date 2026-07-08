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
use crate::gw2::cache::Cache;
use crate::gw2::endpoints::glider::Glider;
use crate::gw2::endpoints::mount::MountSkin;
use crate::gw2::endpoints::outfit::Outfit;
use crate::gw2::endpoints::skiff::Skiff;
use crate::gw2::equipment::Equipment;
use crate::gw2::error::Error;
use crate::gw2::fetch::{Fetch, Gw2LibFetcher, Retry};
use crate::models::skin;
use crate::models::template::TemplateData;

const DEFAULT_BUFFER_SIZE: usize = 10;

pub struct Resolver {
    items: Cache<Item, u32>,
    skins: Cache<Skin, u32>,
    outfits: Cache<Outfit, u32>,
    colors: Cache<Color, u16>,
    mounts: Cache<MountSkin, u32>,
    gliders: Cache<Glider, u32>,
    skiffs: Cache<Skiff, u32>,
    buffer_size: usize,
}

impl Resolver {
    pub fn new<Req>(req: Req) -> Self
    where
        Req: Requester<false, false> + Send + Sync + 'static,
    {
        Self::from_fetcher(Retry::new(Gw2LibFetcher::new(Arc::new(req))))
    }

    pub fn from_fetcher<F>(fetcher: F) -> Self
    where
        F: Fetch<Item, u32>
            + Fetch<Skin, u32>
            + Fetch<Outfit, u32>
            + Fetch<Color, u16>
            + Fetch<MountSkin, u32>
            + Fetch<Glider, u32>
            + Fetch<Skiff, u32>
            + Clone
            + Send
            + Sync
            + 'static,
    {
        Resolver {
            items: Cache::new(Box::new(fetcher.clone())),
            skins: Cache::new(Box::new(fetcher.clone())),
            outfits: Cache::new(Box::new(fetcher.clone())),
            colors: Cache::new(Box::new(fetcher.clone())),
            mounts: Cache::new(Box::new(fetcher.clone())),
            gliders: Cache::new(Box::new(fetcher.clone())),
            skiffs: Cache::new(Box::new(fetcher.clone())),
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
            self.outfits.resolve_name(skin.id.into()),
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
            self.skins.resolve_name(skin.id.into()),
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
            self.mounts.resolve_name(skin.id.into()),
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
            self.gliders.resolve_name(skin.id.into()),
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
            self.skiffs.resolve_name(skin.id.into()),
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

    async fn resolve_doorway_name(&self, _id: u16) -> Result<Option<String>, Error> {
        // It seems there is no API endpoint to get doorway data
        Ok(None)
    }

    async fn resolve_dyes(&self, dyes: &Option<skin::Dyes>) -> Result<Option<skin::Dyes>, Error> {
        if let Some((dye1, dye2, dye3, dye4)) = dyes {
            Ok(Some(tokio::try_join!(
                self.resolve_dye(dye1),
                self.resolve_dye(dye2),
                self.resolve_dye(dye3),
                self.resolve_dye(dye4),
            )?))
        } else {
            Ok(None)
        }
    }

    async fn resolve_dye(&self, dye: &skin::Dye) -> Result<skin::Dye, Error> {
        Ok(skin::Dye {
            name: self.colors.resolve_name(dye.id).await?,
            ..*dye
        })
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new(Client::default())
    }
}
