use chrono::{DateTime, Utc};

use crate::domain::templates::{travel::TravelTemplate, wardrobe::WardrobeTemplate};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Fashion {
    pub id: Option<uuid::Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub character: Option<String>,
    pub wardrobe_template: Option<WardrobeTemplate>,
    pub travel_template: Option<TravelTemplate>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
}

impl Fashion {
    pub fn new(
        name: String,
        description: Option<String>,
        character: Option<String>,
        wardrobe_template: Option<WardrobeTemplate>,
        travel_template: Option<TravelTemplate>,
        tags: &[String],
    ) -> Self {
        Self {
            id: None,
            name,
            description,
            character,
            wardrobe_template,
            travel_template,
            created_at: None,
            updated_at: None,
            tags: tags.to_vec(),
        }
    }
}
