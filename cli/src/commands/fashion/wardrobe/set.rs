use std::io;

use crate::{
    commands::{self, fashion::args::FashionIdentifier},
    environment::Environment,
    input::read_parseable,
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
        "fashion-wardrobe-set"
    }
}
impl Command {
    #[tracing::instrument(name = "fashion-wardrobe-set", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let template = read_parseable(io::stdin().lock(), true)?;
        let service = env.fashion_service().await?;
        let mut fashion = self.id.get_fashion(&service).await?;
        fashion.wardrobe_template = Some(template);
        service.set(&fashion).await?;
        Ok(())
    }
}
