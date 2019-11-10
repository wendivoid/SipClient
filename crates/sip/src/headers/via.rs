use nirah_uri::Uri;
use crate::core::Version;
use nirah_uri::Transport;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct ViaHeader {
    pub version: Version,
    pub transport: Transport,
    pub uri: Uri
}

impl ViaHeader {

    pub fn new(uri: Uri, transport: Transport) -> ViaHeader {
        ViaHeader {
            transport, uri,
            version: Version::default()
        }
    }
}

impl fmt::Display for ViaHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Via: {}/{} {}", self.version, self.transport, self.uri)
    }
}
