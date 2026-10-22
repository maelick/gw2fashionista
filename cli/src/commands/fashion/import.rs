use clap::{ValueEnum, builder::ArgPredicate};
use futures::stream::FuturesOrdered;
use gw2fashionista_core::{
    app::{FashionError, FashionService},
    domain::{
        fashion::{Fashion, FashionIdentifier, FashionRecordRef},
        names::{FashionName, FashionNameError},
    },
};
use gw2fashionista_storage::sqlite;

use crate::{
    commands::{self, args::DataFormat},
    environment::Environment,
    input, output, partition_result,
};

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum Mode {
    /// Overwrite existing fashion templates
    Overwrite,
    /// Patch existing fashion templates
    Patch,
    /// Skip and print a warning for existing fashion templates
    Skip,
    /// Fail if any template already exists
    Fail,
    /// Rename any template to import if there is an existing one
    Rename,
}

#[derive(clap::Args, Debug)]
pub struct Command {
    /// Input format. Auto is based on whether stdin is a TTY (CSV for TTY, JSON if not).
    #[arg(short, long, value_enum,
        default_value_t = DataFormat::Auto,
        default_value_if("json", ArgPredicate::IsPresent, "json"),
        default_value_if("csv", ArgPredicate::IsPresent, "csv"))]
    input: DataFormat,

    /// Output format. Auto uses the same format as the input.
    #[arg(short, long, value_enum,
        default_value_t = DataFormat::Auto,
        default_value_if("json", ArgPredicate::IsPresent, "json"),
        default_value_if("csv", ArgPredicate::IsPresent, "csv"))]
    output: DataFormat,

    /// Forces JSON input and output.
    #[arg(
        long,
        conflicts_with = "input",
        conflicts_with = "output",
        conflicts_with = "csv"
    )]
    json: bool,

    /// Forces CSV input and output.
    #[arg(
        long,
        conflicts_with = "input",
        conflicts_with = "output",
        conflicts_with = "json"
    )]
    csv: bool,

    /// Pretty print (JSON) output.
    #[arg(short, long)]
    pretty: bool,

    /// Import mode.
    #[arg(short, long, value_enum,
        default_value_t = Mode::Fail,
        default_value_if("force", ArgPredicate::IsPresent, "overwrite")
    )]
    mode: Mode,

    /// Force import mode overwrite.
    #[arg(short, long, conflicts_with = "mode")]
    force: bool,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "fashion-import"
    }
}

impl Command {
    #[tracing::instrument(level = "error", name = "fashion-import", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let service = env.fashion_service().await?;
        let (fashions, format) = self.read_templates()?;
        let format = match self.output {
            DataFormat::Auto => match format {
                input::Format::Csv => output::Format::Csv,
                _ => output::Format::Json,
            },
            DataFormat::Csv => output::Format::Csv,
            DataFormat::Json => output::Format::Json,
        };
        self.import_and_print(&service, fashions, format).await
    }

    pub fn read_templates(&self) -> anyhow::Result<(Vec<Fashion>, input::Format)> {
        let (fashions, format) = super::read_templates_from_stdin(&self.input)?;
        let fashions = match fashions {
            input::Input::None | input::Input::Zero => Vec::new(),
            input::Input::One(fashion) => vec![fashion],
            input::Input::Many(fashions) => fashions,
        };
        Ok((fashions, format))
    }

    async fn import_and_print(
        &self,
        service: &FashionService<sqlite::Store>,
        fashions: Vec<Fashion>,
        format: output::Format,
    ) -> anyhow::Result<()> {
        if matches!(&self.mode, Mode::Fail) {
            let fashions = existing_fashions(service, &fashions).await?;
            anyhow::ensure!(
                fashions.is_empty(),
                "Fashion templates already exist: {:?}",
                fashions
            );
        }

        let (created, failed) = self.import_many(service, fashions).await;
        output::OneOrMany::Many(&created).print::<FashionRecordRef>(format, self.pretty)?;
        anyhow::ensure!(
            failed.is_empty(),
            "Failed to import {} fashions",
            failed.len()
        );
        Ok(())
    }

    async fn import_many(
        &self,
        service: &FashionService<sqlite::Store>,
        fashions: Vec<Fashion>,
    ) -> (Vec<Fashion>, Vec<FashionError>) {
        partition_result(FuturesOrdered::from_iter(
            fashions
                .into_iter()
                .map(async |fashion| self.import_one(service, &fashion).await),
        ))
        .await
    }

    async fn import_one(
        &self,
        service: &FashionService<sqlite::Store>,
        fashion: &Fashion,
    ) -> Result<Fashion, FashionError> {
        match create_one(service, fashion).await {
            Ok(created) => Ok(created),
            Err(e) if e.is_conflict() => match self.mode {
                Mode::Overwrite => overwrite_one(service, fashion).await,
                Mode::Patch => patch_one(service, fashion).await,
                Mode::Skip | Mode::Fail => {
                    // Conflict should not be possible when using Mode::Fail due to previous check.
                    // We still handle the failed case as a skip in case it would have been created since the check.
                    tracing::warn!(
                        message = "Skipping already existing fashion template",
                        name = ?fashion.name,
                        character = ?fashion.character,
                    );
                    Err(FashionError::Skipped)
                }
                Mode::Rename => rename_and_create_one(service, fashion).await,
            },
            Err(e) => Err(e),
        }
    }
}

async fn existing_fashions(
    service: &FashionService<sqlite::Store>,
    fashions: &Vec<Fashion>,
) -> Result<Vec<FashionIdentifier>, FashionError> {
    let mut existing = vec![];
    for f in fashions {
        match service
            .get_by_name(&f.name, f.character.as_deref().cloned().as_deref())
            .await
        {
            Ok(_) => existing.push(f.identifier()),
            Err(e) if e.is_not_found() => (),
            Err(e) => return Err(e),
        }
    }
    Ok(existing)
}

async fn rename_and_create_one(
    service: &FashionService<sqlite::Store>,
    fashion: &Fashion,
) -> Result<Fashion, FashionError> {
    tracing::info!(
        message = "Fashion template already exists",
        name = ?fashion.name,
        character = ?fashion.character,
    );
    for i in 2.. {
        let name = format!("{} ({})", fashion.name, i);
        let renamed = rename(fashion, &name).expect("");
        match create_one(service, &renamed).await {
            Ok(created) => return Ok(created),
            Err(e) if e.is_conflict() => {
                tracing::info!(
                    message = "Fashion template already exists",
                    name = ?renamed.name,
                    character = ?renamed.character,
                );
            }
            Err(e) => return Err(e),
        }
    }
    panic!("Unreachable")
}

fn rename(fashion: &Fashion, new_name: &str) -> Result<Fashion, FashionNameError> {
    let mut renamed = fashion.clone();
    renamed.name = FashionName::try_new(new_name)?;
    Ok(renamed)
}

async fn overwrite_one(
    service: &FashionService<sqlite::Store>,
    fashion: &Fashion,
) -> Result<Fashion, FashionError> {
    let res = service.set(fashion).await;
    match &res {
        Ok(fashion) => tracing::info!(
            message = "Overwritten fashion",
            id = ?fashion.id,
            name = ?fashion.name,
            character = ?fashion.character,
        ),
        Err(err) => tracing::error!("Failed to overwrite fashion: {:?}", err),
    }
    res
}

async fn patch_one(
    service: &FashionService<sqlite::Store>,
    fashion: &Fashion,
) -> Result<Fashion, FashionError> {
    let res = service.patch(fashion).await;
    match &res {
        Ok(fashion) => tracing::info!(
            message = "Patched fashion",
            id = ?fashion.id,
            name = ?fashion.name,
            character = ?fashion.character,
        ),
        Err(err) => tracing::error!("Failed to patch fashion: {:?}", err),
    }
    res
}

async fn create_one(
    service: &FashionService<sqlite::Store>,
    fashion: &Fashion,
) -> Result<Fashion, FashionError> {
    let res = service.create(fashion).await;
    if let Ok(fashion) = &res {
        tracing::info!(
            message = "Created fashion",
            id = ?fashion.id,
            name = ?fashion.name,
            character = ?fashion.character,
        );
    }
    res
}
