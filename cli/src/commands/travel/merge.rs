use async_trait::async_trait;
use clap::Args;

use gw2fashionista_core::domain::chatlink::ChatLink;
use gw2fashionista_core::domain::templates::travel::TravelTemplate;

use crate::commands;
use crate::commands::args;
use crate::commands::travel::args::TravelFilters;

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the base fashion template to override
    base_travel_template: TravelTemplate,

    /// Chat link of the fashion template with new values to apply to the base one
    new_travel_template: TravelTemplate,

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
        let filter = (&self.filters).into();
        let new_template = self.new_travel_template.filter(&filter);
        let merged = self.base_travel_template.merge(
            &new_template,
            self.skin_dyes_only.no_skins,
            self.skin_dyes_only.no_dyes,
        );

        println!("{}", ChatLink::TravelTemplate(merged));
        Ok(())
    }
}
