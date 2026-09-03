use gw2fashionista_chatlink::templates::FashionSlot;

#[derive(Debug, Clone, Copy)]
pub enum AppearanceKind {
    Dyeable,
    NonDyeable,
}

#[derive(Debug, thiserror::Error)]
pub enum ModelError<S: FashionSlot> {
    #[error("{slot:?} should be {expected:?} but was {found:?}")]
    IncorrectSlotVariant {
        slot: S,
        expected: AppearanceKind,
        found: AppearanceKind,
    },
}
