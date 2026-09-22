use crate::{
    commands::{self, fashion::args::FashionIdentifier},
    environment::Environment,
};

#[derive(clap::Args, Debug)]
#[command(mut_group("identifier", |g| g.required(true)))]
pub struct Command {
    #[arg(from_global)]
    clipboard: bool,

    #[command(flatten)]
    id: FashionIdentifier,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-delete"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-delete", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        service.delete(&self.id.clone().into()).await?;
        Ok(())
    }
}
