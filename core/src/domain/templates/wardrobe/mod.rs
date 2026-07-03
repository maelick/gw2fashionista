use std::collections::HashSet;
use std::io::Cursor;

use byteorder::{LittleEndian, WriteBytesExt};
use linearize::StaticMap;

use crate::domain::error::ChatLinkError;
use crate::domain::skins::{Appearance, SkinId};
use crate::domain::templates::FashionSlot;
use crate::domain::templates::Template;
use slot::{WardrobeSlot, WardrobeVisibility};

const TEMPLATE_PAYLOAD_SIZE: usize = 96;

pub mod slot;

pub type WardrobeTemplate = Template<WardrobeSlot>;

impl WardrobeTemplate {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ChatLinkError> {
        if bytes.len() != TEMPLATE_PAYLOAD_SIZE {
            return Err(ChatLinkError::TruncatedData(bytes.to_vec()));
        }

        let visibility = WardrobeVisibility::from_bytes(bytes)?;
        let mut cursor = Cursor::new(bytes);

        // ugly trick due to lack of StaticMap::try_from_fn
        // TODO: rewrite if added to linearize
        let mut first_err = None;
        let slots = StaticMap::from_fn(|slot: WardrobeSlot| {
            Appearance::read(
                &mut cursor,
                slot.dyable(),
                visibility.contains(slot.visibility()),
            )
            .unwrap_or_else(|e| {
                first_err.get_or_insert(e);
                Appearance::empty(slot.dyable())
            })
        });

        match first_err {
            None => Ok(Self { slots }),
            Some(e) => Err(ChatLinkError::InvalidPayload(e)),
        }
    }

    fn visibility(&self) -> WardrobeVisibility {
        self.iter()
            .filter(|(slot, appearance)| slot.always_visible() || appearance.is_visible())
            .map(|(slot, _)| slot.visibility())
            .fold(WardrobeVisibility::empty(), |acc, v| acc | v)
    }

    pub fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::with_capacity(TEMPLATE_PAYLOAD_SIZE);

        for (_, slot) in self {
            slot.serialize(&mut buffer)?;
        }

        let visibility = self.visibility();
        buffer.write_u16::<LittleEndian>(visibility.bits())?;
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
