use clap::{Parser};
use clap_verbosity_flag::{Verbosity, InfoLevel};

use gw2fashionista_cli::commands::{Commands};

#[derive(Parser)]
#[command(version, about = "GW2 Fashion Exporter CLI", long_about = None)]
struct Cli {
    #[command(flatten)]
    verbosity: Verbosity<InfoLevel>, // TODO: change to warn instead?

    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    fn init(&self) {
        env_logger::Builder::new()
            .filter_level(self.verbosity.into())
            .init();
    }

    fn execute(&self) {
        let cmd = self.command.as_command();
        log::debug!("Executing command {}: {:?}", cmd.name(), cmd);
        match cmd.execute() {
            Ok(_) => {
                log::debug!("Command {} successful", cmd.name())
            },
            Err(err) => {
                log::error!("Command {} error: {}", cmd.name(), err);
                std::process::exit(1)
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    cli.init();
    cli.execute();
}
