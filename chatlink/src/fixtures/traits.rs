use crate::{ChatLinkType, fixtures::ChatLinkFixture};

pub const ALL: &[ChatLinkFixture] = &[OPENING_STRIKE];

pub const OPENING_STRIKE: ChatLinkFixture = new("Opening Strike", "B/IDAAA="); // 1010

pub const fn new(name: &'static str, chat_link: &'static str) -> ChatLinkFixture {
    ChatLinkFixture::new(name, ChatLinkType::Trait, chat_link)
}
