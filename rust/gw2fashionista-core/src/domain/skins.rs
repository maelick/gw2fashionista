use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkinId(u16);

impl SkinId {
    pub fn new(id: u16) -> Self {
        SkinId(id)
    }

    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        Ok(SkinId(cursor.read_u16::<LittleEndian>()?))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DyeId(u16);

impl DyeId {
    pub fn new(id: u16) -> Self {
        DyeId(id)
    }

    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        Ok(DyeId(cursor.read_u16::<LittleEndian>()?))
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
