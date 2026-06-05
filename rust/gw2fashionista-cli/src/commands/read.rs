use std::io;
use clap::{Args};
use gw2fashionista_core::{domain::{chatlink::ChatLink, error::ChatLinkError}, models::wardrobe_template::WardrobeTemplateData};

#[derive(Args, Debug)]
pub struct Command {
    /// Chat link of the fashion template(s) to read. If empty, chat links will be read from stdin, either as a CSV file from the column fashion_link, or as one link per row
    chat_links: Vec<String>,
}

impl Command {
    fn get_links(&self) -> anyhow::Result<Vec<String>> {
        if self.chat_links.is_empty() {
            Ok(self.read_links(io::stdin().lock())?)
        } else {
            Ok(self.chat_links.clone())
        }
    }

    fn read_links<R: io::BufRead>(&self, reader: R) -> io::Result<Vec<String>> {
        Ok(reader
            .lines()
            .collect::<io::Result<Vec<_>>>()?
            .into_iter()
            .filter(|l| !l.is_empty())
            .collect()
        )
    }
}

impl super::Command for Command {
    fn name(&self) -> &str {
        return "read"
    }

    fn execute(&self) -> anyhow::Result<()> {
        let links = self.get_links()?;
        parse_and_print(&links)
    }
}

fn parse_and_print(chat_links: &Vec<String>) -> anyhow::Result<()> {
    for raw_link in chat_links {
        print(ChatLink::try_from(raw_link.as_str())?)?;
    }
    Ok(())
}

fn print(link: ChatLink) -> anyhow::Result<()> {
    match link {
        ChatLink::WardrobeTemplate(template) => {
            let data = WardrobeTemplateData::from(&template);
            println!("{:?}", data);
            Ok(())
        },
        _ => Err(ChatLinkError::NotImplemented.into())
    }
}
