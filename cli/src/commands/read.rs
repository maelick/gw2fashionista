use async_trait::async_trait;
use clap::Args;
use gw2fashionista_core::domain::templates::FashionSlot;
use gw2fashionista_core::domain::{
    chatlink::ChatLink, error::ChatLinkError, templates::wardrobe::WardrobeTemplate,
};
use gw2fashionista_core::gw2::Resolver;
use gw2fashionista_core::models::template::TemplateData;
use serde::Serialize;
use std::{io, iter};

#[derive(Args, Debug)]
pub struct Command {
    /// Chat links to read.
    /// If empty, chat links will be read from stdin,
    /// either as a CSV file from a specific column (with headers),
    /// or as one link per row (without headers).
    chat_links: Vec<String>,

    /// Do not exit on parse errors.
    /// Attempt to parse all chat links and log the encountered errors on stderr.
    #[arg(long)]
    lenient: bool,

    /// Skip skin/outfit/dye API name resolution.
    #[arg(long)]
    skip_names: bool,

    /// Pretty print (JSON) output.
    #[arg(short, long)]
    pretty: bool,

    /// Name of the CSV column containing the chat link to parse.
    #[arg(short, long, default_value = "fashion_link")]
    column: Option<String>,

    /// Determine concurrency for API calls (maximum 255)
    #[arg(long, default_value_t = 10)]
    concurrency: u8,
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
        if headers.is_empty() {
            Err(anyhow::anyhow!("Empty CSV input"))
        } else if headers.len() == 1 {
            self.read_single_column(headers, &mut reader)
        } else {
            self.read_multiple_columns(headers, &mut reader)
        }
    }

    fn read_single_column<R: io::BufRead>(
        &self,
        headers: csv::StringRecord,
        reader: &mut csv::Reader<R>,
    ) -> anyhow::Result<Vec<String>> {
        let first = iter::once(Ok(headers.get(0).unwrap().to_string()));
        let others = self.read_from_column(reader, 0);
        self.collect(first.chain(others).map(|r| ((), r)), |_, err| {
            tracing::error!(message = "Error reading raw chat links", error = ?err);
        })
    }

    fn read_multiple_columns<R: io::BufRead>(
        &self,
        headers: csv::StringRecord,
        reader: &mut csv::Reader<R>,
    ) -> anyhow::Result<Vec<String>> {
        let col_name = self.column.as_ref().unwrap().as_str();
        let col = self.find_column(headers, col_name)?;
        self.collect(
            self.read_from_column(reader, col).map(|r| ((), r)),
            |_, err| {
                tracing::error!(message = "Error reading chat links from CSV", error = ?err);
            },
        )
    }

    fn read_from_column<R: io::BufRead>(
        &self,
        reader: &mut csv::Reader<R>,
        col: usize,
    ) -> impl Iterator<Item = anyhow::Result<String>> {
        reader.records().map(move |r| self.read_chat_link(r?, col))
    }

    fn read_chat_link(&self, record: csv::StringRecord, col: usize) -> anyhow::Result<String> {
        match record.get(col) {
            Some(value) => Ok(value.to_string()),
            None => Err(anyhow::anyhow!("Invalid CSV file: missing column {}", col)), // Should not be possible
        }
    }

    fn find_column(&self, headers: csv::StringRecord, col_name: &str) -> anyhow::Result<usize> {
        match headers.iter().position(|header| header == col_name) {
            Some(value) => Ok(value),
            None => Err(anyhow::anyhow!("Missing column {} in CSV input", col_name)),
        }
    }

    fn parse(&self, chat_links: &[String]) -> Result<Vec<ChatLink>, ChatLinkError> {
        let iter = chat_links
            .iter()
            .map(|raw_link| (raw_link, ChatLink::try_from(raw_link.as_str())));
        self.collect(iter, |link, err| {
            tracing::error!(message = "Error parsing chat link", chat_link = ?link, error = ?err);
        })
    }

    fn collect<V, T, E, I, F>(&self, iter: I, on_error: F) -> Result<Vec<T>, E>
    where
        I: IntoIterator<Item = (V, Result<T, E>)>,
        F: Fn(V, E),
    {
        if self.lenient {
            collect_lenient(iter, on_error)
        } else {
            iter.into_iter().map(|(_, res)| res).collect()
        }
    }

    async fn process<S: FashionSlot>(
        &self,
        resolver: &Resolver,
        template: &TemplateData<S>,
    ) -> anyhow::Result<()> {
        let data = if self.skip_names {
            template
        } else {
            &resolver.resolve_template(template).await?
        };

        print(data, self.pretty)
    }
}

#[async_trait]
impl super::Command for Command {
    fn name(&self) -> &str {
        "read"
    }

    #[tracing::instrument(name = "read", skip_all)]
    async fn execute(&self) -> anyhow::Result<()> {
        let raw_links = self.get_links()?;
        let links = self.parse(&raw_links)?;
        let resolver = Resolver::default().with_buffer_size(self.concurrency as usize);
        if !self.skip_names {
            resolver
                .cache_wardrobe_templates(wardrobe_templates(&links))
                .await?;
        }

        for link in &links {
            match link {
                ChatLink::WardrobeTemplate(template) => {
                    self.process(&resolver, &template.into()).await
                }
                ChatLink::TravelTemplate(template) => {
                    self.process(&resolver, &template.into()).await
                }
                _ => Err(anyhow::anyhow!("Unsupported chat link type"))?,
            }?;
        }
        Ok(())
    }
}

fn wardrobe_templates(chat_links: &[ChatLink]) -> Vec<&WardrobeTemplate> {
    chat_links
        .iter()
        .filter_map(|link| match link {
            ChatLink::WardrobeTemplate(template) => Some(template),
            _ => None,
        })
        .collect()
}

fn print<S: FashionSlot + Serialize>(data: &TemplateData<S>, pretty: bool) -> anyhow::Result<()> {
    if pretty {
        serde_json::to_writer_pretty(io::stdout(), data)?;
    } else {
        serde_json::to_writer(io::stdout(), data)?;
    }
    Ok(())
}

fn collect_lenient<V, T, E, I, F>(iter: I, on_error: F) -> Result<Vec<T>, E>
where
    I: IntoIterator<Item = (V, Result<T, E>)>,
    F: Fn(V, E),
{
    Ok(iter
        .into_iter()
        .filter_map(|(value, result)| {
            result.map_or_else(
                |err| {
                    on_error(value, err);
                    None
                },
                Some,
            )
        })
        .collect())
}
