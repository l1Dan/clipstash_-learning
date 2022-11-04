use rocket::form::FromFormField;
use serde::{Deserialize, Serialize};

use crate::domain::clip::ClipError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Content(String);

impl Content {
    pub fn new(content: &str) -> Result<Self, ClipError> {
        if !content.trim().is_empty() {
            Ok(Self(content.to_owned()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[rocket::async_trait]
impl<'v> FromFormField<'v> for Content {
    fn from_value(field: rocket::form::ValueField<'v>) -> rocket::form::Result<'v, Self> {
        Ok(
            Self::new(field.value)
                .map_err(|e| rocket::form::Error::validation(format!("{}", e)))?,
        )
    }
}
