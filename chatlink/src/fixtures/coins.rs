use crate::{ChatLinkType, fixtures::ChatLinkFixture};

super::fixtures! { $ coins, ChatLinkType::Coin;
    ZERO = ("coins_0", "AQAAAAA="),
    ONE = ("coins_1", "AQEAAAA="),
    ONE_TWO_THREE = ("coins_123", "AdsnAAA="),
    MAX = ("coins_max", "Af////8="),
}
