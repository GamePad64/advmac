use advmac::{
    mac6, mac8, MacAddr6, MacAddr8, MacAddrFormat, MAC_CANONICAL_SIZE6, MAC_CANONICAL_SIZE8,
    MAC_COLON_NOTATION_SIZE6, MAC_COLON_NOTATION_SIZE8, MAC_DOT_NOTATION_SIZE6,
    MAC_DOT_NOTATION_SIZE8, MAC_HEXADECIMAL0X_SIZE6, MAC_HEXADECIMAL0X_SIZE8,
    MAC_HEXADECIMAL_SIZE6, MAC_HEXADECIMAL_SIZE8, MAC_MAX_SIZE,
};
use arrayvec::ArrayString;
use rstest::rstest;

const TEST_MAC6: MacAddr6 = mac6!("12-34-56-78-9A-BC");
const TEST_MAC8: MacAddr8 = mac8!("12-34-56-78-9A-BC-DE-FF");

#[cfg(feature = "std")]
#[rstest]
#[case(MacAddrFormat::Canonical, "12-34-56-78-9A-BC")]
#[case(MacAddrFormat::ColonNotation, "12:34:56:78:9A:BC")]
#[case(MacAddrFormat::DotNotation, "1234.5678.9ABC")]
#[case(MacAddrFormat::Hexadecimal, "123456789ABC")]
#[case(MacAddrFormat::Hexadecimal0x, "0x123456789ABC")]
fn format_string_mac6(#[case] format: MacAddrFormat, #[case] expected: &str) {
    assert_eq!(TEST_MAC6.format_string(format), expected);
}

#[cfg(feature = "std")]
#[rstest]
#[case(MacAddrFormat::Canonical, "12-34-56-78-9A-BC-DE-FF")]
#[case(MacAddrFormat::ColonNotation, "12:34:56:78:9A:BC:DE:FF")]
#[case(MacAddrFormat::DotNotation, "1234.5678.9ABC.DEFF")]
#[case(MacAddrFormat::Hexadecimal, "123456789ABCDEFF")]
#[case(MacAddrFormat::Hexadecimal0x, "0x123456789ABCDEFF")]
fn format_string_mac8(#[case] format: MacAddrFormat, #[case] expected: &str) {
    assert_eq!(TEST_MAC8.format_string(format), expected);
}

#[rstest]
#[case(MacAddrFormat::Canonical, "12-34-56-78-9A-BC")]
#[case(MacAddrFormat::ColonNotation, "12:34:56:78:9A:BC")]
#[case(MacAddrFormat::DotNotation, "1234.5678.9ABC")]
#[case(MacAddrFormat::Hexadecimal, "123456789ABC")]
#[case(MacAddrFormat::Hexadecimal0x, "0x123456789ABC")]
fn format_arraystring_mac6(#[case] format: MacAddrFormat, #[case] expected: &str) {
    let mut buf = ArrayString::<MAC_MAX_SIZE>::new();
    TEST_MAC6.format_write(&mut buf, format).unwrap();
    assert_eq!(buf.as_str(), expected);
}

#[rstest]
#[case(MacAddrFormat::Canonical, "12-34-56-78-9A-BC-DE-FF")]
#[case(MacAddrFormat::ColonNotation, "12:34:56:78:9A:BC:DE:FF")]
#[case(MacAddrFormat::DotNotation, "1234.5678.9ABC.DEFF")]
#[case(MacAddrFormat::Hexadecimal, "123456789ABCDEFF")]
#[case(MacAddrFormat::Hexadecimal0x, "0x123456789ABCDEFF")]
fn format_arraystring_mac8(#[case] format: MacAddrFormat, #[case] expected: &str) {
    let mut buf = ArrayString::<MAC_MAX_SIZE>::new();
    TEST_MAC8.format_write(&mut buf, format).unwrap();
    assert_eq!(buf.as_str(), expected);
}

#[rstest]
#[case(MacAddrFormat::Canonical, MAC_CANONICAL_SIZE6, MAC_CANONICAL_SIZE8)]
#[case(
    MacAddrFormat::ColonNotation,
    MAC_COLON_NOTATION_SIZE6,
    MAC_COLON_NOTATION_SIZE8
)]
#[case(
    MacAddrFormat::DotNotation,
    MAC_DOT_NOTATION_SIZE6,
    MAC_DOT_NOTATION_SIZE8
)]
#[case(
    MacAddrFormat::Hexadecimal,
    MAC_HEXADECIMAL_SIZE6,
    MAC_HEXADECIMAL_SIZE8
)]
#[case(
    MacAddrFormat::Hexadecimal0x,
    MAC_HEXADECIMAL0X_SIZE6,
    MAC_HEXADECIMAL0X_SIZE8
)]
fn test_length(#[case] format: MacAddrFormat, #[case] expected6: usize, #[case] expected8: usize) {
    assert_eq!(TEST_MAC6.format_string(format).len(), expected6);
    assert_eq!(TEST_MAC8.format_string(format).len(), expected8);
}
