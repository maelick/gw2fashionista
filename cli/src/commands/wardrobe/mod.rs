use crate::commands::Command;

pub mod args;
pub mod export;
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
            Commands::Export(cmd) => cmd,
            Commands::Merge(cmd) => cmd,
            Commands::Filter(cmd) => cmd,
        }
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Export equipment tabs as wardrobe templates using the API.
    Export(export::Command),
    /// Merge two wardrobe templates by overriding specific parts of the first one with values of the second one
    Merge(merge::Command),
    /// Filter a wardrobe template to include only specific parts.
    Filter(filter::Command),
}
