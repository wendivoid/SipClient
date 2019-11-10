use std::fmt;
use std::str::FromStr;
use std::path::PathBuf;
use serde::{ Serialize, Deserialize };
use crate::core::NirahError;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum VariableValue {
    String(String),
    FilePath(PathBuf),
    Integer(i64),
    Boolean(bool)
}

impl FromStr for VariableValue {
    type Err = NirahError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str(s) {
            Ok(data) => Ok(data),
            Err(_err) => {
                Ok(VariableValue::String(s.into()))
            }
        }
    }
}

impl fmt::Display for VariableValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariableValue::String(s) => write!(f, "{}", s),
            VariableValue::FilePath(path) => write!(f, "{:?}", path),
            VariableValue::Integer(i) => write!(f, "{}", i),
            VariableValue::Boolean(b) => write!(f, "{}", b)
        }
    }
}
