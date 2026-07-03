use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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

impl IntoIterator for Dyes {
    type Item = DyeId;
    type IntoIter = std::array::IntoIter<DyeId, 4>;

    fn into_iter(self) -> Self::IntoIter {
        let Dyes(dye1, dye2, dye3, dye4) = self;
        [dye1, dye2, dye3, dye4].into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    NonDyable {
        skin: SkinId,
        visible: bool,
    },
    Dyable {
        skin: SkinId,
        visible: bool,
        dyes: Dyes,
    },
}

impl Slot {
    pub fn empty(dyable: bool) -> Self {
        if dyable {
            Self::Dyable {
                skin: SkinId::default(),
                visible: true,
                dyes: Dyes::default(),
            }
        } else {
            Self::NonDyable {
                skin: SkinId::default(),
                visible: true,
            }
        }
    }

    pub fn skin(self) -> SkinId {
        match self {
            Slot::NonDyable { skin, visible: _ }
            | Slot::Dyable {
                skin,
                visible: _,
                dyes: _,
            } => skin,
        }
    }

    pub fn is_visible(self) -> bool {
        match self {
            Slot::NonDyable { skin: _, visible }
            | Slot::Dyable {
                skin: _,
                visible,
                dyes: _,
            } => visible,
        }
    }

    pub fn dyes(self) -> Option<Dyes> {
        match self {
            Slot::Dyable {
                skin: _,
                visible: _,
                dyes,
            } => Some(dyes),
            Slot::NonDyable {
                skin: _,
                visible: _,
            } => None,
        }
    }

    pub fn is_empty(self) -> bool {
        match self {
            Slot::NonDyable { skin, visible: _ } => skin.is_empty(),
            Slot::Dyable {
                skin,
                visible: _,
                dyes,
            } => skin.is_empty() && dyes.is_empty(),
        }
    }

    pub fn merge(&self, other: &Slot, ignore_skin: bool, ignore_dies: bool) -> Slot {
        if other.is_empty() || (ignore_skin && ignore_dies) {
            *self
        } else if ignore_skin {
            match other {
                Slot::NonDyable {
                    skin: _,
                    visible: _,
                } => Slot::NonDyable {
                    skin: self.skin(),
                    visible: self.is_visible(),
                },
                Slot::Dyable {
                    skin: _,
                    visible: _,
                    dyes,
                } => Slot::Dyable {
                    skin: self.skin(),
                    visible: self.is_visible(),
                    dyes: *dyes,
                },
            }
        } else if ignore_dies {
            match self {
                Slot::NonDyable {
                    skin: _,
                    visible: _,
                } => Slot::NonDyable {
                    skin: other.skin(),
                    visible: other.is_visible(),
                },
                Slot::Dyable {
                    skin: _,
                    visible: _,
                    dyes,
                } => Slot::Dyable {
                    skin: other.skin(),
                    visible: other.is_visible(),
                    dyes: *dyes,
                },
            }
        } else {
            *other
        }
    }

    pub fn read(
        cursor: &mut Cursor<&[u8]>,
        dyable: bool,
        visible: bool,
    ) -> Result<Self, std::io::Error> {
        let skin = SkinId::from_cursor(cursor)?;
        if dyable {
            let dyes = Dyes::from_cursor(cursor)?;
            Ok(Self::Dyable {
                skin,
                visible,
                dyes,
            })
        } else {
            Ok(Self::NonDyable { skin, visible })
        }
    }

    pub fn serialize(&self, buffer: &mut dyn std::io::Write) -> Result<(), std::io::Error> {
        match self {
            Slot::NonDyable { skin, visible: _ } => {
                buffer.write_u16::<LittleEndian>((*skin).into())?;
            }
            Slot::Dyable {
                skin,
                visible: _,
                dyes,
            } => {
                let (dye1, dye2, dye3, dye4) = (*dyes).into();
                buffer.write_u16::<LittleEndian>((*skin).into())?;
                buffer.write_u16::<LittleEndian>(dye1)?;
                buffer.write_u16::<LittleEndian>(dye2)?;
                buffer.write_u16::<LittleEndian>(dye3)?;
                buffer.write_u16::<LittleEndian>(dye4)?;
            }
        }
        Ok(())
    }
}
