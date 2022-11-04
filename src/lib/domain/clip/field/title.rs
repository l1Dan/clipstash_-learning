use std::str::FromStr;

use rocket::form::FromFormField;
use serde::{Deserialize, Serialize};

use crate::domain::clip::ClipError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T: Into<Option<String>>>(title: T) -> Self {
        let title: Option<String> = title.into();
        match title {
            Some(title) => {
                if !title.trim().is_empty() {
                    Self(Some(title))
                } else {
                    Self(None)
                }
            }
            None => Self(None),
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    pub fn has_title(&self) -> bool {
        self.0.is_some()
    }
}

impl Default for Title {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}

#[rocket::async_trait]
impl<'v> FromFormField<'v> for Title {
    fn from_value(field: rocket::form::ValueField<'v>) -> rocket::form::Result<'v, Self> {
        Ok(Self::new(field.value.to_owned()))
    }
}
