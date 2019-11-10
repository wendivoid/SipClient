use super::ConnectionData;
use super::Attribute;
use super::Bandwidth;
use super::EncryptionKey;

mod media_type;
pub use self::media_type::MediaType;

mod protocol;
pub use self::protocol::Protocol;
pub use self::protocol::parse_protocol;

mod codec;
pub use self::codec::Codec;
pub use self::codec::CodecType;

pub struct MediaInformation {
    pub media_type: MediaType,
    pub port: u64,
    pub port_count: Option<u64>,
    pub proto: Protocol,
}

pub struct Media {
    pub media_information: MediaInformation,
    pub media_title: Option<String>,
    pub connection_data: Option<ConnectionData>,
    pub bandwidths: Vec<Bandwidth>,
    pub attributes: Vec<Attribute>,
    pub encryption_key: Option<EncryptionKey>,
    pub formats: Vec<Codec>
}
