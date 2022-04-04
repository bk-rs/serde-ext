#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
pub mod to_and_from_string;
#[cfg(feature = "alloc")]
pub use to_and_from_string::{deserialize as from_str, serialize as to_string};
