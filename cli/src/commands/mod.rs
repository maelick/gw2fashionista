use crate::environment::Environment;

pub mod args;
mod fashion;
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
    /// Fashion template commands
    Fashion(fashion::Args),
}

impl Commands {
    pub fn as_command(&self) -> &dyn Command {
        match self {
            Commands::Read(cmd) => cmd,
            Commands::Wardrobe(args) => args.command(),
            Commands::Travel(args) => args.command(),
            Commands::Fashion(args) => args.command(),
        }
    }

    pub async fn execute(&self, env: Environment) -> anyhow::Result<()> {
        match self {
            Commands::Read(cmd) => cmd.execute().await,
            Commands::Wardrobe(args) => args.execute(env).await,
            Commands::Travel(args) => args.execute().await,
            Commands::Fashion(args) => args.execute(env).await,
        }
    }
}

pub trait Command: std::fmt::Debug {
    fn name(&self) -> &str;
}
