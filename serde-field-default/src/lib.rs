#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
mod std;
#[cfg(feature = "std")]
pub use self::std::{default_ip_addr, default_ipv4_addr, default_ipv6_addr};

#[cfg(feature = "alloc")]
mod alloc_;

#[cfg(feature = "chrono")]
pub mod chrono;

#[cfg(feature = "chrono-tz")]
pub mod chrono_tz;
