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
        "fashion-travel-get"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-travel-get", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        let fashion = self.id.get_fashion(&service).await?;
        let template = fashion.travel_template.unwrap_or_default();
        println!("{}", template);
        Ok(())
    }
}
