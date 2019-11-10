#[macro_use]
extern crate nom;

use nirah_uri::Uri;
use nirah_uri::parse_uri;
use nirah_parse::slice_to_string;

mod timing;
pub use self::timing::TimeZone;
pub use self::timing::TimeDescription;
pub use self::timing::Timing;
pub use self::timing::Repeat;

mod bandwidth;
pub use self::bandwidth::Bandwidth;
pub use self::bandwidth::BandwidthType;

mod origin;
pub use self::origin::Origin;

mod encryption;
pub use self::encryption::EncryptionKey;

mod attribute;
pub use self::attribute::Attribute;

mod media;
pub use self::media::MediaInformation;
pub use self::media::Media;

mod network_type;
pub use self::network_type::NetworkType;
pub use self::network_type::parse_network_type;

mod address_type;
pub use self::address_type::AddressType;
pub use self::address_type::parse_address_type;

mod connection;
pub use self::connection::ConnectionData;

pub struct Version;

named!(pub parse_version<Version>, do_parse!(
    tag!("0") >>
    (Version)
));

named!(pub parse_version_line<Version>, do_parse!(
    tag!("v=") >>
    version: parse_version >>
    (version)
));

named!(pub parse_session_name<String>, do_parse!(
    tag!("s=") >>
    data: map_res!(take_until!("\r"), slice_to_string) >>
    (data)
));

named!(pub parse_session_information<String>, do_parse!(
    tag!("i=") >>
    data: map_res!(take_until!("\r"), slice_to_string) >>
    (data)
));

named!(pub parse_uri_line<Uri>, do_parse!(
    tag!("u=") >>
    data: map_res!(take_until!("\r"), parse_uri) >>
    (data.1)
));

named!(pub parse_email_line<String>, do_parse!(
    tag!("e=") >>
    data: map_res!(take_until!("\r"), slice_to_string) >>
    (data)
));

named!(pub parse_phone_line<String>, do_parse!(
    tag!("p=") >>
    data: map_res!(take_until!("\r"), slice_to_string) >>
    (data)
));

pub struct SdpMessage {
    pub version: Version,
    pub origin: Origin,
    pub session_name: String,
    pub session_information: Option<String>,
    pub uri: Option<Uri>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub connection_data: Option<ConnectionData>,
    pub bandwidth: Option<Bandwidth>,
    pub time: Vec<TimeDescription>,
    pub attributes: Vec<Attribute>,
    pub media: Vec<Media>
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
