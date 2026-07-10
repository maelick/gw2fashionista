use async_trait::async_trait;
use clap::Args;

use crate::commands;
use crate::commands::wardrobe::args::WardrobeFilters;

use gw2fashionista_core::domain::{chatlink::ChatLink, error::ChatLinkError};

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the wardrobe template to filter
    wardrobe_template: String,

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
        let link = ChatLink::try_from(self.wardrobe_template.as_str())?;
        let template = match link {
            ChatLink::WardrobeTemplate(wardrobe_template) => Ok(wardrobe_template),
            _ => Err(ChatLinkError::NotImplemented),
        }?;

        let filter = (&self.filters).into();
        let filtered = ChatLink::WardrobeTemplate(template.filter(&filter));
        println!("{}", filtered.to_string()?);
        Ok(())
    }
}
