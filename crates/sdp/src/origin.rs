use nirah_parse::*;
use nirah_uri::Domain;
use nirah_uri::parse_domain;
use nom::character::is_digit;
use nom::character::is_alphanumeric;

use super::NetworkType;
use super::AddressType;
use super::parse_address_type;
use super::parse_network_type;


pub struct Origin {
    pub username: String,
    pub session_id: u64,
    pub session_version: u64,
    pub network_type: NetworkType,
    pub address_type: AddressType,
    pub unicast_address: Domain
}

named!(pub parse_origin<Origin>, do_parse!(
    name: map_res!(take_while!(is_alphanumeric), slice_to_string) >>
    char!(' ') >>
    id: map_res!(take_while!(is_digit), parse_u64) >>
    char!(' ') >>
    version: map_res!(take_while!(is_digit), parse_u64) >>
    char!(' ') >>
    net: parse_network_type >>
    char!(' ') >>
    addr: parse_address_type >>
    char!(' ') >>
    uaddr: parse_domain >>
    (Origin{ username: name, session_id: id.1, session_version: version.1, network_type: net, address_type: addr, unicast_address: uaddr })
));
