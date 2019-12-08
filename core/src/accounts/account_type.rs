use serde::{ Serialize, Deserialize };

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum AccountType {
    Sip
}
