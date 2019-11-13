mod version;
pub use self::version::SdpVersion;
pub use self::version::parse_version;
pub use self::version::parse_version_line;

mod session_name;
pub use self::session_name::SdpSessionName;
pub use self::session_name::parse_session_name;
pub use self::session_name::parse_session_name_line;

mod origin;
pub use self::origin::SdpOrigin;
pub use self::origin::parse_origin;
pub use self::origin::parse_origin_line;

mod timing;
pub use self::timing::SdpTiming;
pub use self::timing::parse_timing;
pub use self::timing::parse_time_line;

mod connection;
pub use self::connection::SdpConnection;
pub use self::connection::parse_connection;
pub use self::connection::parse_connection_name;

use crate::SdpSessionAttributes;

use nirah_parse::slice_to_string;

named!(pub parse_email_line<SdpSessionAttributes>, do_parse!(
    tag!("e=") >>
    output: map_res!(take_until!("\r"), slice_to_string) >>
    tag!("\r\n") >>
    (SdpSessionAttributes::Email(output))
));

named!(pub parse_phone_line<SdpSessionAttributes>, do_parse!(
    tag!("e=") >>
    output: map_res!(take_until!("\r"), slice_to_string) >>
    tag!("\r\n") >>
    (SdpSessionAttributes::Phone(output))
));

named!(pub parse_information_line<SdpSessionAttributes>, do_parse!(
    tag!("i=") >>
    output: map_res!(take_until!("\r"), slice_to_string) >>
    tag!("\r\n") >>
    (SdpSessionAttributes::Information(output))
));

named!(pub parse_uri_line<SdpSessionAttributes>, do_parse!(
    tag!("u=") >>
    output: map_res!(take_until!("\r"), slice_to_string) >>
    tag!("\r\n") >>
    (SdpSessionAttributes::Uri(output))
));