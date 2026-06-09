use std::io;
use clap::{Args};
use gw2fashionista_core::models::wardrobe_template::WardrobeTemplateData;
use gw2fashionista_core::domain::{chatlink::ChatLink, error::ChatLinkError, wardrobe_template::WardrobeTemplate};
use gw2fashionista_core::gw2_data::Resolver;

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
        let raw_links = self.get_links()?;
        let links = parse(&raw_links)?;
        let templates = wardrobe_templates(&links);

        let mut resolver = Resolver::default();
        resolver.cache_wardrobe_templates(templates)?;

        for link in links {
            let data = resolver.resolve_chat_link(&link)?;
            print(&data)?;
        }
        Ok(())
    }
}

fn wardrobe_templates(chat_links: &Vec<ChatLink>) -> Vec<&WardrobeTemplate> {
    chat_links.iter().filter_map(|link| {
        match link {
            ChatLink::WardrobeTemplate(template) => Some(template),
            _ => None,
        }
    }).collect()
}

fn parse(chat_links: &Vec<String>) -> Result<Vec<ChatLink>, ChatLinkError> {
    let links: Result<Vec<_>, _> = chat_links.iter().map(|raw_link| {
        ChatLink::try_from(raw_link.as_str())
    }).collect();
    Ok(links?)
}

fn print(data: &WardrobeTemplateData) -> anyhow::Result<()> {
    serde_json::to_writer_pretty(io::stdout(), data)?;
    Ok(())
}
