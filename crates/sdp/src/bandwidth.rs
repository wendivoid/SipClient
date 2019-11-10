use nirah_parse::parse_u64;


pub enum BandwidthType {
    Ct,
    As
}

named!(pub parse_bandwidth_type<BandwidthType>, alt!(
        map!(tag!("CT"), |_| BandwidthType::Ct) |
        map!(tag!("AS"), |_| BandwidthType::As)
));

pub struct Bandwidth {
    pub bwtype: BandwidthType,
    pub bandwidth: u64
}

named!(pub parse_bandwidth<Bandwidth>, do_parse!(
    bwtype: parse_bandwidth_type >>
    char!(':') >>
    bandwidth: parse_u64 >>
    (Bandwidth { bwtype, bandwidth })
));
