use std::collections::HashSet;

use crate::domain::error::ChatLinkError;
use crate::domain::skins::SkinId;
use crate::domain::templates::Template;
use slot::WardrobeSlot;

pub mod slot;

pub type WardrobeTemplate = Template<WardrobeSlot>;

impl WardrobeTemplate {
    pub fn all_skin_ids(&self) -> HashSet<SkinId> {
        HashSet::from_iter(self.iter().filter_map(|(slot, appearance)| match slot {
            WardrobeSlot::Outfit => None,
            _ => Some(appearance.skin()).filter(|skin| !skin.is_empty()),
        }))
    }
}

impl TryFrom<&[u8]> for WardrobeTemplate {
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<&WardrobeTemplate> for Vec<u8> {
    type Error = std::io::Error;

    fn try_from(template: &WardrobeTemplate) -> Result<Self, std::io::Error> {
        template.serialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload_size() {
        assert_eq!(WardrobeTemplate::payload_size(), 96)
    }
}
