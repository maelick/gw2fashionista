use crate::{ChatLinkType, fixtures::ChatLinkFixture};

pub const ALL: &[ChatLinkFixture] = &[ZERO, ONE, ONE_TWO_THREE, MAX];

/// 0 gold, 0 silver, 0 copper
pub const ZERO: ChatLinkFixture = new("coins_0", "AQAAAAA=");

/// 0 gold, 0 silver, 1 copper
pub const ONE: ChatLinkFixture = new("coins_1", "AQEAAAA=");

/// 1 gold, 2 silvers, 3 coppers
pub const ONE_TWO_THREE: ChatLinkFixture = new("coins_123", "AdsnAAA=");

/// 429496 gold, 72 silver, 95 copper (max value)
pub const MAX: ChatLinkFixture = new("coins_max", "Af////8=");

pub const fn new(name: &'static str, chat_link: &'static str) -> ChatLinkFixture {
    ChatLinkFixture::new(name, ChatLinkType::Coin, chat_link)
}
