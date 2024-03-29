#![no_std]

extern crate alloc;

use alloc::{boxed::Box, string::String};

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Foo {
    A,
    #[serde(rename = "B")]
    B,
    #[serde(other)]
    Other(String),
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum Bar {
    A,
    #[serde(rename = "B")]
    B,
    #[serde(other)]
    Other(Box<str>),
}

fn main() {}

#[test]
fn simple() {
    use alloc::string::ToString as _;

    assert_eq!(serde_json::to_string(&Foo::A).unwrap(), r#""a""#);
    assert_eq!(serde_json::from_str::<Foo>(r#""a""#).unwrap(), Foo::A);

    assert_eq!(
        serde_json::from_str::<Foo>(r#""c""#).unwrap(),
        Foo::Other("c".into())
    );

    assert_eq!(Foo::A.to_string(), "a");
    assert_eq!("a".parse::<Foo>().unwrap(), Foo::A);
}
