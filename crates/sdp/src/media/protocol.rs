pub enum Protocol {
    Udp,
    RtpAvp,
    RtpSavp,
}

named!(pub parse_protocol<Protocol>, alt!(
        map!(tag!("UDP"), |_| Protocol::Udp) |
        map!(tag!("RTP/AVP"), |_| Protocol::RtpAvp) |
        map!(tag!("RTP/SAVP"), |_| Protocol::RtpSavp)
));
