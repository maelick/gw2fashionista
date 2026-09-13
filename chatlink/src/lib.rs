mod chatlink;
mod error;
pub mod skins;
pub mod templates;

use std::sync::LazyLock;

pub use chatlink::{ChatLink, ChatLinkType};
pub use error::ChatLinkError;
use regex::Regex;

const BASE64_RE: &str = r"[-A-Za-z0-9+/]*={0,3}";

pub static CHAT_LINK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    let pattern = format!(r"^\[&({})\]$", BASE64_RE);
    Regex::new(&pattern).unwrap()
});
