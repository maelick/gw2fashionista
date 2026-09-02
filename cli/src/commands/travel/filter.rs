use async_trait::async_trait;
use clap::Args;

use crate::commands;
use crate::commands::travel::args::TravelFilters;

use gw2fashionista_core::domain::{chatlink::ChatLink, templates::travel::TravelTemplate};

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the travel template to filter
    travel_template: String,

    #[command(flatten)]
    filters: TravelFilters,
}

#[async_trait]
impl commands::Command for Command {
    fn name(&self) -> &str {
        "travel-filter"
    }

    #[tracing::instrument(name = "travel-filter", skip_all)]
    async fn execute(&self) -> anyhow::Result<()> {
        let template: TravelTemplate = self.travel_template.parse()?;
        let filter = (&self.filters).into();
        let filtered = ChatLink::TravelTemplate(template.filter(&filter));
        println!("{}", filtered.to_string()?);
        Ok(())
    }
}
