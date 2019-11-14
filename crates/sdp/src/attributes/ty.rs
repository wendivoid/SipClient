use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum SdpAttributeType {
    Rtpmap,
    RecvOnly,
    SendRecv,
    Fmtp
}

impl fmt::Display for SdpAttributeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpAttributeType::Rtpmap => write!(f, "rtpmap"),
            SdpAttributeType::RecvOnly => write!(f, "recvonly"),
            SdpAttributeType::SendRecv => write!(f, "sendrecv"),
            SdpAttributeType::Fmtp => write!(f, "fmtp")
        }
    }
}

named!(pub parse_attribute_type<SdpAttributeType>, alt!(
    map!(tag!("rtpmap"), |_| SdpAttributeType::Rtpmap) |
    map!(tag!("fmtp"), |_| SdpAttributeType::Fmtp) |
    map!(tag!("recvonly"), |_| SdpAttributeType::RecvOnly) |
    map!(tag!("sendrecv"), |_| SdpAttributeType::SendRecv)
));
