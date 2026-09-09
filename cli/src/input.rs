use std::io::{self, Read};

pub enum Format {
    Json,
    Csv,
    ChatLink,
    None,
}

#[derive(Debug, Clone)]
pub enum Input<T> {
    None,
    Zero,
    One(T),
    Many(Vec<T>),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> Input<T>
where
    T: serde::de::DeserializeOwned,
{
    pub fn from_json<R: io::Read>(reader: R) -> Result<Self, serde_json::Error> {
        Ok(serde_json::from_reader::<_, OneOrMany<T>>(reader)?.into())
    }

    pub fn from_csv<R: io::Read>(reader: R) -> Result<Self, csv::Error> {
        let items: Result<Vec<T>, _> = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(reader)
            .deserialize()
            .collect();
        Ok(items?.into())
    }
}

impl<T> From<OneOrMany<T>> for Input<T> {
    fn from(value: OneOrMany<T>) -> Self {
        match value {
            OneOrMany::One(item) => Self::One(item),
            OneOrMany::Many(items) => match items.len() {
                0 => Self::Zero,
                1 => Self::One(items.into_iter().next().unwrap()),
                _ => Self::Many(items),
            },
        }
    }
}

impl<T> From<Vec<T>> for Input<T> {
    fn from(items: Vec<T>) -> Self {
        if items.is_empty() {
            Input::Zero
        } else if items.len() == 1 {
            Input::One(items.into_iter().next().unwrap())
        } else {
            Input::Many(items)
        }
    }
}

impl<T> From<Input<T>> for Vec<T> {
    fn from(value: Input<T>) -> Self {
        match value {
            Input::Zero | Input::None => Vec::new(),
            Input::One(item) => vec![item],
            Input::Many(items) => items,
        }
    }
}

pub fn read_templates<T, R: io::BufRead>(
    reader: &mut R,
    format: Format,
) -> anyhow::Result<(Input<T>, Format)>
where
    T: serde::de::DeserializeOwned,
{
    Ok((
        match format {
            Format::Json => Input::from_json(reader)?,
            Format::Csv => Input::from_csv(reader)?,
            Format::ChatLink => anyhow::bail!("Invalid fashion template"),
            Format::None => Input::None,
        },
        format,
    ))
}

pub fn detect_format<R: io::BufRead>(reader: &mut R) -> anyhow::Result<(Format, impl io::BufRead)> {
    let first_line = read_first_line(reader)?;
    let format = if first_line.is_empty() {
        Format::None
    } else if line_is_chatlink(&first_line) {
        Format::ChatLink
    } else if line_is_json(&first_line) {
        Format::Json
    } else {
        Format::Csv
    };
    let reader = io::Cursor::new(first_line).chain(reader);
    Ok((format, reader))
}

fn read_first_line<R: io::BufRead>(reader: &mut R) -> anyhow::Result<Vec<u8>> {
    let mut line = Vec::new();
    while reader.read_until(b'\n', &mut line)? > 0 {
        if !line.is_empty() && !line.iter().all(|&b| b.is_ascii_whitespace()) {
            return Ok(line);
        }
        line.clear();
    }
    Ok(Vec::new())
}

fn line_is_json(line: &[u8]) -> bool {
    line.iter()
        .find(|&&b| !b.is_ascii_whitespace())
        .map_or(false, |&b| b == b'{' || b == b'[')
}

fn line_is_chatlink(line: &[u8]) -> bool {
    if let Ok(s) = std::str::from_utf8(line) {
        gw2fashionista_chatlink::CHAT_LINK_REGEX.is_match(s.trim())
    } else {
        false
    }
}
