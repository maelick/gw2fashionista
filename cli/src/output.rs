use std::{io, iter, path::PathBuf};

use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Format {
    /// CSV
    Csv,
    /// JSON
    Json,
}

#[derive(Debug, Clone, Serialize)]
pub enum OneOrMany<'a, T>
where
    T: Serialize,
{
    One(&'a T),
    Many(&'a [T]),
}

impl<'a, T: Serialize + 'static> OneOrMany<'a, T> {
    pub fn print<FlatRecord: Serialize + From<&'a T>>(
        &self,
        format: Format,
        pretty: bool,
    ) -> anyhow::Result<()> {
        self.output::<_, FlatRecord>(format, io::stdout(), pretty)
    }

    pub fn print_csv<FlatRecord: Serialize + From<&'a T>>(&self) -> anyhow::Result<()> {
        self.output_csv::<_, FlatRecord>(io::stdout())
    }

    pub fn print_json(&self, pretty: bool) -> anyhow::Result<()> {
        self.output_json(io::stdout(), pretty)
    }

    pub fn output<W, FlatRecord>(&self, format: Format, dest: W, pretty: bool) -> anyhow::Result<()>
    where
        W: io::Write,
        FlatRecord: Serialize + From<&'a T>,
    {
        match format {
            Format::Csv => self.output_csv::<_, FlatRecord>(dest),
            Format::Json => self.output_json(dest, pretty),
        }
    }

    pub fn output_csv<W, FlatRecord>(&self, dest: W) -> anyhow::Result<()>
    where
        W: io::Write,
        FlatRecord: Serialize + From<&'a T>,
    {
        match self {
            OneOrMany::One(data) => output_csv::<_, _, _, FlatRecord>(iter::once(*data), dest),
            OneOrMany::Many(data) => output_csv::<_, _, _, FlatRecord>(data.iter(), dest),
        }
    }

    pub fn output_json<W: io::Write>(&self, dest: W, pretty: bool) -> anyhow::Result<()> {
        match self {
            OneOrMany::One(data) => output_json(data, dest, pretty),
            OneOrMany::Many(data) => output_json(data, dest, pretty),
        }
    }
}

pub fn detect_format(dest: Option<&PathBuf>) -> Format {
    match dest {
        Some(path) => match path.extension() {
            Some(ext) if ext == "json" => Format::Json,
            _ => Format::Csv,
        },
        None => Format::Csv,
    }
}

fn output_json<T: Serialize, W: io::Write>(data: &T, dest: W, pretty: bool) -> anyhow::Result<()> {
    if pretty {
        serde_json::to_writer_pretty(dest, data)?;
    } else {
        serde_json::to_writer(dest, data)?;
    }
    Ok(())
}

fn output_csv<'a, T, W, FlatRecords, FlatRecord>(data: FlatRecords, dest: W) -> anyhow::Result<()>
where
    W: io::Write,
    T: Serialize + 'static,
    FlatRecords: Iterator<Item = &'a T>,
    FlatRecord: Serialize + From<&'a T>,
{
    let mut writer = csv::Writer::from_writer(dest);
    for d in data {
        writer.serialize(FlatRecord::from(d.into()))?;
    }

    Ok(())
}
