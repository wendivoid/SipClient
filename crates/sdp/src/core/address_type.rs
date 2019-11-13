use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum SdpAddressType {
    Ipv4,
    Ipv6
}

named!(pub parse_address_type<SdpAddressType>, alt!(
    map!(tag!("IP4"), |_| SdpAddressType::Ipv4) |
    map!(tag!("IP6"), |_| SdpAddressType::Ipv6)
));

impl fmt::Display for SdpAddressType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpAddressType::Ipv4 => write!(f, "IP4"),
            SdpAddressType::Ipv6 => write!(f, "IP6")
        }
    }
}
