use async_trait::async_trait;
use clap::Args;

use crate::commands;
use crate::commands::wardrobe::args::WardrobeFilters;

use gw2fashionista_chatlink::{ChatLink, templates::wardrobe::WardrobeTemplate};

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the wardrobe template to filter
    wardrobe_template: WardrobeTemplate,

    #[command(flatten)]
    filters: WardrobeFilters,
}

#[async_trait]
impl commands::Command for Command {
    fn name(&self) -> &str {
        "wardrobe-filter"
    }

    #[tracing::instrument(name = "wardrobe-filter", skip_all)]
    async fn execute(&self) -> anyhow::Result<()> {
        let filter = (&self.filters).into();
        let filtered = ChatLink::WardrobeTemplate(self.wardrobe_template.filter(&filter));
        println!("{}", filtered);
        Ok(())
    }
}
