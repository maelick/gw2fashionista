use async_trait::async_trait;
use clap::Args;

use gw2fashionista_chatlink::domain::chatlink::ChatLink;
use gw2fashionista_chatlink::domain::templates::wardrobe::WardrobeTemplate;

use crate::commands;
use crate::commands::args;
use crate::commands::wardrobe::args::WardrobeFilters;

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the base fashion template to override
    base_wardrobe_template: WardrobeTemplate,

    /// Chat link of the fashion template with new values to apply to the base one
    new_wardrobe_template: WardrobeTemplate,

    #[command(flatten)]
    skin_dyes_only: args::SkinsOrDyes,

    #[command(flatten)]
    filters: WardrobeFilters,
}

#[async_trait]
impl commands::Command for Command {
    fn name(&self) -> &str {
        "wardrobe-merge"
    }

    #[tracing::instrument(name = "wardrobe-merge", skip_all)]
    async fn execute(&self) -> anyhow::Result<()> {
        let filter = (&self.filters).into();
        let new_template = self.new_wardrobe_template.filter(&filter);
        let merged = self.base_wardrobe_template.merge(
            &new_template,
            self.skin_dyes_only.no_skins,
            self.skin_dyes_only.no_dyes,
        );

        println!("{}", ChatLink::WardrobeTemplate(merged));
        Ok(())
    }
}
