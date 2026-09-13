use std::io::{self, IsTerminal};

use gw2fashionista_core::domain::fashion::FashionRecordRef;

use crate::{
    commands::{self, args::DataFormat, fashion::args::FashionIdentifier},
    environment::Environment,
    output,
};

#[derive(clap::Args, Debug)]
#[command(mut_group("identifier", |g| g.required(true)))]
pub struct Command {
    #[arg(from_global)]
    clipboard: bool,

    #[command(flatten)]
    id: FashionIdentifier,

    /// Output format. Auto is based on whether stdout is a TTY (CSV for TTY, JSON if not).
    #[arg(short, long, value_enum, default_value_t = DataFormat::Auto)]
    format: DataFormat,

    /// Pretty print (JSON) output.
    #[arg(short, long)]
    pretty: bool,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-get"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-get", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        let fashion = self.id.get_fashion(&service).await?;
        let format = match self.format {
            DataFormat::Auto => {
                if io::stdout().is_terminal() {
                    output::Format::Csv
                } else {
                    output::Format::Json
                }
            }
            DataFormat::Csv => output::Format::Csv,
            DataFormat::Json => output::Format::Json,
        };
        output::OneOrMany::One(&fashion).print::<FashionRecordRef>(format, self.pretty)
    }
}
