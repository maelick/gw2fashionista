use std::{collections::HashMap, sync::LazyLock};

use linearize::StaticMap;
use serde::Deserialize;
use toml::value::Datetime;

use crate::{domain::templates::FashionSlotKind, gw2::named::StaticName};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    pub id: u32,
    pub name: String,
    pub wiki: String,
    pub last_checked: Datetime,
}

impl From<Entry> for StaticName {
    fn from(e: Entry) -> Self {
        StaticName {
            id: e.id,
            name: e.name,
        }
    }
}

pub fn skins(kind: FashionSlotKind) -> &'static HashMap<u32, StaticName> {
    &MISSING_SKINS[kind]
}

static MISSING_SKINS: LazyLock<StaticMap<FashionSlotKind, HashMap<u32, StaticName>>> =
    LazyLock::new(parse_all);

fn parse_all() -> StaticMap<FashionSlotKind, HashMap<u32, StaticName>> {
    StaticMap::from_fn(|kind: FashionSlotKind| parse_file(kind))
}

fn parse_file(kind: FashionSlotKind) -> HashMap<u32, StaticName> {
    let file: NamesFile = toml::from_str(read_file(kind)).unwrap();
    file.skins
        .into_iter()
        .map(|entry| {
            #[cfg(feature = "tracing")]
            tracing::debug!(
                message = "Using fallback skin",
                ?kind,
                id = entry.id,
                name = entry.name,
                wiki = entry.wiki,
                last_checked = ?entry.last_checked
            );
            (entry.id, entry.into())
        })
        .collect()
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
    skins: Vec<Entry>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_data_parses() {
        for (_, file_content) in StaticMap::from_fn(|kind: FashionSlotKind| read_file(kind)).iter()
        {
            let entries: NamesFile = toml::from_str(file_content).unwrap();
            for entry in entries.skins {
                assert!(entry.id > 0);
                assert!(!entry.name.is_empty());
                assert!(!entry.wiki.is_empty());
                assert!(!entry.last_checked.date.unwrap().year >= 2026);
            }
        }
    }
}
