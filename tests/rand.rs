#![cfg(feature = "rand")]

use advmac::{MacAddr6, MacAddr8};

#[test]
fn test_random() {
    assert_ne!(MacAddr6::random(), MacAddr6::random());
    assert_ne!(MacAddr8::random(), MacAddr8::random());
}
