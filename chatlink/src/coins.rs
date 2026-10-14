use std::io::{self, Cursor};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::Serialize;

use crate::ChatLinkError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Default)]
pub struct Coins {
    pub copper: u8,
    pub silver: u8,
    pub gold: u32,
}

impl Coins {
    pub fn from_copper(total: u32) -> Self {
        let copper = (total % 100) as u8;
        let silver = total / 100;
        let gold = silver / 100;
        let silver = (silver % 100) as u8;
        Coins {
            copper,
            silver,
            gold,
        }
    }

    pub fn as_copper(&self) -> u32 {
        (self.gold * 100 + self.silver as u32) * 100 + self.copper as u32
    }

    pub fn encode<T: io::Write + ?Sized>(&self, buffer: &mut T) -> Result<(), io::Error> {
        let total = self.as_copper();
        buffer.write_u32::<LittleEndian>(total)
    }
}

impl From<u32> for Coins {
    fn from(value: u32) -> Self {
        Self::from_copper(value)
    }
}

impl From<Coins> for u32 {
    fn from(value: Coins) -> Self {
        value.as_copper()
    }
}

impl TryFrom<&[u8]> for Coins {
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 4 {
            return Err(ChatLinkError::InvalidLength(bytes.to_vec()));
        }
        let mut cursor = Cursor::new(bytes);
        let copper = cursor.read_u32::<LittleEndian>()?;
        Ok(Self::from_copper(copper))
    }
}
