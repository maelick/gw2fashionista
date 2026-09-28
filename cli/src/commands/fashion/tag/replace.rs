use gw2fashionista_core::domain::names::TagName;

use crate::{commands, environment::Environment};

#[derive(clap::Args, Debug)]
pub struct Command {
    /// Name of the tag(s) to replace.
    #[arg(required = true)]
    pub tags: Vec<TagName>,

    /// Name of the tag to use as replacement.
    #[arg(required = true)]
    pub with: TagName,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-tag-replace"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-tag-replace", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        service.replace_tags(&self.tags, &self.with).await?;
        Ok(())
    }
}
