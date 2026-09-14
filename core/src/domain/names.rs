use std::{borrow::Cow, collections::HashSet, sync::LazyLock};

use nutype::nutype;
use regex::Regex;

#[derive(Debug, thiserror::Error)]
pub enum FashionNameError {
    #[error("Fashion name is empty")]
    Empty,
    #[error("Fashion name is a UUID")]
    IsUuid,
}

#[nutype(
    sanitize(trim, with = normalize_fashion_name),
    validate(with = validate_fashion_name, error = FashionNameError),
    derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Deserialize, Serialize, TryFrom, Into, FromStr, Display, AsRef, Deref)
)]
pub struct FashionName(String);

fn normalize_fashion_name(s: String) -> String {
    normalize_whitespaces(&s).to_string()
}

fn validate_fashion_name(s: &str) -> Result<(), FashionNameError> {
    if s.is_empty() {
        Err(FashionNameError::Empty)
    } else if uuid::Uuid::parse_str(s).is_ok() {
        Err(FashionNameError::IsUuid)
    } else {
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CharacterNameError {
    #[error("Invalid character name length: {0:?}")]
    InvalidLength(usize),
    #[error("Character name includes invalid characters: {0:?}")]
    InvalidCharacter(char),
}

#[nutype(
    sanitize(trim, with=normalize_char_name),
    validate(with = validate_character_name, error = CharacterNameError),
    derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Deserialize, Serialize, TryFrom, Into, FromStr, Display, AsRef, Deref, Borrow)
)]
pub struct CharacterName(String);

static CHARACTER_NAME_DIACRITICS: LazyLock<HashSet<char>> = LazyLock::new(|| {
    // The line below is copied verbatim from the GW2 wiki:
    // https://wiki.guildwars2.com/wiki/Character_creation#Name
    "Áá Ââ Ää Àà Ææ Çç Êê Éé Ëë Èè Ïï Íí Îî Ññ Œœ Ôô Öö Óó Úú Üü Ûû Ùù"
        .replace(' ', "")
        .chars()
        .collect()
});

fn is_allowed_character_name_char(c: char) -> bool {
    c == ' ' || c.is_ascii_alphabetic() || CHARACTER_NAME_DIACRITICS.contains(&c)
}

fn validate_character_name(s: &str) -> Result<(), CharacterNameError> {
    let len = s.len();
    if !(3..=19).contains(&len) {
        return Err(CharacterNameError::InvalidLength(len));
    }
    for c in s.chars() {
        if !is_allowed_character_name_char(c) {
            return Err(CharacterNameError::InvalidCharacter(c));
        }
    }
    Ok(())
}

fn normalize_char_name(s: String) -> String {
    capitalize_character_name(normalize_whitespaces(&s).as_ref())
}

fn capitalize_character_name(s: &str) -> String {
    let words: Vec<_> = s.split_whitespace().map(capitalize_word).collect();
    words.join(" ")
}

fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::default(),
        Some(first) => {
            let first = first.to_string().to_uppercase();
            first + &chars.as_str().to_lowercase()
        }
    }
}

#[nutype(
    sanitize(trim, lowercase, with = normalize_tag_name),
    validate(not_empty),
    derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Deserialize, Serialize, TryFrom, Into, FromStr, Display, AsRef, Deref)
)]
pub struct TagName(String);

fn normalize_tag_name(s: String) -> String {
    normalize_whitespaces(&s).to_string()
}

static RE_WHITESPACES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());

fn normalize_whitespaces<'a>(s: &'a str) -> Cow<'a, str> {
    RE_WHITESPACES.replace_all(s, " ")
}
