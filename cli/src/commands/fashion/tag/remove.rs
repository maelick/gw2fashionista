use crate::{
    commands::{
        self,
        args::{DataFormat, InputMode},
        fashion::args::FashionIdentifier,
    },
    environment::Environment,
};

#[derive(clap::Args, Debug)]
pub struct Command {
    /// Tags to remove.
    #[arg(value_name = "TAG", required = true)]
    tags: Vec<String>,

    #[arg(from_global)]
    clipboard: bool,

    #[command(flatten)]
    id: Option<FashionIdentifier>,

    /// Input format. Auto is based on whether stdin is a TTY (CSV for TTY, JSON if not).
    #[arg(short, long, value_enum, default_value_t = DataFormat::Auto)]
    format: DataFormat,

    /// Input mode. Auto is based on whether stdin is a TTY (never for TTY, always otherwise).
    #[arg(long, value_enum, default_value_t = InputMode::Auto)]
    stdin: InputMode,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-tag-remove"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-tag-remove", skip_all)]
    pub async fn execute(&self, _env: Environment) -> anyhow::Result<()> {
        todo!()
    }
}
