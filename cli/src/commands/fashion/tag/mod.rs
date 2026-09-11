use crate::{commands::Command, environment::Environment};

mod clean;
mod list;

#[derive(clap::Args, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

impl Args {
    pub(crate) fn command(&self) -> &dyn Command {
        match &self.command {
            Commands::List(cmd) => cmd,
            Commands::Clean(cmd) => cmd,
        }
    }

    pub async fn execute(&self, env: Environment) -> anyhow::Result<()> {
        match &self.command {
            Commands::List(cmd) => cmd.execute(env).await,
            Commands::Clean(cmd) => cmd.execute(env).await,
        }
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// List existing tags.
    #[command(visible_alias = "ls")]
    List(list::Command),
    /// Remove unused tags.
    Clean(clean::Command),
}
