pub enum NetworkType {
    Internet
}

named!(pub parse_network_type<NetworkType>, do_parse!(
    tag!("IN") >>
    (NetworkType::Internet)
));
