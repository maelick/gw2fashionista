use crate::{ChatLinkType, fixtures::ChatLinkFixture};

pub const ALL_TEMPLATES: &[ChatLinkFixture] = &[
    DR_BLEENTS_ENCAMPMENT,
    DESIDER_ATUM_WAYPOINT,
    CAUDECUS_ESTATE_WAYPOINT,
];

pub const DR_BLEENTS_ENCAMPMENT: ChatLinkFixture = new("Dr. Bleent's Encampment", "BDgAAAA="); // 56
pub const DESIDER_ATUM_WAYPOINT: ChatLinkFixture = new("Desider Atum Waypoint", "BEgAAAA="); // 72
pub const CAUDECUS_ESTATE_WAYPOINT: ChatLinkFixture = new("Caudecus' Estate Waypoint", "BDkDAAA="); // 825

pub const fn new(name: &'static str, chat_link: &'static str) -> ChatLinkFixture {
    ChatLinkFixture::new(name, ChatLinkType::MapLink, chat_link)
}
