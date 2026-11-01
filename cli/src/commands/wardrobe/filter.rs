use std::io;

use clap::Args;
use gw2fashionista_chatlink::templates::SlotFilter;
use gw2fashionista_chatlink::templates::wardrobe::WardrobeSlot;

use crate::commands::wardrobe::args::WardrobeFilters;
use crate::{commands, input::read_parseable};

use gw2fashionista_chatlink::{ChatLink, templates::wardrobe::WardrobeTemplate};

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the wardrobe template to filter
    wardrobe_template: Option<WardrobeTemplate>,

    #[command(flatten)]
    filters: WardrobeFilters,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "wardrobe-filter"
    }
}

impl Command {
    #[tracing::instrument(level = "error", name = "wardrobe-filter", skip_all)]
    pub async fn execute(&self) -> anyhow::Result<()> {
        let filter = (&self.filters).into();
        let filtered = ChatLink::WardrobeTemplate(self.filter_template(&filter)?);
        println!("{}", filtered);
        Ok(())
    }

    fn filter_template(
        &self,
        filter: &SlotFilter<WardrobeSlot>,
    ) -> anyhow::Result<WardrobeTemplate> {
        if let Some(template) = &self.wardrobe_template {
            Ok(template.filter(filter))
        } else {
            let template: WardrobeTemplate = read_parseable(io::stdin().lock(), true)?;
            Ok(template.filter(filter))
        }
    }
}
