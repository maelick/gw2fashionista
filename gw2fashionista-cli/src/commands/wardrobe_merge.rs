use clap::{Args};

use super::args;
use gw2fashionista_core::domain::{chatlink::ChatLink, error::ChatLinkError};

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the base fashion template to override
    base_wardrobe_template: String,

    /// Chat link of the fashion template with new values to apply to the base one
    new_wardrobe_template: String,

    #[command(flatten)]
    skin_dyes_only: args::SkinsOrDyes,

    #[command(flatten)]
    filters: args::EquipmentFilters,
}

impl super::Command for Command {
    fn name(&self) -> &str {
        return "merge"
    }

    fn execute(&self) -> anyhow::Result<()> {
        let base_template = match ChatLink::try_from(self.base_wardrobe_template.as_str())? {
            ChatLink::WardrobeTemplate(wardrobe_template) => Ok(wardrobe_template),
            _ => Err(ChatLinkError::NotImplemented),
        }?;

        let filter = (&self.filters).into();
        let new_template = match ChatLink::try_from(self.new_wardrobe_template.as_str())? {
            ChatLink::WardrobeTemplate(wardrobe_template) => Ok(wardrobe_template),
            _ => Err(ChatLinkError::NotImplemented),
        }?;

        let new_template = new_template.filter(&filter);
        let merged = base_template.merge(&new_template, self.skin_dyes_only.no_skins, self.skin_dyes_only.no_dyes);

        println!("{}", ChatLink::WardrobeTemplate(merged).to_string()?);
        Ok(())
    }
}
