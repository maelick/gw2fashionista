use std::io::{self, IsTerminal};

use clap::{Parser};
use clap_verbosity_flag::{Verbosity, InfoLevel};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

use gw2fashionista_cli::commands::Commands;

#[derive(Parser, Debug)]
#[command(version, about = "GW2 Fashion Exporter CLI", long_about = None)]
struct Cli {
    #[command(flatten)]
    verbosity: Verbosity<InfoLevel>,

    /// Output all logs (from external libraries)
    #[arg(long, global = true)]
    pub all_logs: bool,

    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    fn init(&self) {
        self.init_tracing(self.verbosity.into(), self.all_logs);
    }

    fn init_tracing(&self, level: LevelFilter, all_logs: bool) {
        let layer = tracing_subscriber::fmt::layer()
            .with_writer(tracing_writer)
            .with_filter(level)
            .with_filter(tracing_subscriber::filter::filter_fn(move |metadata| {
                all_logs || metadata.target().starts_with("gw2fashionista")
            }));
        tracing_subscriber::registry().with(layer).init()
    }

    #[tracing::instrument(name = "gw2fashionista", skip_all)]
    async fn execute(&self) {
        let cmd = self.command.as_command();
        tracing::debug!(message = "Executing command", name = cmd.name(), args = ?cmd);
        match cmd.execute().await {
            Ok(_) => {
                tracing::debug!(message = "Command successful", name = cmd.name())
            },
            Err(err) => {
                tracing::error!(message = "Command error", name = cmd.name(), error = ?err);
                std::process::exit(1)
            }
        }
    }
}

fn tracing_writer() -> Box<dyn io::Write> {
    if io::stdout().is_terminal() {
        Box::new(io::stdout())
    } else {
        Box::new(io::stderr())
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    cli.init();
    cli.execute().await;
}
