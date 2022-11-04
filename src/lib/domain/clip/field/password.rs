use std::str::FromStr;

use rocket::form::FromFormField;
use serde::{Deserialize, Serialize};

use crate::domain::clip::ClipError;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Password(Option<String>);

impl Password {
    pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        let password: Option<String> = password.into();
        match password {
            Some(password) => {
                if !password.trim().is_empty() {
                    Ok(Self(Some(password)))
                } else {
                    Ok(Self(None))
                }
            }
            None => Ok(Self(None)),
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }
}

impl Default for Password {
    fn default() -> Self {
        Self(None)
    }
}

impl FromStr for Password {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

#[rocket::async_trait]
impl<'v> FromFormField<'v> for Password {
    fn from_value(field: rocket::form::ValueField<'v>) -> rocket::form::Result<'v, Self> {
        Ok(Self::new(field.value.to_owned())
            .map_err(|e| rocket::form::Error::validation(format!("{}", e)))?)
    }
}
