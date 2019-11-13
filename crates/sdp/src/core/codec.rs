use nom::character::is_digit;
use nirah_parse::parse_u32;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Codec {
    Pcmu,
    Pcma,
    Unknown(u32)
}

impl fmt::Display for Codec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Codec::Pcmu => write!(f, "0"),
            Codec::Pcma => write!(f, "8"),
            Codec::Unknown(num) => write!(f, "{}", num)
        }
    }
}

named!(pub _parse_codec<Codec>, alt!(
    map!(tag!("0"), |_| Codec::Pcmu) |
    map!(tag!("8"), |_| Codec::Pcma) |
    parse_unknown_codec
));
named!(pub parse_codec<Codec>, do_parse!(
    out: _parse_codec >>
    (out)
));

named!(parse_unknown_codec<Codec>, do_parse!(
    num: map_res!(take_while!(is_digit), parse_u32) >>
    (Codec::Unknown(num))
));
