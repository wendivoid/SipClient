use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum SdpNetworkType {
    Internet
}

named!(pub parse_network_type<SdpNetworkType>, alt!(
    map!(tag!("IN"), |_| SdpNetworkType::Internet)
));

impl fmt::Display for SdpNetworkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpNetworkType::Internet => write!(f, "IN")
        }
    }
}
