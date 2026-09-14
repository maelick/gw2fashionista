use nutype::nutype;

#[nutype(
    sanitize(trim, with = normalize_fashion_name),
    validate(not_empty),
    derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Deserialize, Serialize, TryFrom, Into, FromStr, Display, AsRef, Deref)
)]
pub struct FashionName(String);

fn normalize_fashion_name(s: String) -> String {
    s
}

#[nutype(
    sanitize(trim, with=normalize_char_name),
    validate(not_empty, len_char_min = 3, len_char_max = 19),
    derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Deserialize, Serialize, TryFrom, Into, FromStr, Display, AsRef, Deref, Borrow)
)]
pub struct CharacterName(String);

fn normalize_char_name(s: String) -> String {
    s
}

#[nutype(
    sanitize(trim, lowercase, with = normalize_tag_name),
    validate(not_empty),
    derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Deserialize, Serialize, TryFrom, Into, FromStr, Display, AsRef, Deref)
)]
pub struct TagName(String);

fn normalize_tag_name(s: String) -> String {
    s
}
