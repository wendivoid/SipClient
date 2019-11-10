pub enum AddressType {
    Ipv4,
    Ipv6
}

named!(pub parse_address_type<AddressType>, alt!(
        map!(tag!("IPV4"), |_| AddressType::Ipv4) |
        map!(tag!("IPV6"), |_| AddressType::Ipv6)
));
