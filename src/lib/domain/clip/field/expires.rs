use std::str::FromStr;

use rocket::form::FromFormField;
use serde::{Deserialize, Serialize};

use crate::domain::{clip::ClipError, time::Time};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Expires(Option<Time>);

impl Expires {
    pub fn new<T: Into<Option<Time>>>(expires: T) -> Self {
        Self(expires.into())
    }

    pub fn into_inner(self) -> Option<Time> {
        self.0
    }
}

impl Default for Expires {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Expires {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            Ok(Self(None))
        } else {
            match Time::from_str(s) {
                Ok(time) => Ok(Self::new(time)),
                Err(e) => Err(e.into()),
            }
        }
    }
}

#[rocket::async_trait]
impl<'v> FromFormField<'v> for Expires {
    fn from_value(field: rocket::form::ValueField<'v>) -> rocket::form::Result<'v, Self> {
        if field.value.trim().is_empty() {
            Ok(Self(None))
        } else {
            Ok(Self::from_str(field.value)
                .map_err(|e| rocket::form::Error::validation(format!("{}", e)))?)
        }
    }
}
