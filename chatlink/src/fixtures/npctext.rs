use crate::{ChatLinkType, fixtures::ChatLinkFixture};

pub const ALL: &[ChatLinkFixture] = &[NPCTEXT10007, NPCTEXT10008, NPCTEXT10009, NPCTEXT10016];

pub const NPCTEXT10007: ChatLinkFixture = new("Fight what cannot be fought.", "AxcnAAA="); // 10007
pub const NPCTEXT10008: ChatLinkFixture = new("Any secrets you want to share?", "AxgnAAA="); // 10008
pub const NPCTEXT10009: ChatLinkFixture = new("Any news?", "AxknAAA="); // 10009
pub const NPCTEXT10016: ChatLinkFixture = new("Do you need assistance?", "AyAnAAA="); // 10016

pub const fn new(name: &'static str, chat_link: &'static str) -> ChatLinkFixture {
    ChatLinkFixture::new(name, ChatLinkType::NpcText, chat_link)
}
