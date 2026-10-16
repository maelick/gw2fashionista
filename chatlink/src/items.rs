use std::io::{self, Cursor};

use byteorder::{ReadBytesExt, WriteBytesExt};
use serde::Serialize;

use crate::{
    ChatLinkError,
    id::{WardrobeSkinId, kind::id_kind},
};

id_kind!(ItemId = ItemKind: u32, width = 3, default = 0);

id_kind!(UpgradeId = Upgrade: u32, width = 3, padding = 1, default = 0);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Default)]
pub struct Item {
    pub count: u8,
    pub id: ItemId,
    pub skin: Option<WardrobeSkinId>,
    pub first_upgrade: Option<UpgradeId>,
    pub second_upgrade: Option<UpgradeId>,
}

impl Item {
    pub fn new(id: ItemId, count: u8) -> Self {
        Item {
            count,
            id,
            skin: None,
            first_upgrade: None,
            second_upgrade: None,
        }
    }

    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        let count = cursor.read_u8()?;
        let id = ItemId::from_cursor(cursor)?;
        let flags = cursor.read_u8()?;

        let skin = read_if_flag(flags, 7, || WardrobeSkinId::from_cursor(cursor))?;
        let first_upgrade = read_if_flag(flags, 6, || UpgradeId::from_cursor(cursor))?;
        let second_upgrade = read_if_flag(flags, 5, || UpgradeId::from_cursor(cursor))?;

        Ok(Self {
            count,
            id,
            skin,
            first_upgrade,
            second_upgrade,
        })
    }

    pub fn encode<T: io::Write + ?Sized>(&self, buffer: &mut T) -> Result<(), io::Error> {
        buffer.write_u8(self.count)?;
        self.id.encode(buffer)?;
        buffer.write_u8(self.flags())?;
        self.skin.map(|skin| skin.encode(buffer)).transpose()?;
        self.first_upgrade
            .map(|upgrade| upgrade.encode(buffer))
            .transpose()?;
        self.second_upgrade
            .map(|upgrade| upgrade.encode(buffer))
            .transpose()?;
        Ok(())
    }

    fn flags(&self) -> u8 {
        flag_from(self.skin, 7)
            | flag_from(self.first_upgrade, 6)
            | flag_from(self.second_upgrade, 5)
    }
}

impl TryFrom<&[u8]> for Item {
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from_cursor(&mut Cursor::new(bytes))?)
    }
}

fn read_if_flag<T, E>(
    flags: u8,
    pos: u8,
    f: impl FnOnce() -> Result<T, E>,
) -> Result<Option<T>, E> {
    has_flag(flags, pos).then(f).transpose()
}

fn has_flag(flags: u8, pos: u8) -> bool {
    flags & 1 << pos != 0
}

fn flag_from<T>(value: Option<T>, pos: u8) -> u8 {
    (value.is_some() as u8) << pos
}
