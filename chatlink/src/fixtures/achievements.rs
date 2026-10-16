use crate::{ChatLinkType, fixtures::ChatLinkFixture};

pub const ALL: &[ChatLinkFixture] = &[
    BAR_BRAWL_COMBOS,
    BIRTHDAY_YEAR1,
    VOIDWALKER,
    SHATTERED_WEAPON_COLLECTOR,
];

pub const BAR_BRAWL_COMBOS: ChatLinkFixture = new("Bar Brawl Combos", "DrYAAAA="); // 182
pub const BIRTHDAY_YEAR1: ChatLinkFixture = new("Birthday—Year 1", "DpEGAAA="); // 1681
pub const VOIDWALKER: ChatLinkFixture = new("Voidwalker", "DoQZAAA="); // 6532
pub const SHATTERED_WEAPON_COLLECTOR: ChatLinkFixture =
    new("Shattered Weapon Collector", "DuMbAAA="); // 7139

pub const fn new(name: &'static str, chat_link: &'static str) -> ChatLinkFixture {
    ChatLinkFixture::new(name, ChatLinkType::Achievement, chat_link)
}
