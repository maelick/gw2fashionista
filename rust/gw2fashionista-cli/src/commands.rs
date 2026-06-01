use clap::{Subcommand};

mod args;
pub mod export;
pub mod read;
pub mod merge;
pub mod filter;

#[derive(Subcommand)]
pub enum Commands {
    /// Export equipment tabs as fashion templates using the API.
    Export(export::Command),
    /// Read a fashion template and prints its content by retrieving values from the GW2 API.
    Read(read::Command),
    /// Merge two fashion templates by overriding specific parts of the first one with values of the second one
    Merge(merge::Command),
    /// Filter a fashion template to include only specific parts.
    Filter(filter::Command),
}

impl Commands {
    pub fn as_command(&self) -> &dyn Command {
        match self {
            Commands::Export(cmd) => cmd,
            Commands::Read(cmd) => cmd,
            Commands::Merge(cmd) => cmd,
            Commands::Filter(cmd) => cmd,
        }
    }
}

pub trait Command: std::fmt::Debug {
    fn name(&self) -> &str;
    fn execute(&self) -> anyhow::Result<()>;
}
