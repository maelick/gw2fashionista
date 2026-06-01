use clap::{Args};

use super::args;

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the fashion template to filter
    fashion_template: String,

    #[command(flatten)]
    filters: args::SkinFilters,
}

impl super::Command for Command {
    fn name(&self) -> &str {
        return "filter"
    }

    fn execute(&self) -> anyhow::Result<()> {
        return Err(anyhow::anyhow!("not implemented"))
    }
}
