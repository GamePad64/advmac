#![cfg(all(feature = "std", feature = "serde"))]

use advmac::{mac6, mac8, MacAddr6, MacAddr8};
use serde::{Deserialize, Serialize};

#[test]
fn test_serde() {
    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct S {
        pub mac6: MacAddr6,
        pub mac8: MacAddr8,
    }
    let s = S {
        mac6: mac6!("12:34:56:78:9A:BC"),
        mac8: mac8!("12:34:56:78:9A:BC:DE:FF"),
    };
    let serialized = serde_json::to_string(&s).unwrap();
    assert_eq!(
        serialized,
        r#"{"mac6":"12-34-56-78-9A-BC","mac8":"12-34-56-78-9A-BC-DE-FF"}"#
    );
    let parsed: S = serde_json::from_str(&serialized).unwrap();
    assert_eq!(parsed, s);
}
