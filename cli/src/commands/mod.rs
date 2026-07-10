use async_trait::async_trait;
use clap::{Args, Subcommand};

mod args;
mod read;
mod travel;
mod wardrobe;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Read a chat linkand prints its content, potentially resolving ids by retrieving values from the GW2 API.
    Read(read::Command),
    /// Wardrobe template commands
    Wardrobe(WardrobeArgs),
    /// Travel template commands
    Travel(TravelArgs),
}

#[derive(Args, Debug)]
pub struct WardrobeArgs {
    #[command(subcommand)]
    command: WardrobeCommands,
}

#[derive(Subcommand, Debug)]
pub enum WardrobeCommands {
    /// Export equipment tabs as wardrobe templates using the API.
    Export(wardrobe::export::Command),
    /// Merge two wardrobe templates by overriding specific parts of the first one with values of the second one
    Merge(wardrobe::merge::Command),
    /// Filter a wardrobe template to include only specific parts.
    Filter(wardrobe::filter::Command),
}

#[derive(Args, Debug)]
pub struct TravelArgs {
    #[command(subcommand)]
    command: TravelCommands,
}

#[derive(Subcommand, Debug)]
pub enum TravelCommands {
    /// Merge two travel templates by overriding specific parts of the first one with values of the second one
    Merge(travel::merge::Command),
    /// Filter a travel template to include only specific parts.
    Filter(travel::filter::Command),
}

impl Commands {
    pub fn as_command(&self) -> &dyn Command {
        match self {
            Commands::Read(cmd) => cmd,
            Commands::Wardrobe(args) => match &args.command {
                WardrobeCommands::Export(cmd) => cmd,
                WardrobeCommands::Merge(cmd) => cmd,
                WardrobeCommands::Filter(cmd) => cmd,
            },
            Commands::Travel(args) => match &args.command {
                TravelCommands::Merge(cmd) => cmd,
                TravelCommands::Filter(cmd) => cmd,
            },
        }
    }
}

#[async_trait]
pub trait Command: std::fmt::Debug {
    fn name(&self) -> &str;
    async fn execute(&self) -> anyhow::Result<()>;
}
