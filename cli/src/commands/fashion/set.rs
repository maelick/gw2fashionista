use crate::{
    commands::{
        self,
        args::{DataFormat, InputMode},
        fashion::args::FashionFields,
    },
    environment::Environment,
};

#[derive(clap::Args, Debug)]
pub struct Command {
    #[arg(from_global)]
    clipboard: bool,

    /// Id of the fashion template.
    #[arg(long, value_name = "UUID")]
    id: Option<String>,

    #[command(flatten)]
    data: FashionFields,

    /// Input and output format. Auto is based on whether stdin and stdout are TTYs (CSV for TTY, JSON if not).
    #[arg(short, long, value_enum, default_value_t = DataFormat::Auto)]
    format: DataFormat,

    /// Input mode. Auto is based on whether stdin is a TTY (never for TTY, always otherwise).
    #[arg(long, value_enum, default_value_t = InputMode::Auto)]
    stdin: InputMode,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-set"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-set", skip_all)]
    pub async fn execute(&self, _env: Environment) -> anyhow::Result<()> {
        todo!()
    }
}
