use bon::Builder;
use chrono::{DateTime, Utc};
use itertools::Itertools;

use gw2fashionista_chatlink::templates::{travel::TravelTemplate, wardrobe::WardrobeTemplate};
use serde::{Deserialize, Serialize};

use crate::domain::{
    fashion::fashion_builder::{IsUnset, SetCharacter, SetName, SetTags, State},
    names::{
        CharacterName, CharacterNameError, FashionName, FashionNameError, TagName, TagNameError,
    },
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FashionIdentifier {
    Id(uuid::Uuid),
    NameAndCharacter {
        name: String,
        character: Option<String>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Builder, Deserialize, Serialize)]
pub struct Fashion {
    #[builder(into)]
    pub id: Option<uuid::Uuid>,

    pub name: FashionName,

    #[builder(into)]
    pub description: Option<String>,

    pub character: Option<CharacterName>,

    #[serde(default, with = "display_fromstr_option")]
    pub wardrobe_template: Option<WardrobeTemplate>,

    #[serde(default, with = "display_fromstr_option")]
    pub travel_template: Option<TravelTemplate>,

    pub created_at: Option<DateTime<Utc>>,

    pub updated_at: Option<DateTime<Utc>>,

    #[builder(default, into)]
    #[serde(default)]
    pub tags: Vec<TagName>,
}

impl<S: State> FashionBuilder<S> {
    pub fn name_str(self, s: &str) -> Result<FashionBuilder<SetName<S>>, FashionNameError>
    where
        S::Name: IsUnset,
    {
        let name = s.parse::<FashionName>()?;
        Ok(self.name(name))
    }

    pub fn character_str(
        self,
        s: &str,
    ) -> Result<FashionBuilder<SetCharacter<S>>, CharacterNameError>
    where
        S::Character: IsUnset,
    {
        let character = s.parse::<CharacterName>()?;
        Ok(self.character(character))
    }

    pub fn tags_str(self, s: &[&str]) -> Result<FashionBuilder<SetTags<S>>, TagNameError>
    where
        S::Tags: IsUnset,
    {
        let tags: Result<Vec<_>, _> = s.iter().map(|s| s.parse::<TagName>()).collect();
        Ok(self.tags(tags?))
    }
}

impl Fashion {
    pub fn with_id(mut self, id: uuid::Uuid) -> Self {
        self.id = Some(id);
        self
    }

    pub fn patch(mut self, other: &Fashion) -> Self {
        if let Some(id) = &other.id {
            self.id = Some(*id);
        }
        if !other.name.as_ref().is_empty() {
            self.name = other.name.clone();
        }
        if let Some(description) = &other.description {
            self.description = Some(description.clone());
        }
        if let Some(character) = &other.character {
            self.character = Some(character.clone());
        }
        if let Some(wardrobe_template) = &other.wardrobe_template {
            self.wardrobe_template = Some(wardrobe_template.clone());
        }
        if let Some(travel_template) = &other.travel_template {
            self.travel_template = Some(travel_template.clone());
        }
        if !other.tags.is_empty() {
            merge_tags(&mut self.tags, &other.tags);
        }
        self
    }
}

mod display_fromstr_option {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::{fmt::Display, str::FromStr};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        value.as_ref().map(T::to_string).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        Option::<String>::deserialize(deserializer)?
            .map(|s| s.parse().map_err(serde::de::Error::custom))
            .transpose()
    }
}

impl From<&Fashion> for FashionIdentifier {
    fn from(fashion: &Fashion) -> Self {
        if let Some(id) = fashion.id {
            FashionIdentifier::Id(id)
        } else {
            FashionIdentifier::NameAndCharacter {
                name: fashion.name.to_string(),
                character: fashion.character.as_ref().map(CharacterName::to_string),
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct FashionRecord {
    pub id: Option<uuid::Uuid>,
    pub name: FashionName,
    pub description: Option<String>,
    pub character: Option<CharacterName>,
    #[serde(default, with = "display_fromstr_option")]
    pub wardrobe_template: Option<WardrobeTemplate>,
    #[serde(default, with = "display_fromstr_option")]
    pub travel_template: Option<TravelTemplate>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub tags: Tags,
}

impl TryFrom<FashionRecord> for Fashion {
    type Error = TagNameError;
    fn try_from(record: FashionRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            id: record.id,
            name: record.name,
            description: record.description,
            character: record.character,
            wardrobe_template: record.wardrobe_template,
            travel_template: record.travel_template,
            created_at: record.created_at,
            updated_at: record.updated_at,
            tags: record.tags.try_into()?,
        })
    }
}

#[derive(Serialize)]
pub struct FashionRecordRef<'a> {
    pub id: Option<&'a uuid::Uuid>,
    pub name: &'a FashionName,
    pub description: Option<&'a str>,
    pub character: Option<&'a CharacterName>,
    #[serde(default, with = "display_fromstr_option")]
    pub wardrobe_template: Option<&'a WardrobeTemplate>,
    #[serde(default, with = "display_fromstr_option")]
    pub travel_template: Option<&'a TravelTemplate>,
    pub created_at: Option<&'a DateTime<Utc>>,
    pub updated_at: Option<&'a DateTime<Utc>>,
    #[serde(default)]
    pub tags: Tags,
}

impl<'a> From<&'a Fashion> for FashionRecordRef<'a> {
    fn from(fashion: &'a Fashion) -> Self {
        Self {
            id: fashion.id.as_ref(),
            name: &fashion.name,
            description: fashion.description.as_deref(),
            character: fashion.character.as_ref(),
            wardrobe_template: fashion.wardrobe_template.as_ref(),
            travel_template: fashion.travel_template.as_ref(),
            created_at: fashion.created_at.as_ref(),
            updated_at: fashion.updated_at.as_ref(),
            tags: Tags(
                Itertools::intersperse(fashion.tags.iter().map(|t| t.as_str()), ",").collect(),
            ),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tags(String);

impl Tags {
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0
            .split(",")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
    }
}

impl TryFrom<Tags> for Vec<TagName> {
    type Error = TagNameError;

    fn try_from(tags: Tags) -> Result<Self, Self::Error> {
        tags.iter().map(str::parse).collect()
    }
}

fn merge_tags(existing: &mut Vec<TagName>, new: &[TagName]) {
    let existing_set: std::collections::HashSet<_> = existing.iter().collect();
    let new_tags: Vec<TagName> = new
        .iter()
        .filter(|t| !existing_set.contains(t))
        .cloned()
        .collect();
    existing.extend(new_tags);
}
