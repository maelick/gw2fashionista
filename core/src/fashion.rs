use bon::Builder;
use chrono::{DateTime, Utc};

use gw2fashionista_chatlink::templates::{
    travel::TravelTemplate, wardrobe::WardrobeTemplate,
};

#[derive(Debug, Clone, Eq, PartialEq, Builder)]
pub struct Fashion {
    #[builder(into)]
    pub id: Option<uuid::Uuid>,

    #[builder(into)]
    pub name: String,

    #[builder(into)]
    pub description: Option<String>,

    #[builder(into)]
    pub character: Option<String>,

    pub wardrobe_template: Option<WardrobeTemplate>,

    pub travel_template: Option<TravelTemplate>,

    pub created_at: Option<DateTime<Utc>>,

    pub updated_at: Option<DateTime<Utc>>,

    #[builder(default, into)]
    pub tags: Vec<String>,
}
