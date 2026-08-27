use gw2fashionista_core::domain::{
    self,
    chatlink::ChatLink,
    error::ChatLinkError,
    templates::{FashionSlot, Template},
};
use sqlx::types::{
    chrono::{DateTime, Utc},
    uuid,
};

#[derive(sqlx::FromRow, Debug, Clone, Eq, PartialEq)]
pub struct Fashion {
    pub id: uuid::fmt::Hyphenated,
    pub name: String,
    pub description: String,
    pub character: String,
    pub wardrobe_template: String,
    pub travel_template: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow, Debug, Clone, Eq, PartialEq)]
pub struct Tag {
    pub id: uuid::fmt::Hyphenated,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl TryFrom<Fashion> for domain::fashion::Fashion {
    type Error = crate::Error;

    fn try_from(model: Fashion) -> Result<Self, Self::Error> {
        Ok(domain::fashion::Fashion::builder()
            .id(model.id)
            .name(model.name)
            .maybe_description(non_empty(model.description))
            .maybe_character(non_empty(model.character))
            .wardrobe_template(parse_template(&model.wardrobe_template)?)
            .travel_template(parse_template(&model.travel_template)?)
            .maybe_created_at(model.created_at)
            .maybe_updated_at(model.updated_at)
            .build())
    }
}

impl From<&domain::fashion::Fashion> for Fashion {
    fn from(fashion: &domain::fashion::Fashion) -> Self {
        Fashion {
            id: fashion.id.unwrap_or_else(|| uuid::Uuid::now_v7()).into(),
            name: fashion.name.clone(),
            description: fashion.description.clone().unwrap_or_default(),
            character: fashion.character.clone().unwrap_or_default(),
            wardrobe_template: serialize_template(fashion.wardrobe_template.clone()),
            travel_template: serialize_template(fashion.travel_template.clone()),
            created_at: fashion.created_at,
            updated_at: fashion.updated_at,
        }
    }
}

fn parse_template<S: FashionSlot>(s: &str) -> crate::Result<Template<S>>
where
    Template<S>: Default + TryFrom<ChatLink, Error = ChatLinkError>,
{
    Ok(if s.is_empty() {
        Template::<S>::default()
    } else {
        ChatLink::from_string(s)?.try_into()?
    })
}

fn serialize_template<S: FashionSlot>(template: Option<Template<S>>) -> String
where
    ChatLink: From<Template<S>>,
{
    template
        .filter(|t| !t.is_empty())
        .map(|t| {
            ChatLink::from(t)
                .to_string()
                .expect("ChatLink serialization should be infallible for supported FashionSlot")
        })
        .unwrap_or_default()
}

fn non_empty(s: String) -> Option<String> {
    if s.is_empty() { None } else { Some(s) }
}
