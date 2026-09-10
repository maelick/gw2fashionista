use std::io;

use futures::StreamExt;
use futures::stream::FuturesOrdered;
use gw2fashionista_core::{
    app::{FashionError, FashionService},
    domain::fashion::{Fashion, FashionRecord, FashionRecordRef},
};
use gw2fashionista_storage::sqlite;

use crate::{
    commands::{
        self,
        args::{DataFormat, InputMode},
        fashion::args::FashionFields,
    },
    environment::Environment,
    input, output,
};

#[derive(clap::Args, Debug)]
pub struct Command {
    #[arg(from_global)]
    clipboard: bool,

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
        "fashion-create"
    }
}

impl Command {
    #[tracing::instrument(name = "fashion-create", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        let (fashions, format) = self.read_templates()?;
        let format = match format {
            input::Format::Csv => output::Format::Csv,
            _ => output::Format::Json,
        };
        match fashions {
            input::OneOrMany::One(fashion) => create_one_and_print(&service, fashion, format).await,
            input::OneOrMany::Many(fashions) => {
                create_many_and_print(&service, fashions, format).await
            }
        }
    }

    fn read_templates(&self) -> anyhow::Result<(input::OneOrMany<Fashion>, input::Format)> {
        let (fashions, format) = if (&self.stdin).into() {
            self.read_templates_from_stdin()?
        } else {
            (
                input::Input::None,
                match self.format {
                    DataFormat::Csv => input::Format::Csv,
                    DataFormat::Json | DataFormat::Auto => input::Format::Json,
                },
            )
        };
        let fashions = match fashions {
            input::Input::None => input::OneOrMany::One((&self.data).try_into()?),
            input::Input::Zero => input::OneOrMany::Many(Vec::new()),
            input::Input::One(fashion) => input::OneOrMany::One(self.merge_fashion(fashion)?),
            input::Input::Many(fashions) => input::OneOrMany::Many(self.merge_tags(fashions)?),
        };
        Ok((fashions, format))
    }

    fn read_templates_from_stdin(&self) -> anyhow::Result<(input::Input<Fashion>, input::Format)> {
        let mut stdin = io::stdin().lock();
        match self.format {
            DataFormat::Auto => {
                let (format, mut reader) = input::detect_format(&mut stdin)?;
                input::read_csv_json::<Fashion, _, FashionRecord>(&mut reader, format)
            }
            DataFormat::Csv => {
                input::read_csv_json::<Fashion, _, FashionRecord>(&mut stdin, input::Format::Csv)
            }
            DataFormat::Json => {
                input::read_csv_json::<Fashion, _, FashionRecord>(&mut stdin, input::Format::Json)
            }
        }
    }

    fn merge_fashion(&self, mut fashion: Fashion) -> anyhow::Result<Fashion> {
        if let Some(name) = &self.data.name {
            fashion.name = name.clone();
        }
        if let Some(description) = &self.data.description {
            fashion.description = Some(description.clone());
        }
        if let Some(character) = &self.data.character {
            fashion.character = Some(character.clone());
        }
        if let Some(wardrobe_template) = &self.data.wardrobe {
            fashion.wardrobe_template = Some(wardrobe_template.clone());
        }
        if let Some(travel_template) = &self.data.travel {
            fashion.travel_template = Some(travel_template.clone());
        }
        self.ensure_tags(fashion)
    }

    fn ensure_tags(&self, mut fashion: Fashion) -> anyhow::Result<Fashion> {
        for tag in &self.data.tags {
            if !fashion.tags.contains(tag) {
                fashion.tags.push(tag.clone());
            }
        }
        Ok(fashion)
    }

    fn merge_tags(&self, mut fashions: Vec<Fashion>) -> anyhow::Result<Vec<Fashion>> {
        for fashion in &mut fashions {
            *fashion = self.ensure_tags(fashion.clone())?;
        }
        Ok(fashions)
    }
}

async fn create_one_and_print(
    service: &FashionService<sqlite::Repository>,
    fashion: Fashion,
    format: output::Format,
) -> anyhow::Result<()> {
    let created = service.create(&fashion).await?;
    output::OneOrMany::One(&created).print::<FashionRecordRef>(format, true)
}

async fn create_many_and_print(
    service: &FashionService<sqlite::Repository>,
    fashions: Vec<Fashion>,
    format: output::Format,
) -> anyhow::Result<()> {
    let (created, failed) = create_many(service, fashions).await;
    output::OneOrMany::Many(&created).print::<FashionRecordRef>(format, false)?;
    anyhow::ensure!(
        failed.is_empty(),
        "Failed to create {} fashions",
        failed.len()
    );
    Ok(())
}

async fn create_many(
    service: &FashionService<sqlite::Repository>,
    fashions: Vec<Fashion>,
) -> (Vec<Fashion>, Vec<FashionError>) {
    partition_result(FuturesOrdered::from_iter(
        fashions
            .into_iter()
            .map(async |fashion| create_one(service, fashion).await),
    ))
    .await
}

async fn partition_result<T, E>(
    result_stream: impl futures::Stream<Item = Result<T, E>>,
) -> (Vec<T>, Vec<E>) {
    result_stream
        .fold(
            (Vec::new(), Vec::new()),
            async |(mut created, mut failed), res| {
                match res {
                    Ok(fashion) => created.push(fashion),
                    Err(err) => failed.push(err),
                }
                (created, failed)
            },
        )
        .await
}

async fn create_one(
    service: &FashionService<sqlite::Repository>,
    fashion: Fashion,
) -> Result<Fashion, FashionError> {
    let res = service.create(&fashion).await;
    match &res {
        Ok(fashion) => tracing::info!("Created fashion: {:?}", fashion),
        Err(err) => tracing::error!("Failed to create fashion: {:?}", err),
    }
    res
}
