use crate::{
    commands::{self, args::DataFormat},
    environment::Environment,
};

#[derive(clap::Args, Debug)]
pub struct Command {
    /// Tags to list fashion templates for.
    #[arg(value_name = "TAG")]
    tags: Vec<String>,

    #[arg(from_global)]
    clipboard: bool,

    /// Output format. Auto is based on whether stdout is a TTY (CSV for TTY, JSON if not).
    #[arg(short, long, value_enum, default_value_t = DataFormat::Auto)]
    format: DataFormat,

    /// Pretty print (JSON) output.
    #[arg(short, long)]
    pretty: bool,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-list"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-list", skip_all)]
    pub async fn execute(&self, _env: Environment) -> anyhow::Result<()> {
        todo!()
    }
}
