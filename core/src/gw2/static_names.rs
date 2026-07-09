use std::{collections::HashMap, sync::LazyLock};

use linearize::StaticMap;
use serde::Deserialize;
use toml::value::Datetime;

use crate::{domain::templates::FashionSlotKind, gw2::named::Named};

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StaticName {
    pub id: u32,
    pub name: String,
    pub wiki: String,
    pub last_checked: Datetime,
}

impl Named for StaticName {
    fn name(&self) -> &str {
        &self.name
    }
}

pub fn missing(kind: FashionSlotKind) -> &'static HashMap<u32, StaticName> {
    &MISSING_SKINS[kind]
}

static MISSING_SKINS: LazyLock<StaticMap<FashionSlotKind, HashMap<u32, StaticName>>> =
    LazyLock::new(|| StaticMap::from_fn(|kind: FashionSlotKind| parse_file(read_file(kind))));

fn parse_file(s: &'static str) -> HashMap<u32, StaticName> {
    let file: NamesFile = toml::from_str(s).unwrap();
    file.skins.into_iter().map(|s| (s.id, s.clone())).collect()
}

pub fn read_file(kind: FashionSlotKind) -> &'static str {
    match kind {
        FashionSlotKind::Equipment => include_str!("../../data/names/skins.toml"),
        FashionSlotKind::Outfit => include_str!("../../data/names/outfits.toml"),
        FashionSlotKind::Mount => include_str!("../../data/names/mounts.toml"),
        FashionSlotKind::Glider => include_str!("../../data/names/gliders.toml"),
        FashionSlotKind::Skiff => include_str!("../../data/names/skiffs.toml"),
        FashionSlotKind::Doorway => include_str!("../../data/names/doorways.toml"),
    }
}

#[derive(Clone, Deserialize)]
struct NamesFile {
    #[serde(default)]
    skins: Vec<StaticName>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_data_parses() {
        LazyLock::force(&MISSING_SKINS);
    }
}
