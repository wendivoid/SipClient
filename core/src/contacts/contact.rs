use libsip::Uri;
use serde::{ Serialize, Deserialize };

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: u32,
    pub display_name: Option<String>,
    pub uri: Uri
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewContact {
    pub display_name: Option<String>,
    pub uri: Uri
}
