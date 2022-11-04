use derive_more::Constructor;
use serde::{Deserialize, Serialize};

use crate::data::DbId;

#[derive(Debug, Clone, Constructor, Deserialize, Serialize)]
pub struct ClipId(DbId);

impl ClipId {
    pub fn info_inner(self) -> DbId {
        self.0
    }
}

impl From<DbId> for ClipId {
    fn from(id: DbId) -> Self {
        Self(id)
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}
