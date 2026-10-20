use std::io::{self, Cursor};

use serde::Serialize;

use crate::{ChatLinkError, id::kind::id_kind};

id_kind!(ObjectiveId = ObjectiveIdKind: u32, width = 3, padding = 1, default = 0);
id_kind!(MapId = Map: u32, width = 3, padding = 1, default = 0);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Default)]
pub struct Objective {
    pub objective_id: ObjectiveId,
    pub map_id: MapId,
}

impl Objective {
    pub fn new(objective_id: ObjectiveId, map_id: MapId) -> Self {
        Objective {
            objective_id,
            map_id,
        }
    }

    pub fn from_cursor(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        let objective_id = ObjectiveId::from_cursor(cursor)?;
        let map_id = MapId::from_cursor(cursor)?;
        Ok(Self {
            objective_id,
            map_id,
        })
    }

    pub fn encode<T: io::Write + ?Sized>(&self, buffer: &mut T) -> Result<(), io::Error> {
        self.objective_id.encode(buffer)?;
        self.map_id.encode(buffer)
    }
}

impl TryFrom<&[u8]> for Objective {
    type Error = ChatLinkError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::from_cursor(&mut Cursor::new(bytes))?)
    }
}
