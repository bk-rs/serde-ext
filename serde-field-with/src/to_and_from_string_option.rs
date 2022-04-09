use alloc::boxed::Box;
use core::{fmt, str};

use serde::{
    de::{self, Deserialize as _, Deserializer},
    ser::Serializer,
};

use crate::to_and_from_string;

pub fn serialize<S, T>(this: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: fmt::Display,
{
    match *this {
        Some(ref t) => to_and_from_string::serialize(t, serializer),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: str::FromStr,
    <T as str::FromStr>::Err: fmt::Display,
{
    match Option::<Box<str>>::deserialize(deserializer)? {
        Some(s) => T::from_str(&s).map(Some).map_err(de::Error::custom),
        None => Ok(None),
    }
}
