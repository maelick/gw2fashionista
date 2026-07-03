use std::collections::HashSet;
use std::io::Cursor;

use byteorder::{LittleEndian, WriteBytesExt};
use linearize::static_map;

use crate::domain::error::ChatLinkError;
use crate::domain::skins::{Appearance, SkinId};
use crate::domain::templates::FashionSlot;
use crate::domain::templates::Template;
use slot::WardrobeSlot;

pub mod slot;

pub type WardrobeTemplate = Template<WardrobeSlot>;

impl WardrobeTemplate {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        if bytes.len() != Self::payload_size() {
            return Err(ChatLinkError::TruncatedData(bytes.to_vec()));
        }

        let visibility = Self::read_visibility(bytes)?;
        let mut cursor = Cursor::new(bytes);

        Ok(Self {
            slots: static_map! {
                slot => Appearance::read(
                    &mut cursor,
                    slot.dyeable(),
                    slot.is_visible(visibility),
                )?
            },
        })
    }

    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::with_capacity(Self::payload_size());
        for (_, slot) in self {
            slot.serialize(&mut buffer)?;
        }
        buffer.write_u16::<LittleEndian>(self.visibility())?;
        Ok(buffer)
    }

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
