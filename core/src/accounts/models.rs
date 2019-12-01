use serde::{ Serialize, Deserialize };
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum AccountType {
    Sip
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewAccount {
    pub ty: AccountType,
    pub username: String,
    pub password: String,
    pub host: String,
    pub activate: bool,
    pub vars: HashMap<String, String>
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: u32,
    pub ty: AccountType,
    pub username: String,
    pub password: String,
    pub host: String,
    pub activate: bool,
    pub vars: HashMap<String, String>
}

impl Account {

    pub fn get_socket_address(&self) -> String {
        if self.host.contains(":") {
            self.host.clone()
        } else {
            format!("{}:5060", self.host)
        }
    }
}
