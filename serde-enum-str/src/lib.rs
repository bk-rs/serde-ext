//! Serialize and deserialize enum string.
//!
//! ```Cargo.toml
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde-enum-str = "0.2"
//! ```
//!
//! ```
//! use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
//!
//! #[derive(Deserialize_enum_str, Serialize_enum_str, PartialEq, Debug)]
//! #[serde(rename_all = "snake_case")]
//! enum Foo {
//!     A,
//!     #[serde(rename = "B")]
//!     B,
//!     #[serde(other)]
//!     Other(String),
//! }
//!
//! fn main() -> serde_json::Result<()> {
//!     assert_eq!(serde_json::to_string(&Foo::A).unwrap(), r#""a""#);
//!     assert_eq!(serde_json::from_str::<Foo>(r#""a""#).unwrap(), Foo::A);
//!
//!     assert_eq!(
//!         serde_json::from_str::<Foo>(r#""c""#).unwrap(),
//!         Foo::Other("c".to_owned())
//!     );
//!
//!     assert_eq!(Foo::A.to_string(), "a");
//!     assert_eq!("a".parse::<Foo>().unwrap(), Foo::A);
//!
//!     Ok(())
//! }
//! ```
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::single_match_else)]

extern crate proc_macro;
extern crate alloc;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod input;
mod output;
mod output_de;
mod output_ser;

#[proc_macro_derive(Serialize_enum_str, attributes(serde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as self::input::Input);
    let input = self::output_ser::InputWrapper(input);

    TokenStream::from(quote!(#input))
}

#[proc_macro_derive(Deserialize_enum_str, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as self::input::Input);
    let input = self::output_de::InputWrapper(input);

    TokenStream::from(quote!(#input))
}
