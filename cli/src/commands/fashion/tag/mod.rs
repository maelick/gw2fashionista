use crate::{commands::Command, environment::Environment};

mod clean;
mod delete;
mod list;
mod rename;
mod replace;

#[derive(clap::Args, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

impl Args {
    pub(crate) fn command(&self) -> &dyn Command {
        match &self.command {
            Commands::List(cmd) => cmd,
            Commands::Rename(cmd) => cmd,
            Commands::Replace(cmd) => cmd,
            Commands::Delete(cmd) => cmd,
            Commands::Clean(cmd) => cmd,
        }
    }

    pub async fn execute(&self, env: Environment) -> anyhow::Result<()> {
        match &self.command {
            Commands::List(cmd) => cmd.execute(env).await,
            Commands::Rename(cmd) => cmd.execute(env).await,
            Commands::Replace(cmd) => cmd.execute(env).await,
            Commands::Delete(cmd) => cmd.execute(env).await,
            Commands::Clean(cmd) => cmd.execute(env).await,
        }
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// List existing tags.
    #[command(visible_alias = "ls")]
    List(list::Command),
    /// Rename a tag.
    Rename(rename::Command),
    /// Replace tags with another one.
    Replace(replace::Command),
    /// Delete tags.
    Delete(delete::Command),
    /// Delete unused tags.
    Clean(clean::Command),
}
