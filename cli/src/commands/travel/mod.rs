use crate::commands::Command;

pub mod args;
pub mod filter;
pub mod merge;

#[derive(clap::Args, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

impl Args {
    pub(crate) fn command(&self) -> &dyn Command {
        match &self.command {
            Commands::Merge(cmd) => cmd,
            Commands::Filter(cmd) => cmd,
        }
    }

    pub async fn execute(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::Merge(cmd) => cmd.execute().await,
            Commands::Filter(cmd) => cmd.execute().await,
        }
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Merge two travel templates by overriding specific parts of the first one with values of the second one
    Merge(merge::Command),
    /// Filter a travel template to include only specific parts.
    Filter(filter::Command),
}
