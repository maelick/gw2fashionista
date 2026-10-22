use clap::Args;
use clap::builder::ArgPredicate;
use gw2fashionista_appearance::gw2::{equipment::Equipment, import::Importer, resolve::Resolver};
use gw2fashionista_core::ports::repositories::SecretRepository;
use serde::{Deserialize, Serialize};
use std::{fs, io};

use crate::commands::args;
use crate::commands::wardrobe::args::WardrobeFilters;
use crate::environment::Environment;
use crate::{commands, output};

#[derive(Args, Debug)]
pub struct Command {
    // List of characters (if empty, uses all characters)
    characters: Vec<String>,

    /// Output format. Auto is based on the output filename extension (default to CSV if missing filename or unknown extension).
    #[arg(short, long, value_enum,
        default_value_t = args::DataFormat::Auto,
        default_value_if("json", ArgPredicate::IsPresent, "json"),
        default_value_if("csv", ArgPredicate::IsPresent, "csv"))]
    format: args::DataFormat,

    /// Forces JSON output.
    #[arg(long, conflicts_with = "format", conflicts_with = "csv")]
    json: bool,

    /// Forces CSV output.
    #[arg(long, conflicts_with = "format", conflicts_with = "json")]
    csv: bool,

    /// Pretty print (JSON) output.
    #[arg(short, long)]
    pretty: bool,

    /// Filename to use as output
    #[arg(short, long, display_order = 3)]
    output: Option<std::path::PathBuf>,

    #[command(flatten)]
    filters: WardrobeFilters,

    /// Determine concurrency for API calls (maximum 255, default = number of characters)
    #[arg(long)]
    concurrency: Option<u8>,
}

impl commands::Command for Command {
    fn name(&self) -> &str {
        "wardrobe-export"
    }
}

impl Command {
    #[tracing::instrument(level = "error", name = "wardrobe-export", skip_all)]
    pub async fn execute(&self, mut env: Environment) -> anyhow::Result<()> {
        let api_key = env.secret_repo().await?.gw2_api_key().await;
        if let Err(err) = &api_key {
            anyhow::bail!("GW2 API key: {}", err);
        }
        let importer = Importer::with_api_key(&api_key?);

        let characters = if self.characters.is_empty() {
            importer.characters().await?
        } else {
            self.characters.clone()
        };

        let concurrency = self.concurrency.map_or(characters.len(), |c| c as usize);
        let equipments = importer
            .with_buffer_size(concurrency)
            .fetch_equipment(&characters)
            .await?;
        let resolved = Resolver::default()
            .with_buffer_size(concurrency)
            .resolve_equipment(equipments)
            .await?;

        let exported: Vec<_> = resolved.iter().map(ExportedEquipment::new).collect();
        self.output_equipments(exported)
    }

    fn output_equipments(&self, equipments: Vec<ExportedEquipment>) -> anyhow::Result<()> {
        let format = match self.format {
            args::DataFormat::Auto => output::detect_format(self.output.as_ref()),
            args::DataFormat::Csv => output::Format::Csv,
            args::DataFormat::Json => output::Format::Json,
        };
        output::OneOrMany::Many(&equipments).output::<_, &ExportedEquipment>(
            format,
            self.open_output()?,
            self.pretty,
        )
    }

    fn open_output(&self) -> anyhow::Result<Box<dyn io::Write>> {
        self.output
            .as_ref()
            .map(open_file)
            .unwrap_or_else(|| Ok(Box::new(io::stdout())))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ExportedEquipment {
    pub name: String,
    pub character: String,
    pub tab_id: usize,
    pub wardrobe_template: String,
}

impl ExportedEquipment {
    pub fn new(equipment: &Equipment) -> Self {
        let name = if equipment.tab_name.is_empty() {
            format!("Equipment tab {}", equipment.tab_id)
        } else {
            format!("Equipment tab {}", equipment.tab_name)
        };
        ExportedEquipment {
            name,
            character: equipment.char_name.clone(),
            tab_id: equipment.tab_id,
            wardrobe_template: equipment.to_template().to_string(),
        }
    }
}

fn open_file(path: &std::path::PathBuf) -> anyhow::Result<Box<dyn io::Write>> {
    let writer = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    Ok(Box::new(writer))
}
