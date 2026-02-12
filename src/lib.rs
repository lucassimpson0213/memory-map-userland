#![cfg_attr(not(test), no_std)]

// When running tests, Rust automatically enables std.
// When used in kernel, std disappears.

#[cfg(test)]
extern crate std;

pub mod frames;
pub mod raw;
pub mod region;
