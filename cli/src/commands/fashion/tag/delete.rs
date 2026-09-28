use gw2fashionista_core::domain::names::TagName;

use crate::{commands, environment::Environment};

#[derive(clap::Args, Debug)]
pub struct Command {
    /// Name of the tag(s) to delete.
    pub tags: Vec<TagName>,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-tag-delete"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-tag-delete", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        let num_deleted = service.delete_tags(&self.tags).await?;
        tracing::info!("{} tags removed", num_deleted);
        Ok(())
    }
}
