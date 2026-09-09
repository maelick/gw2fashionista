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
