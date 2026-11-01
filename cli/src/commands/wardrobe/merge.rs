use std::io;

use clap::Args;

use gw2fashionista_chatlink::ChatLink;
use gw2fashionista_chatlink::templates::SlotFilter;
use gw2fashionista_chatlink::templates::wardrobe::{WardrobeSlot, WardrobeTemplate};

use crate::commands;
use crate::commands::args;
use crate::commands::wardrobe::args::WardrobeFilters;
use crate::input::read_parseable;

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the base fashion template to override
    base_wardrobe_template: WardrobeTemplate,

    /// Chat link of the fashion template with new values to apply to the base one
    new_wardrobe_template: Option<WardrobeTemplate>,

    #[command(flatten)]
    skin_dyes_only: args::SkinsOrDyes,

    #[command(flatten)]
    filters: WardrobeFilters,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "wardrobe-merge"
    }
}

impl Command {
    #[tracing::instrument(level = "error", name = "wardrobe-merge", skip_all)]
    pub async fn execute(&self) -> anyhow::Result<()> {
        let filter = (&self.filters).into();
        let new_template = self.filter_new_template(&filter)?;
        let merged = self.base_wardrobe_template.merge(
            &new_template,
            self.skin_dyes_only.no_skins,
            self.skin_dyes_only.no_dyes,
        );

        println!("{}", ChatLink::WardrobeTemplate(merged));
        Ok(())
    }

    fn filter_new_template(
        &self,
        filter: &SlotFilter<WardrobeSlot>,
    ) -> anyhow::Result<WardrobeTemplate> {
        if let Some(template) = &self.new_wardrobe_template {
            Ok(template.filter(filter))
        } else {
            let template: WardrobeTemplate = read_parseable(io::stdin().lock(), true)?;
            Ok(template.filter(filter))
        }
    }
}
