use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn default_ip_addr() -> IpAddr {
    IpAddr::V4(default_ipv4_addr())
}

pub fn default_ipv4_addr() -> Ipv4Addr {
    Ipv4Addr::new(127, 0, 0, 1)
}

pub fn default_ipv6_addr() -> Ipv6Addr {
    Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)
}
