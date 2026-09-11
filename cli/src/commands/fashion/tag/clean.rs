use crate::{commands, environment::Environment};

#[derive(clap::Args, Debug)]
pub struct Command {
    #[arg(from_global)]
    clipboard: bool,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-tag-clean"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-tag-clean", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        let num_cleaned = service.clean_tags().await?;
        tracing::info!("{} unused tags removed", num_cleaned);
        Ok(())
    }
}
