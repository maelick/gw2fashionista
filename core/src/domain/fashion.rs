use crate::domain::templates::{travel::TravelTemplate, wardrobe::WardrobeTemplate};

pub struct Fashion {
    pub name: String,
    pub description: Option<String>,
    pub character: Option<String>,
    pub wardrobe_template: Option<WardrobeTemplate>,
    pub travel_template: Option<TravelTemplate>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub tags: Vec<String>,
}

impl Fashion {
    pub fn new(
        name: String,
        description: Option<String>,
        character: Option<String>,
        wardrobe_template: Option<WardrobeTemplate>,
        travel_template: Option<TravelTemplate>,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
        tags: &[String],
    ) -> Self {
        Self {
            name,
            description,
            character,
            wardrobe_template,
            travel_template,
            created_at,
            updated_at,
            tags: tags.to_vec(),
        }
    }
}
