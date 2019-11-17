use std::fmt;
use std::str::FromStr;
use serde::{ Serialize, Deserialize };

use crate::core::NirahError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct VariableKey(String);

impl VariableKey {

    pub fn new<S: Into<String>>(string: S) -> VariableKey {
        VariableKey(string.into())
    }
}

impl AsRef<str> for VariableKey {

    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl FromStr for VariableKey {
    type Err = NirahError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(VariableKey(s.into()))
    }
}

impl fmt::Display for VariableKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
