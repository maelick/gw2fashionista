use clap::{Args};

use super::args;

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the base fashion template to override
    base_fashion_template: String,

    /// Chat link of the fashion template with new values to apply to the base one
    new_fashion_template: String,

    #[command(flatten)]
    skin_dyes_only: args::SkinsOrDyes,

    #[command(flatten)]
    filters: args::EquipmentFilters,
}

impl super::Command for Command {
    fn name(&self) -> &str {
        return "merge"
    }

    fn execute(&self) -> anyhow::Result<()> {
        return Err(anyhow::anyhow!("not implemented"))
    }
}
