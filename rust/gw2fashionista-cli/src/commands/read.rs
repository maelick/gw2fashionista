use clap::{Args};
use gw2fashionista_core::domain::chatlink::ChatLink;

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
        if self.chat_links.is_empty() {
            Err(anyhow::anyhow!("reading from stdin not implemented"))
        } else {
            parse_and_print(&self.chat_links)
        }
    }
}

fn parse_and_print(chat_links: &Vec<String>) -> anyhow::Result<()> {
    for raw_link in chat_links {
        let link = ChatLink::try_from(raw_link.as_str())?;
        println!("{:?}", link);
    }
    Ok(())
}
