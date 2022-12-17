#![cfg(feature = "std")]

use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr};

#[test]
fn simple() {
    #[derive(Deserialize)]
    struct Foo {
        #[serde(default = "serde_field_default::default_ip_addr")]
        bar: IpAddr,
    }

    assert_eq!(
        serde_json::from_str::<Foo>("{}").unwrap().bar,
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
    );
}
