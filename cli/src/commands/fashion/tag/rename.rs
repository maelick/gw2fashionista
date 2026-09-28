use gw2fashionista_core::domain::names::TagName;

use crate::{commands, environment::Environment};

#[derive(clap::Args, Debug)]
pub struct Command {
    /// Name of the tag to use as replacement.
    #[arg(required = true)]
    pub from: TagName,

    /// Name of the tag to use as replacement.
    #[arg(required = true)]
    pub to: TagName,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-tag-rename"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-tag-rename", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        service.rename_tag(&self.from, &self.to).await?;
        Ok(())
    }
}
