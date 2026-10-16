use crate::{ChatLinkType, fixtures::ChatLinkFixture};

super::fixtures! { $ npctext, ChatLinkType::NpcText;
    NPCTEXT10007 = ("npctext_fight_what_cannot_be_fought.", "AxcnAAA="), // 10007
    NPCTEXT10008 = ("npctext_any_secrets_you_want_to_share", "AxgnAAA="), // 10008
    NPCTEXT10009 = ("npctext_any_news", "AxknAAA="), // 10009
    NPCTEXT10016 = ("npctext_do_you_need_assistance", "AyAnAAA="), // 10016
}

pub const fn new(name: &'static str, chat_link: &'static str) -> ChatLinkFixture {
    ChatLinkFixture::new(name, ChatLinkType::NpcText, chat_link)
}
