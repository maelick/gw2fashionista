use crate::{ChatLinkType, fixtures::ChatLinkFixture};

pub const ALL_TEMPLATES: &[ChatLinkFixture] = &[AEGIS, FIREBALL, METEOR_SHOWER];

pub const AEGIS: ChatLinkFixture = new("Aegis", "BucCAAA="); // 743
pub const FIREBALL: ChatLinkFixture = new("Fireball", "BnMVAAA="); // 5491
pub const METEOR_SHOWER: ChatLinkFixture = new("Meteor Shower", "Bn0VAAA="); // 5501

pub const fn new(name: &'static str, chat_link: &'static str) -> ChatLinkFixture {
    ChatLinkFixture::new(name, ChatLinkType::Skill, chat_link)
}
