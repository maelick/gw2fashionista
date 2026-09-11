use anyhow::anyhow;
use gw2fashionista_chatlink::templates::{travel::TravelTemplate, wardrobe::WardrobeTemplate};
use gw2fashionista_core::{app::FashionService, domain::fashion::Fashion};
use gw2fashionista_storage::sqlite;

#[derive(clap::Args, Debug)]
pub struct FashionFields {
    /// Name of the fashion template.
    #[arg(short, long)]
    pub name: Option<String>,

    /// Associated character.
    #[arg(short, long, alias = "char")]
    pub character: Option<String>,

    /// Description of the fashion template.
    #[arg(short, long, value_name = "TEXT")]
    pub description: Option<String>,

    /// Chatlink of the wardrobe template.
    #[arg(long, value_name = "CHATLINK")]
    pub wardrobe: Option<WardrobeTemplate>,

    /// Chatlink of the travel template.
    #[arg(long, value_name = "CHATLINK")]
    pub travel: Option<TravelTemplate>,

    /// Tags
    #[arg(short, long = "tag", value_name = "TAG")]
    pub tags: Vec<String>,
}

impl TryFrom<&FashionFields> for Fashion {
    type Error = anyhow::Error;

    fn try_from(fields: &FashionFields) -> Result<Self, Self::Error> {
        let name = fields
            .name
            .clone()
            .ok_or(anyhow!("Fashion template name must be provided"))?;
        Ok(Fashion::builder()
            .name(name)
            .maybe_character(fields.character.clone())
            .maybe_description(fields.description.clone())
            .maybe_wardrobe_template(fields.wardrobe.clone())
            .maybe_travel_template(fields.travel.clone())
            .tags(fields.tags.clone())
            .build())
    }
}

#[derive(clap::Args, Debug)]
#[command(group(clap::ArgGroup::new("identifier").multiple(false)))]
pub struct FashionIdentifier {
    /// Id of the fashion template.
    #[arg(long, value_name = "UUID", group = "identifier")]
    pub id: Option<uuid::fmt::Hyphenated>,

    /// Name of the fashion template.
    #[arg(short, long, group = "identifier")]
    pub name: Option<String>,

    /// Associated character.
    #[arg(short, long, requires = "name", conflicts_with = "id", alias = "char")]
    pub character: Option<String>,
}

impl FashionIdentifier {
    pub async fn get_fashion(
        &self,
        service: &FashionService<sqlite::Repository>,
    ) -> anyhow::Result<Fashion> {
        Ok(match self.id {
            Some(id) => service.get_by_id(&id.into()).await?,
            None => match &self.name {
                Some(name) => {
                    let character = self.character.as_deref();
                    service.get_by_name(name, character).await?
                }
                None => anyhow::bail!("Missing fashion template id or name"), // Should never happen
            },
        })
    }
}
