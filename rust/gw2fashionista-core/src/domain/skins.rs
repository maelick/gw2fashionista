use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkinId(u16);

impl SkinId {
    pub fn new(id: u16) -> Self {
        SkinId(id)
    }

    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        Ok(SkinId(cursor.read_u16::<LittleEndian>()?))
    }

    pub fn is_empty(self) -> bool {
        matches!(self, SkinId(id) if id == 0)
    }
}

impl From<u16> for SkinId {
    fn from(id: u16) -> Self {
        Self::new(id)
    }
}

impl From<SkinId> for u16 {
    fn from(SkinId(id): SkinId) -> u16 {
        id
    }
}

impl From<SkinId> for u32 {
    fn from(SkinId(id): SkinId) -> u32 {
        id as u32
    }
}

impl Default for SkinId {
    fn default() -> Self {
        SkinId(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DyeId(u16);

impl DyeId {
    pub fn new(id: u16) -> Self {
        DyeId(id)
    }

    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        Ok(DyeId(cursor.read_u16::<LittleEndian>()?))
    }

    pub fn is_empty(self) -> bool {
        matches!(self, DyeId(id) if id == 1)
    }
}

impl From<u16> for DyeId {
    fn from(id: u16) -> Self {
        Self::new(id)
    }
}

impl From<DyeId> for u16 {
    fn from(DyeId(id): DyeId) -> u16 {
        id
    }
}

impl From<DyeId> for u32 {
    fn from(DyeId(id): DyeId) -> u32 {
        id as u32
    }
}

impl Default for DyeId {
    fn default() -> Self {
        DyeId(1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dyes(DyeId, DyeId, DyeId, DyeId);

impl Dyes {
    pub fn new(dye1: DyeId, dye2: DyeId, dye3: DyeId, dye4: DyeId) -> Self {
        Dyes(dye1, dye2, dye3, dye4)
    }

    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        Ok(Dyes(
            DyeId::from_cursor(cursor)?,
            DyeId::from_cursor(cursor)?,
            DyeId::from_cursor(cursor)?,
            DyeId::from_cursor(cursor)?,
        ))
    }

    pub fn is_empty(self) -> bool {
        matches!(self, Dyes(dye1, dye2, dye3, dye4) if dye1.is_empty() && dye2.is_empty() && dye3.is_empty() && dye4.is_empty())
    }
}

impl Default for Dyes {
    fn default() -> Self {
        Dyes(DyeId::default(), DyeId::default(), DyeId::default(), DyeId::default())
    }
}

impl From<(u16, u16, u16, u16)> for Dyes {
    fn from((id1, id2, id3, id4): (u16, u16, u16, u16)) -> Self {
        Self::new(id1.into(), id2.into(), id3.into(), id4.into())
    }
}

impl From<(DyeId, DyeId, DyeId, DyeId)> for Dyes {
    fn from((dye1, dye2, dye3, dye4): (DyeId, DyeId, DyeId, DyeId)) -> Self {
        Self::new(dye1, dye2, dye3, dye4)
    }
}

impl From<Dyes> for (DyeId, DyeId, DyeId, DyeId) {
    fn from(Dyes(dye1, dye2, dye3, dye4): Dyes) -> (DyeId, DyeId, DyeId, DyeId) {
        (dye1, dye2, dye3, dye4)
    }
}

impl From<Dyes> for (u16, u16, u16, u16) {
    fn from(Dyes(dye1, dye2, dye3, dye4): Dyes) -> (u16, u16, u16, u16) {
        (dye1.into(), dye2.into(), dye3.into(), dye4.into())
    }
}
