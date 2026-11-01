use std::io;

use clap::Args;

use gw2fashionista_chatlink::ChatLink;
use gw2fashionista_chatlink::templates::SlotFilter;
use gw2fashionista_chatlink::templates::travel::{TravelSlot, TravelTemplate};

use crate::commands;
use crate::commands::args;
use crate::commands::travel::args::TravelFilters;
use crate::input::read_parseable;

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the base fashion template to override
    base_travel_template: TravelTemplate,

    /// Chat link of the fashion template with new values to apply to the base one
    new_travel_template: Option<TravelTemplate>,

    #[command(flatten)]
    skin_dyes_only: args::SkinsOrDyes,

    #[command(flatten)]
    filters: TravelFilters,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "travel-merge"
    }
}

impl Command {
    #[tracing::instrument(level = "error", name = "travel-merge", skip_all)]
    pub async fn execute(&self) -> anyhow::Result<()> {
        let filter = (&self.filters).into();
        let new_template = self.filter_new_template(&filter)?;
        let merged = self.base_travel_template.merge(
            &new_template,
            self.skin_dyes_only.no_skins,
            self.skin_dyes_only.no_dyes,
        );

        println!("{}", ChatLink::TravelTemplate(merged));
        Ok(())
    }

    fn filter_new_template(
        &self,
        filter: &SlotFilter<TravelSlot>,
    ) -> anyhow::Result<TravelTemplate> {
        if let Some(template) = &self.new_travel_template {
            Ok(template.filter(filter))
        } else {
            let template: TravelTemplate = read_parseable(io::stdin().lock(), true)?;
            Ok(template.filter(filter))
        }
    }
}
