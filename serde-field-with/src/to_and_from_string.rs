//! https://doc.rust-lang.org/rust-by-example/conversion/string.html

use alloc::{boxed::Box, string::ToString as _};
use core::{fmt, str};

use serde::{
    de::{self, Deserialize as _, Deserializer},
    ser::{Serialize as _, Serializer},
};

pub fn serialize<S, T>(this: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: fmt::Display,
{
    this.to_string().serialize(serializer)
}

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: str::FromStr,
    <T as str::FromStr>::Err: fmt::Display,
{
    let s = Box::<str>::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}
