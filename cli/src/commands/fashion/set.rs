use futures::stream::FuturesOrdered;
use gw2fashionista_core::{
    app::{FashionError, FashionService},
    domain::fashion::{Fashion, FashionRecordRef},
};
use gw2fashionista_storage::sqlite;

use crate::{
    commands::{
        self,
        args::{DataFormat, InputMode},
        fashion::{args::FashionFields, read_templates},
    },
    environment::Environment,
    input, output, partition_result,
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

    /// Input format. Auto is based on whether stdin is a TTY (CSV for TTY, JSON if not).
    #[arg(short, long, value_enum, default_value_t = DataFormat::Auto)]
    input: DataFormat,

    /// Output format. Auto uses the same format as the input.
    #[arg(short, long, value_enum, default_value_t = DataFormat::Auto)]
    output: DataFormat,

    /// Input mode. Auto is based on whether stdin is a TTY (never for TTY, always otherwise).
    #[arg(long, value_enum, default_value_t = InputMode::Auto)]
    stdin: InputMode,

    /// Pretty print (JSON) output.
    #[arg(short, long)]
    pretty: bool,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-set"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-set", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        let (fashions, format) = read_templates(&self.data, &self.input, &self.stdin)?;
        let format = match self.output {
            DataFormat::Auto => match format {
                input::Format::Csv => output::Format::Csv,
                _ => output::Format::Json,
            },
            DataFormat::Csv => output::Format::Csv,
            DataFormat::Json => output::Format::Json,
        };
        match fashions {
            input::OneOrMany::One(fashion) => {
                self.set_one_and_print(&service, fashion, format).await
            }
            input::OneOrMany::Many(fashions) => {
                self.set_many_and_print(&service, fashions, format).await
            }
        }
    }

    async fn set_one_and_print(
        &self,
        service: &FashionService<sqlite::Repository>,
        fashion: Fashion,
        format: output::Format,
    ) -> anyhow::Result<()> {
        let created = service.set(&fashion).await?;
        output::OneOrMany::One(&created).print::<FashionRecordRef>(format, self.pretty)
    }

    async fn set_many_and_print(
        &self,
        service: &FashionService<sqlite::Repository>,
        fashions: Vec<Fashion>,
        format: output::Format,
    ) -> anyhow::Result<()> {
        let (created, failed) = set_many(service, fashions).await;
        output::OneOrMany::Many(&created).print::<FashionRecordRef>(format, self.pretty)?;
        anyhow::ensure!(failed.is_empty(), "Failed to set {} fashions", failed.len());
        Ok(())
    }
}

async fn set_many(
    service: &FashionService<sqlite::Repository>,
    fashions: Vec<Fashion>,
) -> (Vec<Fashion>, Vec<FashionError>) {
    partition_result(FuturesOrdered::from_iter(
        fashions
            .into_iter()
            .map(async |fashion| set_one(service, fashion).await),
    ))
    .await
}

async fn set_one(
    service: &FashionService<sqlite::Repository>,
    fashion: Fashion,
) -> Result<Fashion, FashionError> {
    let res = service.set(&fashion).await;
    match &res {
        Ok(fashion) => tracing::info!("Created fashion: {:?}", fashion),
        Err(err) => tracing::error!("Failed to set fashion: {:?}", err),
    }
    res
}
