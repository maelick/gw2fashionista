use async_trait::async_trait;
use clap::Args;
use gw2fashionista_core::{
    domain::{chatlink::ChatLink, error::ChatLinkError},
    gw2_data::{Resolver, equipment::Equipment, import::Importer},
};
use serde::{Deserialize, Serialize};
use std::{fs, io};

use super::args;

#[derive(Args, Debug)]
pub struct Command {
    // List of characters (if empty, uses all characters)
    characters: Vec<String>,

    /// GW2 API key
    #[arg(long, display_order = 2, env = "GW2_API_KEY", required = true)]
    #[clap(hide_env_values = true)]
    api_key: Option<String>,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = args::Format::Auto, display_order = 3)]
    format: args::Format,

    /// Filename to use as output
    #[arg(short, long, display_order = 3)]
    output: Option<std::path::PathBuf>,

    /// When provided, do not generate names for tabs without one
    #[arg(long, display_order = 4)]
    no_default_name: bool,

    #[command(flatten)]
    filters: args::EquipmentFilters,

    /// Determine concurrency for API calls (maximum 255, default = number of characters)
    #[arg(long)]
    concurrency: Option<u8>,
}

#[async_trait]
impl super::Command for Command {
    fn name(&self) -> &str {
        "wardrobe-export"
    }

    #[tracing::instrument(name = "wardrobe-export", skip_all)]
    async fn execute(&self) -> anyhow::Result<()> {
        let api_key = self.api_key.as_ref().unwrap();
        let importer = Importer::with_api_key(api_key);

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

        let exported: Result<Vec<_>, _> =
            resolved.iter().map(|e| self.export_equipment(e)).collect();
        self.output_equipments(exported?)
    }
}

impl Command {
    fn export_equipment(&self, equipment: &Equipment) -> Result<ExportedEquipment, ChatLinkError> {
        let equipment = ExportedEquipment::new(equipment)?;
        if !self.no_default_name {
            Ok(equipment.with_default_name())
        } else {
            Ok(equipment)
        }
    }

    fn output_equipments(&self, equipments: Vec<ExportedEquipment>) -> anyhow::Result<()> {
        let format = match self.format {
            args::Format::Auto => self.detect_format(),
            _ => self.format,
        };
        match format {
            args::Format::Csv => self.output_csv(equipments)?,
            args::Format::Json => self.output_json(equipments)?,
            _ => todo!(),
        };
        Ok(())
    }

    fn detect_format(&self) -> args::Format {
        match &self.output {
            Some(path) => match path.extension() {
                Some(ext) if ext == "json" => args::Format::Json,
                _ => args::Format::Csv,
            },
            None => args::Format::Csv,
        }
    }

    fn open_output(&self) -> anyhow::Result<Box<dyn io::Write>> {
        self.output
            .as_ref()
            .map(open_file)
            .unwrap_or_else(|| Ok(Box::new(io::stdout())))
    }

    fn output_csv(&self, equipments: Vec<ExportedEquipment>) -> anyhow::Result<()> {
        let mut writer = csv::Writer::from_writer(self.open_output()?);
        for e in equipments {
            writer.serialize(e)?;
        }
        Ok(())
    }

    fn output_json(&self, equipments: Vec<ExportedEquipment>) -> anyhow::Result<()> {
        serde_json::to_writer_pretty(self.open_output()?, &equipments)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ExportedEquipment {
    pub char_name: String,
    pub tab_id: usize,
    pub tab_name: String,
    pub fashion_link: String,
}

impl ExportedEquipment {
    pub fn new(equipment: &Equipment) -> Result<Self, ChatLinkError> {
        let chat_link = ChatLink::WardrobeTemplate(equipment.into());
        Ok(ExportedEquipment {
            char_name: equipment.char_name.clone(),
            tab_id: equipment.tab_id,
            tab_name: equipment.tab_name.clone(),
            fashion_link: chat_link.to_string()?,
        })
    }

    pub fn with_default_name(self) -> Self {
        if self.tab_name.is_empty() {
            ExportedEquipment {
                tab_name: format!("{} {}", self.char_name, self.tab_id),
                ..self
            }
        } else {
            self
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
