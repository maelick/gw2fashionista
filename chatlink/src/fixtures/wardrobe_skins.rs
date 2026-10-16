use crate::{ChatLinkType, fixtures::ChatLinkFixture};

pub const ALL_TEMPLATES: &[ChatLinkFixture] = &[APPRENTICE_COAT];

pub const APPRENTICE_COAT: ChatLinkFixture = new("Apprentice Coat", "CgQAAAA="); // 4

pub const fn new(name: &'static str, chat_link: &'static str) -> ChatLinkFixture {
    ChatLinkFixture::new(name, ChatLinkType::Wardrobe, chat_link)
}
