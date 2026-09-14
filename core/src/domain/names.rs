use std::{borrow::Cow, sync::LazyLock};

use nutype::nutype;
use regex::Regex;

#[nutype(
    sanitize(trim, with = normalize_fashion_name),
    validate(not_empty),
    derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Deserialize, Serialize, TryFrom, Into, FromStr, Display, AsRef, Deref)
)]
pub struct FashionName(String);

fn normalize_fashion_name(s: String) -> String {
    normalize_whitespaces(&s).to_string()
}

#[nutype(
    sanitize(trim, with=normalize_char_name),
    validate(not_empty, len_char_min = 3, len_char_max = 19),
    derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Deserialize, Serialize, TryFrom, Into, FromStr, Display, AsRef, Deref, Borrow)
)]
pub struct CharacterName(String);

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
