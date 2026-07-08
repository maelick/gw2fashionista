use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use futures::stream::{self, StreamExt, TryStreamExt};
use gw2lib::model::authenticated::characters::Equip;
use gw2lib::model::{
    items::{Item, skins::Skin},
    misc::colors::Color,
};
use gw2lib::{Client, Requester};
use linearize::StaticMap;

use crate::domain::skins::{DyeId, SkinId};
use crate::domain::templates::{FashionSlot, FashionSlotKind, Template};
use crate::gw2::cache::Cache;
use crate::gw2::doorway::Doorway;
use crate::gw2::endpoints::glider::Glider;
use crate::gw2::endpoints::mount::MountSkin;
use crate::gw2::endpoints::outfit::Outfit;
use crate::gw2::endpoints::skiff::Skiff;
use crate::gw2::equipment::Equipment;
use crate::gw2::error::Error;
use crate::gw2::fetch::{Fetch, Gw2LibFetcher, Retry};
use crate::gw2::lookup::Lookup;
use crate::gw2::named::Named;
use crate::models::skin;
use crate::models::template::TemplateData;

const DEFAULT_BUFFER_SIZE: usize = 10;

pub struct Resolver {
    items: Cache<Item, u32>,
    colors: Cache<Color, u16>,
    fashion_lookup: StaticMap<FashionSlotKind, Box<dyn Lookup<u32> + Send + Sync>>,
    buffer_size: usize,
}

impl Resolver {
    pub fn new<Req>(req: Req) -> Self
    where
        Req: Requester<false, false> + Send + Sync + 'static,
    {
        Self::from_fetcher(Retry::new(Gw2LibFetcher::new(Arc::new(req))))
    }

    pub fn from_fetcher<F: FashionFetch>(fetcher: F) -> Self {
        Resolver {
            items: Cache::new(fetcher.clone()),
            colors: Cache::new(fetcher.clone()),
            fashion_lookup: StaticMap::from_fn(|kind| Self::lookup_for(kind, &fetcher)),
            buffer_size: DEFAULT_BUFFER_SIZE,
        }
    }

    fn lookup_for<F: FashionFetch>(
        kind: FashionSlotKind,
        fetcher: &F,
    ) -> Box<dyn Lookup<u32> + Send + Sync> {
        match kind {
            FashionSlotKind::Equipment => Self::api_lookup::<Skin, _>(fetcher),
            FashionSlotKind::Outfit => Self::api_lookup::<Outfit, _>(fetcher),
            FashionSlotKind::Mount => Self::api_lookup::<MountSkin, _>(fetcher),
            FashionSlotKind::Glider => Self::api_lookup::<Glider, _>(fetcher),
            FashionSlotKind::Skiff => Self::api_lookup::<Skiff, _>(fetcher),
            FashionSlotKind::Doorway => Box::new(Doorway::lookup()),
        }
    }

    fn api_lookup<T, F>(fetcher: &F) -> Box<dyn Lookup<u32> + Send + Sync>
    where
        T: Named + Clone + Send + Sync + 'static,
        F: Fetch<T, u32> + Clone + Send + Sync + 'static,
    {
        Box::new(Cache::new(fetcher.clone()))
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    pub fn clear(&self) {
        self.items.clear();
        self.colors.clear();
        for lookup in self.fashion_lookup.values() {
            lookup.clear();
        }
    }

    pub async fn cache_templates<
        'a,
        S: FashionSlot,
        Templates: IntoIterator<Item = &'a Template<S>>,
    >(
        &self,
        templates: Templates,
    ) -> Result<(), Error> {
        let mut skins = HashMap::<FashionSlotKind, HashSet<SkinId>>::new();
        let mut dyes = HashSet::new();
        for t in templates {
            for (kind, template_skins) in t.all_skin_ids() {
                skins.entry(kind).or_default().extend(template_skins);
            }
            dyes.extend(t.all_dye_ids());
        }
        self.cache_fashion_data(skins, dyes).await
    }

    pub async fn cache_template<S: FashionSlot>(
        &self,
        template: &Template<S>,
    ) -> Result<(), Error> {
        let skins = template.all_skin_ids();
        let dyes = template.all_dye_ids();
        self.cache_fashion_data(skins, dyes).await
    }

    async fn cache_fashion_data<
        Skins: IntoIterator<Item = (FashionSlotKind, HashSet<SkinId>)>,
        Dyes: IntoIterator<Item = DyeId>,
    >(
        &self,
        skins: Skins,
        dyes: Dyes,
    ) -> Result<(), Error> {
        tokio::try_join!(self.cache_skins(skins), self.colors.ensure(dyes),)?;
        Ok(())
    }

    async fn cache_skins<Skins: IntoIterator<Item = (FashionSlotKind, HashSet<SkinId>)>>(
        &self,
        skins: Skins,
    ) -> Result<(), Error> {
        for (kind, skins) in skins {
            self.fashion_lookup[kind]
                .ensure(skins.into_iter().map(Into::into).collect())
                .await?
        }
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
        self.items.ensure(items).await?;

        stream::iter(equipments)
            .map(async |e| {
                self.resolve_default_skins(&e.slots)
                    .await
                    .map(|s| e.with_slots(s))
            })
            .buffered(self.buffer_size)
            .try_collect()
            .await
    }

    pub async fn resolve_default_skins(&self, slots: &[Equip]) -> Result<Vec<Equip>, Error> {
        stream::iter(slots)
            .then(async |s| {
                if s.skin.is_none() {
                    Ok::<_, Error>(Equip {
                        skin: self.items.get(s.id).await?.default_skin,
                        ..s.clone()
                    })
                } else {
                    Ok(s.clone())
                }
            })
            .try_collect()
            .await
    }

    pub async fn resolve_template<S: FashionSlot>(
        &self,
        template: &TemplateData<S>,
    ) -> Result<TemplateData<S>, Error> {
        let mut slots = HashMap::with_capacity(template.len());
        for (slot, skin) in template {
            let resolved = self.resolve_skin(slot.kind(), skin).await?;
            slots.insert(*slot, resolved);
        }
        Ok(TemplateData::new(slots))
    }

    async fn resolve_skin(
        &self,
        kind: FashionSlotKind,
        skin: &skin::Skin,
    ) -> Result<skin::Skin, Error> {
        let (name, dyes) = tokio::try_join!(
            self.fashion_lookup[kind].resolve_name(skin.id.into()),
            self.resolve_dyes(&skin.dyes),
        )?;
        Ok(skin::Skin {
            name,
            dyes,
            ..*skin
        })
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

pub trait FashionFetch:
    Fetch<Item, u32>
    + Fetch<Skin, u32>
    + Fetch<Outfit, u32>
    + Fetch<Color, u16>
    + Fetch<MountSkin, u32>
    + Fetch<Glider, u32>
    + Fetch<Skiff, u32>
    + Clone
    + Send
    + Sync
    + 'static
{
}

impl<F> FashionFetch for F where
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
        + 'static
{
}
