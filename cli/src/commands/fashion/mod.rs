use std::{io, path::PathBuf};

use gw2fashionista_core::domain::fashion::{Fashion, FashionRecord};

use crate::{
    commands::{
        Command,
        args::{DataFormat, InputMode},
        fashion::args::{FashionFields, FashionIdentifier},
    },
    environment::Environment,
    input,
};

mod args;
mod create;
mod get;
mod list;
mod patch;
mod set;
mod tag;
mod travel;
mod untag;
mod wardrobe;

#[derive(clap::Args, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Read or write data from clipboard.
    /// If any other data is also provided on stdin or as arguments, it overrides the data from the clipboard.
    #[arg(long, global = true)]
    clipboard: bool,

    /// GW2 API key
    #[arg(long = "db", env = "GW2FASHIONISTA_DB", required = true)]
    #[clap(hide_env_values = false)]
    db_path: PathBuf,
}

impl Args {
    pub(crate) fn command(&self) -> &dyn Command {
        match &self.command {
            Commands::Create(cmd) => cmd,
            Commands::Get(cmd) => cmd,
            Commands::Set(cmd) => cmd,
            Commands::Patch(cmd) => cmd,
            Commands::List(cmd) => cmd,
            Commands::Wardrobe(args) => args.command(),
            Commands::Travel(args) => args.command(),
            Commands::Tag(args) => args.command(),
            Commands::Untag(cmd) => cmd,
        }
    }

    pub async fn execute(&self) -> anyhow::Result<()> {
        let env = Environment::builder().db_path(&self.db_path).build();
        match &self.command {
            Commands::Create(cmd) => cmd.execute(env).await,
            Commands::Get(cmd) => cmd.execute(env).await,
            Commands::Set(cmd) => cmd.execute(env).await,
            Commands::Patch(cmd) => cmd.execute(env).await,
            Commands::List(cmd) => cmd.execute(env).await,
            Commands::Wardrobe(args) => args.execute(env).await,
            Commands::Travel(args) => args.execute(env).await,
            Commands::Tag(args) => args.execute(env).await,
            Commands::Untag(cmd) => cmd.execute(env).await,
        }
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Create a fashion template.
    Create(create::Command),
    /// Get an existing fashion template.
    Get(get::Command),
    /// Set an existing fashion template, overriding all fields with those provided and removing the ones not set.
    Set(set::Command),
    /// Patch an existing fashion, overriding only provided fields.
    Patch(patch::Command),
    /// List existing fashion templates.
    #[command(visible_alias = "ls")]
    List(list::Command),
    /// Get or set the wardrobe template of a fashion template.
    Wardrobe(wardrobe::Args),
    /// Get or set the travel template of a fashion template.
    Travel(travel::Args),
    /// Manage fashion template tags
    #[command(visible_alias = "tags")]
    Tag(tag::Args),
    /// Untag fashion templates
    Untag(untag::Command),
}

pub fn read_identifiers(
    input: &DataFormat,
) -> anyhow::Result<(input::Input<FashionIdentifier>, input::Format)> {
    let mut stdin = io::stdin().lock();
    match input {
        DataFormat::Auto => {
            let (format, mut reader) = input::detect_format(&mut stdin)?;
            input::read_csv_json::<FashionIdentifier, _, FashionIdentifier>(&mut reader, format)
        }
        DataFormat::Csv => input::read_csv_json::<FashionIdentifier, _, FashionIdentifier>(
            &mut stdin,
            input::Format::Csv,
        ),
        DataFormat::Json => input::read_csv_json::<FashionIdentifier, _, FashionIdentifier>(
            &mut stdin,
            input::Format::Json,
        ),
    }
}

pub fn read_templates(
    fashion_fields: &FashionFields,
    input: &DataFormat,
    stdin: &InputMode,
) -> anyhow::Result<(input::OneOrMany<Fashion>, input::Format)> {
    let (fashions, format) = if stdin.into() {
        read_templates_from_stdin(input)?
    } else {
        (
            input::Input::None,
            match input {
                DataFormat::Csv => input::Format::Csv,
                DataFormat::Json | DataFormat::Auto => input::Format::Json,
            },
        )
    };
    let fashions = match fashions {
        input::Input::None => input::OneOrMany::One((fashion_fields).try_into()?),
        input::Input::Zero => input::OneOrMany::Many(Vec::new()),
        input::Input::One(fashion) => {
            input::OneOrMany::One(merge_fashion(fashion, fashion_fields)?)
        }
        input::Input::Many(fashions) => {
            input::OneOrMany::Many(merge_tags(fashions, &fashion_fields.tags)?)
        }
    };
    Ok((fashions, format))
}

fn read_templates_from_stdin(
    input: &DataFormat,
) -> anyhow::Result<(input::Input<Fashion>, input::Format)> {
    let mut stdin = io::stdin().lock();
    match input {
        DataFormat::Auto => {
            let (format, mut reader) = input::detect_format(&mut stdin)?;
            input::read_csv_json::<Fashion, _, FashionRecord>(&mut reader, format)
        }
        DataFormat::Csv => {
            input::read_csv_json::<Fashion, _, FashionRecord>(&mut stdin, input::Format::Csv)
        }
        DataFormat::Json => {
            input::read_csv_json::<Fashion, _, FashionRecord>(&mut stdin, input::Format::Json)
        }
    }
}

fn merge_fashion(mut fashion: Fashion, fashion_fields: &FashionFields) -> anyhow::Result<Fashion> {
    if let Some(name) = &fashion_fields.name {
        fashion.name = name.clone();
    }
    if let Some(description) = &fashion_fields.description {
        fashion.description = Some(description.clone());
    }
    if let Some(character) = &fashion_fields.character {
        fashion.character = Some(character.clone());
    }
    if let Some(wardrobe_template) = &fashion_fields.wardrobe {
        fashion.wardrobe_template = Some(wardrobe_template.clone());
    }
    if let Some(travel_template) = &fashion_fields.travel {
        fashion.travel_template = Some(travel_template.clone());
    }
    ensure_tags(fashion, &fashion_fields.tags)
}

fn ensure_tags(mut fashion: Fashion, tags: &[String]) -> anyhow::Result<Fashion> {
    for tag in tags {
        if !fashion.tags.contains(tag) {
            fashion.tags.push(tag.clone());
        }
    }
    Ok(fashion)
}

fn merge_tags(mut fashions: Vec<Fashion>, tags: &[String]) -> anyhow::Result<Vec<Fashion>> {
    for fashion in &mut fashions {
        *fashion = ensure_tags(fashion.clone(), tags)?;
    }
    Ok(fashions)
}
