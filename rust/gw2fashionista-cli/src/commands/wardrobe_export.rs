use clap::{Args};
use gw2fashionista_core::{domain::chatlink::ChatLink, gw2_data::import::Importer};

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
}

impl super::Command for Command {
    fn name(&self) -> &str {
        return "export"
    }

    fn execute(&self) -> anyhow::Result<()> {
        let api_key = self.api_key.as_ref().unwrap();
        let importer = Importer::with_api_key(api_key);
        let equipments = importer.fetch_equipment(&self.characters)?;
        for e in &equipments {
            let chatlink = ChatLink::WardrobeTemplate(e.into());
            println!("{}", chatlink.to_string()?);
        }
        Ok(())
    }
}
