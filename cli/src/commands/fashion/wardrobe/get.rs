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
        "fashion-wardrobe-get"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-wardrobe-get", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        let fashion = self.id.get_fashion(&service).await?;
        let template = fashion.wardrobe_template.unwrap_or_default();
        println!("{}", template);
        Ok(())
    }
}
