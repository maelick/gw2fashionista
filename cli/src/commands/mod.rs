use async_trait::async_trait;

mod args;
mod read;
mod travel;
mod wardrobe;

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Read a chat linkand prints its content, potentially resolving ids by retrieving values from the GW2 API.
    Read(read::Command),
    /// Wardrobe template commands
    Wardrobe(wardrobe::Args),
    /// Travel template commands
    Travel(travel::Args),
}

impl Commands {
    pub fn as_command(&self) -> &dyn Command {
        match self {
            Commands::Read(cmd) => cmd,
            Commands::Wardrobe(args) => args.command(),
            Commands::Travel(args) => args.command(),
        }
    }
}

#[async_trait]
pub trait Command: std::fmt::Debug {
    fn name(&self) -> &str;
    async fn execute(&self) -> anyhow::Result<()>;
}
