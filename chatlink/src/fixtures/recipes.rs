use crate::{ChatLinkType, fixtures::ChatLinkFixture};

pub const ALL: &[ChatLinkFixture] = &[SOFT_WOOD_PLANK, ANCIENT_WOOD_PLANK, BOLT_OF_COTTON];

pub const SOFT_WOOD_PLANK: ChatLinkFixture = new("Recipe: Soft Wood Plank", "CQEAAAA="); // 1
pub const ANCIENT_WOOD_PLANK: ChatLinkFixture = new("Recipe: Ancient Wood Plank", "CQIAAAA="); // 2
pub const BOLT_OF_COTTON: ChatLinkFixture = new("Recipe: Bolt of Cotton", "CQcAAAA="); // 7

pub const fn new(name: &'static str, chat_link: &'static str) -> ChatLinkFixture {
    ChatLinkFixture::new(name, ChatLinkType::Recipe, chat_link)
}
