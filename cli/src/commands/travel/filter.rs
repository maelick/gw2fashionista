use clap::Args;

use crate::commands;
use crate::commands::travel::args::TravelFilters;

use gw2fashionista_chatlink::{ChatLink, templates::travel::TravelTemplate};

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the travel template to filter
    travel_template: TravelTemplate,

    #[command(flatten)]
    filters: TravelFilters,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "travel-filter"
    }
}

impl Command {
    #[tracing::instrument(name = "travel-filter", skip_all)]
    pub async fn execute(&self) -> anyhow::Result<()> {
        let filter = (&self.filters).into();
        let filtered = ChatLink::TravelTemplate(self.travel_template.filter(&filter));
        println!("{}", filtered);
        Ok(())
    }
}
