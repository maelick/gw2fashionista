use crate::{ChatLinkType, fixtures::ChatLinkFixture};

pub const ALL: &[ChatLinkFixture] = &[WITCHS_OUTFIT];

pub const WITCHS_OUTFIT: ChatLinkFixture = new("Witch's Outfit", "CwQAAAA="); // 4

pub const fn new(name: &'static str, chat_link: &'static str) -> ChatLinkFixture {
    ChatLinkFixture::new(name, ChatLinkType::Outfit, chat_link)
}
