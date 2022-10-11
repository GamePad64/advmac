#![cfg(feature = "std")]

use advmac::{mac6, IpError, MacAddr6};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

#[test]
fn test_link_local() {
    use std::net::Ipv6Addr;

    let mac = mac6!("52:74:f2:b1:a8:7f");
    let ip = Ipv6Addr::from_str("fe80::5074:f2ff:feb1:a87f").unwrap();

    assert_eq!(mac.to_link_local_ipv6(), ip);
    assert_eq!(MacAddr6::try_from_link_local_ipv6(ip).unwrap(), mac);
}

const IPV4_MULTICAST_IP: Ipv4Addr = Ipv4Addr::new(224, 11, 1, 2);
const IPV4_MULTICAST_MAC: MacAddr6 = mac6!("01:00:5e:0b:01:02");
const IPV4_MULTICAST_IP_INVALID: Ipv4Addr = Ipv4Addr::new(192, 168, 1, 1);

#[test]
fn test_multicast_ipv4() {
    assert!(IPV4_MULTICAST_IP.is_multicast());
    assert_eq!(
        MacAddr6::try_from_multicast_ip(IPV4_MULTICAST_IP.into()).unwrap(),
        IPV4_MULTICAST_MAC
    );
}

#[test]
fn test_multicast_ipv4_invalid() {
    assert!(!IPV4_MULTICAST_IP_INVALID.is_multicast());
    assert_eq!(
        MacAddr6::try_from_multicast_ip(IPV4_MULTICAST_IP_INVALID.into()),
        Err(IpError::NotMulticast)
    );
}

// ff02::1:ffe8:658f
const IPV6_MULTICAST_IP: Ipv6Addr = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 1, 0xffe8, 0x658f);
const IPV6_MULTICAST_MAC: MacAddr6 = mac6!("33:33:ff:e8:65:8f");
const IPV6_MULTICAST_IP_INVALID: Ipv6Addr =
    Ipv6Addr::new(0xfe80, 0, 0, 0, 0x4a5d, 0x60ff, 0xfee8, 0x658f);

#[test]
fn test_multicast_ipv6() {
    assert!(IPV6_MULTICAST_IP.is_multicast());
    assert_eq!(
        MacAddr6::try_from_multicast_ip(IPV6_MULTICAST_IP.into()).unwrap(),
        IPV6_MULTICAST_MAC
    );
}

#[test]
fn test_multicast_ipv6_invalid() {
    assert!(!IPV6_MULTICAST_IP_INVALID.is_multicast());
    assert_eq!(
        MacAddr6::try_from_multicast_ip(IPV6_MULTICAST_IP_INVALID.into()),
        Err(IpError::NotMulticast)
    );
}
