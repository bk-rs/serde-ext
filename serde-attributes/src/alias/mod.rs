//! [Official doc](https://serde.rs/variant-attrs.html#alias)

#[cfg(feature = "with-darling")]
pub mod darling;
#[cfg(feature = "with-syn")]
pub mod syn;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Alias(pub String);
