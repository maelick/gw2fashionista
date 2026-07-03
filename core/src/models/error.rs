use crate::domain::templates::wardrobe::slot::SlotType;

#[derive(Debug, Clone, Copy)]
pub enum SlotVariant {
    Dyable,
    NonDyable,
}

#[derive(Debug, thiserror::Error)]
pub enum ModelError {
    #[error("{slot_type:?} should be {expected:?} but was {found:?}")]
    IncorrectSlotVariant {
        slot_type: SlotType,
        expected: SlotVariant,
        found: SlotVariant,
    },
}
