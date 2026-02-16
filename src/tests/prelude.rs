#![cfg(all(test, feature = "std"))]

use std::sync::Once;

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        let _ = color_eyre::install();
    });
}

pub mod prelude {
    pub use crate::testing::init;
    pub use hex::{decode as hex_decode, encode as hex_encode};
    pub use insta::{assert_debug_snapshot, assert_json_snapshot, assert_snapshot};
    pub use pretty_assertions::{assert_eq, assert_ne};
    pub use proptest::prelude::*;
    pub use rstest::rstest;
    pub use similar_asserts::assert_eq as assert_str_eq;
}

pub fn decode_hex_relaxed(s: &str) -> Vec<u8> {
    let filtered: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    hex::decode(filtered).expect("invalid hex in test vector")
}

pub fn hexdump(bytes: &[u8]) -> String {
    const BYTES_PER_LINE: usize = 16;
    let mut out = String::new();
    for (i, chunk) in bytes.chunks(BYTES_PER_LINE).enumerate() {
        let offset = i * BYTES_PER_LINE;
        out.push_str(&format!("{offset:08x}: "));
        for b in chunk {
            out.push_str(&format!("{b:02x} "));
        }
        out.push('\n');
    }
    out
}
