use bon::Builder;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Eq, PartialEq, Builder)]
pub struct Tag {
    #[builder(into)]
    pub id: Option<uuid::Uuid>,

    #[builder(into)]
    pub name: String,

    pub created_at: Option<DateTime<Utc>>,

    pub updated_at: Option<DateTime<Utc>>,
}
