use async_trait::async_trait;
use clap::Args;

use gw2fashionista_core::domain::{chatlink::ChatLink, error::ChatLinkError};

use crate::commands;
use crate::commands::args;
use crate::commands::travel::args::TravelFilters;

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the base fashion template to override
    base_travel_template: String,

    /// Chat link of the fashion template with new values to apply to the base one
    new_travel_template: String,

    #[command(flatten)]
    skin_dyes_only: args::SkinsOrDyes,

    #[command(flatten)]
    filters: TravelFilters,
}

#[async_trait]
impl commands::Command for Command {
    fn name(&self) -> &str {
        "travel-merge"
    }

    #[tracing::instrument(name = "travel-merge", skip_all)]
    async fn execute(&self) -> anyhow::Result<()> {
        let base_template = match ChatLink::try_from(self.base_travel_template.as_str())? {
            ChatLink::TravelTemplate(travel_template) => Ok(travel_template),
            _ => Err(ChatLinkError::NotImplemented),
        }?;

        let filter = (&self.filters).into();
        let new_template = match ChatLink::try_from(self.new_travel_template.as_str())? {
            ChatLink::TravelTemplate(travel_template) => Ok(travel_template),
            _ => Err(ChatLinkError::NotImplemented),
        }?;

        let new_template = new_template.filter(&filter);
        let merged = base_template.merge(
            &new_template,
            self.skin_dyes_only.no_skins,
            self.skin_dyes_only.no_dyes,
        );

        println!("{}", ChatLink::TravelTemplate(merged).to_string()?);
        Ok(())
    }
}
