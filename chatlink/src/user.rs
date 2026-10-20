use std::io::{self, Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::Serialize;

use crate::ChatLinkError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Default)]
pub struct User {
    pub user_id: uuid::Uuid,
    pub character_name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CharacterNameError {
    #[error("Invalid length: {0:}")]
    InvalidLength(usize),

    #[error("Missing two-byte null terminator: {0:?}")]
    MissingTerminatorBytes(Vec<u16>),

    #[error("Invalid UTF16LE: {0:?}")]
    InvalidEncoding(#[from] std::string::FromUtf16Error),

    #[error("Invalid character name data: {0:?}")]
    InvalidData(#[from] io::Error),
}

impl User {
    pub fn new(user_id: uuid::Uuid, character_name: String) -> Self {
        User {
            user_id,
            character_name,
        }
    }

    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        let user_id = Self::read_user_id(cursor)?;
        let character_name = Self::read_character_name(cursor)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(Self::new(user_id, character_name))
    }

    pub fn encode<T: io::Write + ?Sized>(&self, buffer: &mut T) -> Result<(), io::Error> {
        buffer.write_all(&self.user_id.to_bytes_le())?;
        for b in self.character_name.encode_utf16() {
            buffer.write_u16::<LittleEndian>(b)?;
        }
        buffer.write_u16::<LittleEndian>(0)
    }

    fn read_user_id(cursor: &mut Cursor<&[u8]>) -> Result<uuid::Uuid, std::io::Error> {
        let mut buf = [0u8; 16];
        cursor.read_exact(&mut buf)?;
        Ok(uuid::Uuid::from_bytes_le(buf))
    }

    fn read_character_name(cursor: &mut Cursor<&[u8]>) -> Result<String, CharacterNameError> {
        let remaining = cursor.get_ref().len() - cursor.position() as usize;
        if !remaining.is_multiple_of(2) {
            return Err(CharacterNameError::InvalidLength(remaining));
        }
        let mut buf = vec![0u16; remaining / 2];
        cursor.read_u16_into::<LittleEndian>(&mut buf)?;
        let buf = buf
            .strip_suffix(&[0])
            .ok_or(CharacterNameError::MissingTerminatorBytes(buf.clone()))?;
        let name = String::from_utf16(buf)?;
        Ok(name)
    }
}

impl TryFrom<&[u8]> for User {
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from_cursor(&mut Cursor::new(bytes))?)
    }
}
