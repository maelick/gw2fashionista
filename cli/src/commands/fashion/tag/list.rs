use std::io::{self, IsTerminal};

use gw2fashionista_core::domain::{filters::StringFilters, tag::Tag};

use crate::{
    commands::{self, args::DataFormat},
    environment::Environment,
    output,
};

#[derive(clap::Args, Debug)]
pub struct Command {
    #[arg(from_global)]
    clipboard: bool,

    /// Output format. Auto is based on whether stdout is a TTY (CSV for TTY, JSON if not).
    #[arg(short, long, value_enum, default_value_t = DataFormat::Auto)]
    format: DataFormat,

    /// Pretty print (JSON) output.
    #[arg(short, long)]
    pretty: bool,

    /// Prefix to filter tags.
    #[arg(long)]
    prefix: Option<String>,

    /// Suffix to filter tags.
    #[arg(long)]
    suffix: Option<String>,

    /// Infix to filter tags.
    #[arg(long)]
    contains: Vec<String>,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-tag-list"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-tag-list", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        let filters = StringFilters::builder()
            .maybe_prefix(self.prefix.as_deref())
            .maybe_suffix(self.prefix.as_deref())
            .substrings(self.contains.iter())
            .build();
        let tags = service.list_tags(filters).await?;
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
        output::OneOrMany::Many(&tags).print::<&Tag>(format, self.pretty)
    }
}
