use std::{io, iter};
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
            self.read_links(io::stdin().lock())
        } else {
            Ok(self.chat_links.clone())
        }
    }

    fn read_links<R: io::BufRead>(&self, reader: R) -> anyhow::Result<Vec<String>> {
        let mut reader = csv::Reader::from_reader(reader);
        let headers = reader.headers()?.clone();
        if headers.len() == 0 {
            Err(anyhow::anyhow!("Empty CSV input"))
        } else if headers.len() == 1 {
            self.read_single_column(headers, &mut reader)
        } else {
            self.read_multiple_columns(headers, &mut reader)
        }
    }

    fn read_single_column<R: io::BufRead>(&self, headers: csv::StringRecord, reader: &mut csv::Reader<R>) -> anyhow::Result<Vec<String>> {
        let first = iter::once(Ok(headers.get(0).unwrap().to_string()));
        let others = self.read_from_column(reader, 0);
        first.chain(others).collect()
    }

    fn read_multiple_columns<R: io::BufRead>(&self, headers: csv::StringRecord, reader: &mut csv::Reader<R>) -> anyhow::Result<Vec<String>> {
        let col = self.find_column(headers, "fashion_link")?;
        self.read_from_column(reader, col).collect()
    }

    fn read_from_column<R: io::BufRead>(&self, reader: &mut csv::Reader<R>, col: usize) -> impl Iterator<Item = anyhow::Result<String>> {
        reader.records().map(move |r| self.read_chat_link(r?, col))
    }

    fn read_chat_link(&self, record: csv::StringRecord, col: usize) -> anyhow::Result<String> {
        match record.get(col) {
            Some(value) => Ok(value.to_string()),
            None => Err(anyhow::anyhow!("Invalid CSV file: row {} missing column {}", 0, col)),
        }
    }

    fn find_column(&self, headers: csv::StringRecord, col_name: &str) -> anyhow::Result<usize> {
        match headers.iter().position(|header| header == col_name) {
            Some(value) => Ok(value),
            None => Err(anyhow::anyhow!("Missing column {} in CSV input", col_name))
        }
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
