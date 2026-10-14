use std::{
    fmt, hash,
    io::{self, Cursor},
    marker::PhantomData,
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::Serialize;

use crate::ChatLinkError;

pub mod kind;
pub use kind::{
    AchievementId, MapPointId, NpcTextId, OutfitId, RecipeId, SkillId, TraitId, WardrobeSkinId,
};

pub trait IdKind {
    type Value: Copy + Eq + hash::Hash + fmt::Debug + Into<u64> + TryFrom<u64, Error: fmt::Debug>;
    const WIDTH: usize;
    const PADDING: usize = 0;
    const NAME: &'static str;
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct Id<K: IdKind> {
    value: K::Value,
    #[serde(skip)]
    _kind: PhantomData<K>,
}

impl<K> Id<K>
where
    K: IdKind,
{
    const WIDTH: usize = {
        assert!(
            K::WIDTH > 0 && K::WIDTH <= size_of::<K::Value>(),
            "id width must fit in value type"
        );
        K::WIDTH
    };

    const PADDING: usize = K::PADDING;

    const MAX: u64 = u64::MAX >> (64 - 8 * Self::WIDTH);

    pub fn new(value: K::Value) -> Self {
        Self {
            value,
            _kind: PhantomData,
        }
    }

    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        let id = cursor.read_uint::<LittleEndian>(Self::WIDTH)?;
        for _ in 0..Self::PADDING {
            cursor.read_u8()?;
        }
        Ok(Self::new(
            id.try_into().expect("width checked at compile time"),
        ))
    }

    pub fn value(&self) -> K::Value {
        self.value
    }

    pub fn encode<T: io::Write + ?Sized>(&self, buffer: &mut T) -> Result<(), io::Error> {
        let value = self.value.into();
        if value > Self::MAX {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("id {value} does not fit in {} bytes", Self::WIDTH),
            ));
        }
        buffer.write_uint::<LittleEndian>(value, Self::WIDTH)?;
        for _ in 0..Self::PADDING {
            buffer.write_u8(0)?;
        }
        Ok(())
    }
}

impl<K> fmt::Debug for Id<K>
where
    K: IdKind,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(K::NAME).field(&self.value).finish()
    }
}

impl<K> TryFrom<&[u8]> for Id<K>
where
    K: IdKind,
{
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != Self::WIDTH + Self::PADDING {
            return Err(ChatLinkError::InvalidLength(bytes.to_vec()));
        }
        Ok(Self::from_cursor(&mut Cursor::new(bytes))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::assert_matches;

    #[test]
    fn skill_id() {
        super::kind::id_kind!(TestId = Test: u32, width = 3, padding = 1);

        let id = TestId::new(1);
        assert_eq!(id.value(), 1);
        assert_eq!(format!("{:?}", id), "TestId(1)");
        let mut buf: Vec<u8> = Vec::new();
        id.encode(&mut buf).unwrap();
        assert_eq!(buf, vec![1, 0, 0, 0]);
        assert_eq!(TestId::try_from(buf.as_slice()).unwrap(), id);
    }

    #[test]
    fn encode_max() {
        super::kind::id_kind!(TestId = Test: u32, width = 3, padding = 1);

        assert!(TestId::MAX < (u32::MAX as u64));
        let id = TestId::new(TestId::MAX as u32);
        let mut buf: Vec<u8> = Vec::new();
        id.encode(&mut buf).unwrap();
        assert_eq!(buf, vec![0xFF, 0xFF, 0xFF, 0]);

        let id = TestId::new(TestId::MAX as u32 + 1);
        let mut buf: Vec<u8> = Vec::new();
        let err = id.encode(&mut buf).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn no_padding() {
        super::kind::id_kind!(TestId = Test: u16, width = 2, default = 1);

        let id = TestId::default();
        assert_eq!(id.value(), 1);
        assert_eq!(format!("{:?}", id), "TestId(1)");
        let mut buf: Vec<u8> = Vec::new();
        id.encode(&mut buf).unwrap();
        assert_eq!(buf, vec![1, 0]);
        assert_eq!(TestId::try_from(buf.as_slice()).unwrap(), id);
    }

    #[test]
    fn invalid_length() {
        super::kind::id_kind!(TestId = Test: u32, width = 3, padding = 1);

        let err = TestId::try_from(vec![1 as u8, 0, 0].as_slice()).unwrap_err();
        assert_matches!(err, ChatLinkError::InvalidLength(_));

        let err = TestId::try_from(vec![1 as u8, 0, 0, 0, 0].as_slice()).unwrap_err();
        assert_matches!(err, ChatLinkError::InvalidLength(_));
    }

    #[test]
    fn non_zero_padding() {
        super::kind::id_kind!(TestId = Test: u32, width = 3, padding = 1);

        let id = TestId::try_from(vec![1 as u8, 0, 0, 0xFF].as_slice()).unwrap();
        assert_eq!(id, TestId::new(1));
    }
}
