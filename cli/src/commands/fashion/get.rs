use std::io::{self, IsTerminal};

use gw2fashionista_core::{
    app::FashionService,
    domain::fashion::{Fashion, FashionRecordRef},
};
use gw2fashionista_storage::sqlite;

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
        let fashion = self.get_fashion(&service).await?;
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

    async fn get_fashion(
        &self,
        service: &FashionService<sqlite::Repository>,
    ) -> anyhow::Result<Fashion> {
        Ok(match self.id.id {
            Some(id) => service.get_by_id(&id.into()).await?,
            None => match &self.id.name {
                Some(name) => {
                    let character = self.id.character.as_deref();
                    service.get_by_name(name, character).await?
                }
                None => anyhow::bail!("Missing fashion template id or name"), // Should never happen
            },
        })
    }
}
