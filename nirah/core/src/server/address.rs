use pnet::datalink::interfaces;
use ipnetwork::IpNetwork;

pub struct AddressManager {
    start: u16,
    cur: u16
}

impl AddressManager {

    pub fn new(start: u16) -> AddressManager {
        AddressManager {
            start, cur: 0
        }
    }

    pub fn port(&mut self) -> u16 {
        self.cur += 1;
        self.cur-1+self.start
    }

    pub fn network_from_name(&mut self, name: &str) -> Option<String> {
        interfaces().into_iter()
            .filter(|item| item.name == name)
            .map(|item| {
                for ip in item.ips {
                    if let IpNetwork::V4(net) = ip {
                        return format!("{}", net.ip());
                    }
                }
                unreachable!()
            })
            .next()
    }
}
