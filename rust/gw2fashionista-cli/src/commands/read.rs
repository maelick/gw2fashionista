use clap::{Args};

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the fashion template(s) to read. If empty, chat links will be read from stdin, either as a CSV file from the column fashion_link, or as one link per row
    chat_links: Vec<String>,
}

impl super::Command for Command {
    fn name(&self) -> &str {
        return "read"
    }

    fn execute(&self) -> anyhow::Result<()> {
        return Err(anyhow::anyhow!("not implemented"))
    }
}
