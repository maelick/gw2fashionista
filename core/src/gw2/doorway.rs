use crate::gw2::{lookup::StaticLookup, named::Named};

pub const KNOWN_DOORWAYS: &[Doorway] = &[
    Doorway {
        id: 2,
        name: "Kodan Conjured Doorway",
    },
    Doorway {
        id: 5,
        name: "Choya Piñata Conjured Doorway",
    },
];

#[derive(Clone)]
pub struct Doorway {
    pub id: u32,
    pub name: &'static str,
}

impl Doorway {
    pub fn lookup() -> StaticLookup<Self, u32> {
        StaticLookup::new(KNOWN_DOORWAYS.iter().cloned().map(|d| (d.id, d)))
    }
}

impl Named for Doorway {
    fn name(&self) -> &str {
        self.name
    }
}
