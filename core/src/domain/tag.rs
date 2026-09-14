use bon::Builder;
use chrono::{DateTime, Utc};

use crate::domain::{
    names::{TagName, TagNameError},
    tag::tag_builder::{IsUnset, SetName, State},
};

#[derive(Debug, Clone, Eq, PartialEq, Builder, serde::Deserialize, serde::Serialize)]
pub struct Tag {
    #[builder(into)]
    pub id: Option<uuid::Uuid>,

    pub name: TagName,

    pub created_at: Option<DateTime<Utc>>,

    pub updated_at: Option<DateTime<Utc>>,
}

impl<S: State> TagBuilder<S> {
    pub fn name_str(self, s: &str) -> Result<TagBuilder<SetName<S>>, TagNameError>
    where
        S::Name: IsUnset,
    {
        let name = s.parse::<TagName>()?;
        Ok(self.name(name))
    }
}
