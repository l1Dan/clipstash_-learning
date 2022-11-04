use derive_more::Constructor;
use serde::{Deserialize, Serialize};

use crate::domain::time::Time;

#[derive(Debug, Clone, Constructor, Deserialize, Serialize)]
pub struct Posted(Time);

impl Posted {
    pub fn into_inner(self) -> Time {
        self.0
    }
}
