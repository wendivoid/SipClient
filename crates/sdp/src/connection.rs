use nirah_parse::slice_to_string;
use super::NetworkType;
use super::AddressType;
use super::parse_network_type;
use super::parse_address_type;

pub struct ConnectionData {
    pub network_type: NetworkType,
    pub address_type: AddressType,
    pub connection_address: String
}

named!(pub parse_connection_data<ConnectionData>, do_parse!(
    network_type: parse_network_type >>
    address_type: parse_address_type >>
    connection_address: map_res!(take_until!("\r"), slice_to_string) >>
    (ConnectionData { network_type, address_type, connection_address })
));
