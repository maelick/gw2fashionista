use crate::domain::templates::wardrobe::slot::WardrobeSlot;

#[derive(Debug, Clone, Copy)]
pub enum AppearanceKind {
    Dyeable,
    NonDyeable,
}

#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("{slot:?} should be {expected:?} but was {found:?}")]
    IncorrectSlotVariant {
        slot: WardrobeSlot,
        expected: AppearanceKind,
        found: AppearanceKind,
    },
}
