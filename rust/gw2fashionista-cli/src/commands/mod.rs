use clap::{Subcommand, Args};

mod args;
mod read;
mod wardrobe_export;
mod wardrobe_merge;
mod wardrobe_filter;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Read a chat linkand prints its content, potentially resolving ids by retrieving values from the GW2 API.
    Read(read::Command),
    /// Wardrobe template commands
    Wardrobe(WardrobeArgs)
}

#[derive(Args, Debug)]
pub struct WardrobeArgs {
    #[command(subcommand)]
    command: WardrobeCommands,
}

#[derive(Subcommand, Debug)]
pub enum WardrobeCommands {
    /// Export equipment tabs as wardrobe templates using the API.
    Export(wardrobe_export::Command),
    /// Merge two wardrobe templates by overriding specific parts of the first one with values of the second one
    Merge(wardrobe_merge::Command),
    /// Filter a wardrobe template to include only specific parts.
    Filter(wardrobe_filter::Command),
}

impl Commands {
    pub fn as_command(&self) -> &dyn Command {
        match self {
            Commands::Read(cmd) => cmd,
            Commands::Wardrobe(args) => match &args.command {
                WardrobeCommands::Export(cmd) => cmd,
                WardrobeCommands::Merge(cmd) => cmd,
                WardrobeCommands::Filter(cmd) => cmd,
            }
        }
    }
}

pub trait Command: std::fmt::Debug {
    fn name(&self) -> &str;
    fn execute(&self) -> anyhow::Result<()>;
}
