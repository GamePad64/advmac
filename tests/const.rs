use advmac::{mac6, mac8, MacAddr6, MacAddr8};

const TEST_MAC_CONST6: MacAddr6 = mac6!("11-33-55-77-99-AA");
const TEST_MAC_CONST8: MacAddr8 = mac8!("11-33-55-77-99-AA-CC-EE");

#[test]
fn test_const_vars() {
    assert_eq!(
        TEST_MAC_CONST6.to_array(),
        [0x11, 0x33, 0x55, 0x77, 0x99, 0xAA]
    );
    assert_eq!(
        TEST_MAC_CONST8.to_array(),
        [0x11, 0x33, 0x55, 0x77, 0x99, 0xAA, 0xCC, 0xEE]
    );
}
