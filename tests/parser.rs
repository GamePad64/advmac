use advmac::{mac6, mac8, MacAddr6, MacAddr8, ParseError};
use std::str::FromStr;

const TEST_MAC6: MacAddr6 = mac6!("12-34-56-78-9A-BC");
const TEST_MAC8: MacAddr8 = mac8!("12-34-56-78-9A-BC-DE-FF");

#[test]
fn test_parse_canonical() {
    assert_eq!(TEST_MAC6, "12-34-56-78-9A-BC".parse().unwrap());
    assert_eq!(TEST_MAC6, "12-34-56-78-9a-bc".parse().unwrap());
    assert_eq!(TEST_MAC8, "12-34-56-78-9A-BC-DE-FF".parse().unwrap());
    assert_eq!(TEST_MAC8, "12-34-56-78-9a-bc-de-ff".parse().unwrap());
}

#[test]
fn test_parse_colon_notation() {
    assert_eq!(TEST_MAC6, "12:34:56:78:9A:BC".parse().unwrap());
    assert_eq!(TEST_MAC6, "12:34:56:78:9a:bc".parse().unwrap());
    assert_eq!(TEST_MAC8, "12:34:56:78:9A:BC:DE:FF".parse().unwrap());
    assert_eq!(TEST_MAC8, "12:34:56:78:9a:bc:de:ff".parse().unwrap());
}

#[test]
fn test_parse_dot_notation() {
    assert_eq!(TEST_MAC6, "1234.5678.9ABC".parse().unwrap());
    assert_eq!(TEST_MAC6, "1234.5678.9abc".parse().unwrap());
    assert_eq!(TEST_MAC8, "1234.5678.9ABC.DEFF".parse().unwrap());
    assert_eq!(TEST_MAC8, "1234.5678.9abc.deff".parse().unwrap());
}

#[test]
fn test_parse_hexadecimal() {
    assert_eq!(TEST_MAC6, "123456789ABC".parse().unwrap());
    assert_eq!(TEST_MAC6, "123456789abc".parse().unwrap());
    assert_eq!(TEST_MAC8, "123456789ABCDEFF".parse().unwrap());
    assert_eq!(TEST_MAC8, "123456789abcdeff".parse().unwrap());
}

#[test]
fn test_parse_hexadecimal0x() {
    assert_eq!(TEST_MAC6, "0x123456789ABC".parse().unwrap());
    assert_eq!(TEST_MAC6, "0x123456789abc".parse().unwrap());
    assert_eq!(TEST_MAC8, "0x123456789ABCDEFF".parse().unwrap());
    assert_eq!(TEST_MAC8, "0x123456789abcdeff".parse().unwrap());
}

#[test]
fn test_errors() {
    // Inconsistent separators
    assert_eq!(
        MacAddr6::from_str("11-22:03:00:50:6A"),
        Err(ParseError::InvalidMac)
    );

    // Invalid length
    assert_eq!(
        MacAddr6::from_str("1122:03:00:50:6A"),
        Err(ParseError::InvalidLength { length: 16 })
    );
}
