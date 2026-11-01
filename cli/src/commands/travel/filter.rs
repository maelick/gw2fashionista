use std::io;

use clap::Args;

use crate::commands::travel::args::TravelFilters;
use crate::{commands, input::read_parseable};

use gw2fashionista_chatlink::{
    ChatLink,
    templates::{
        SlotFilter,
        travel::{TravelSlot, TravelTemplate},
    },
};

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the travel template to filter
    travel_template: Option<TravelTemplate>,

    #[command(flatten)]
    filters: TravelFilters,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "travel-filter"
    }
}

impl Command {
    #[tracing::instrument(level = "error", name = "travel-filter", skip_all)]
    pub async fn execute(&self) -> anyhow::Result<()> {
        let filter = (&self.filters).into();
        let filtered = ChatLink::TravelTemplate(self.filter_template(&filter)?);
        println!("{}", filtered);
        Ok(())
    }

    fn filter_template(&self, filter: &SlotFilter<TravelSlot>) -> anyhow::Result<TravelTemplate> {
        if let Some(template) = &self.travel_template {
            Ok(template.filter(filter))
        } else {
            let template: TravelTemplate = read_parseable(io::stdin().lock(), true)?;
            Ok(template.filter(filter))
        }
    }
}
