use bon::Builder;
use chrono::{DateTime, Utc};

use gw2fashionista_chatlink::templates::{travel::TravelTemplate, wardrobe::WardrobeTemplate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Builder, Deserialize, Serialize)]
pub struct Fashion {
    #[builder(into)]
    pub id: Option<uuid::Uuid>,

    #[builder(into)]
    pub name: String,

    #[builder(into)]
    pub description: Option<String>,

    #[builder(into)]
    pub character: Option<String>,

    #[serde(default, with = "display_fromstr_option")]
    pub wardrobe_template: Option<WardrobeTemplate>,

    #[serde(default, with = "display_fromstr_option")]
    pub travel_template: Option<TravelTemplate>,

    pub created_at: Option<DateTime<Utc>>,

    pub updated_at: Option<DateTime<Utc>>,

    #[builder(default, into)]
    #[serde(default)]
    pub tags: Vec<String>,
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

#[derive(Deserialize, Serialize)]
pub struct FashionRecord {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub character: Option<String>,
    #[serde(default, with = "display_fromstr_option")]
    pub wardrobe_template: Option<WardrobeTemplate>,
    #[serde(default, with = "display_fromstr_option")]
    pub travel_template: Option<TravelTemplate>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub tags: Tags,
}

impl From<FashionRecord> for Fashion {
    fn from(record: FashionRecord) -> Self {
        Self {
            id: record.id,
            name: record.name,
            description: record.description,
            character: record.character,
            wardrobe_template: record.wardrobe_template,
            travel_template: record.travel_template,
            created_at: record.created_at,
            updated_at: record.updated_at,
            tags: record.tags.into(),
        }
    }
}

#[derive(Serialize)]
pub struct FashionRecordRef<'a> {
    pub id: Option<&'a uuid::Uuid>,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub character: Option<&'a str>,
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
            character: fashion.character.as_deref(),
            wardrobe_template: fashion.wardrobe_template.as_ref(),
            travel_template: fashion.travel_template.as_ref(),
            created_at: fashion.created_at.as_ref(),
            updated_at: fashion.updated_at.as_ref(),
            tags: Tags(fashion.tags.join(",")),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Tags(String);

impl From<Tags> for Vec<String> {
    fn from(Tags(tags): Tags) -> Self {
        tags.split(",")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }
}

impl Default for Tags {
    fn default() -> Self {
        Self(Default::default())
    }
}
