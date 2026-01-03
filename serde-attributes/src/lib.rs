//! Serde Attributes. [Extract from](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/attr.rs#L290)
//!
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(feature = "attr-alias")]
pub mod alias;
#[cfg(feature = "attr-alias")]
pub use alias::Alias;

#[cfg(feature = "attr-rename")]
pub mod rename;
#[cfg(feature = "attr-rename")]
pub use rename::{Rename, RenameIndependent};

#[cfg(feature = "attr-rename-all")]
pub mod rename_all;
#[cfg(feature = "attr-rename-all")]
pub use rename_all::{RenameAll, RenameAllIndependent};

pub mod symbol {
    //
    #[derive(Copy, Clone)]
    pub struct Symbol(pub &'static str);

    /// [Ref](https://github.com/serde-rs/serde/blob/v1.0.228/serde_derive/src/internals/symbol.rs#L30)
    #[cfg(any(feature = "attr-rename", feature = "attr-rename-all"))]
    pub const SERIALIZE: Symbol = Symbol("serialize");
    #[cfg(any(feature = "attr-rename", feature = "attr-rename-all"))]
    /// [Ref](https://github.com/serde-rs/serde/blob/v1.0.228/serde_derive/src/internals/symbol.rs#L14)
    pub const DESERIALIZE: Symbol = Symbol("deserialize");

    #[cfg(feature = "with-syn")]
    impl PartialEq<Symbol> for syn::Ident {
        fn eq(&self, word: &Symbol) -> bool {
            self == word.0
        }
    }

    #[cfg(feature = "with-syn")]
    impl PartialEq<Symbol> for &syn::Ident {
        fn eq(&self, word: &Symbol) -> bool {
            *self == word.0
        }
    }

    #[cfg(feature = "with-syn")]
    impl PartialEq<Symbol> for syn::Path {
        fn eq(&self, word: &Symbol) -> bool {
            self.is_ident(word.0)
        }
    }

    #[cfg(feature = "with-syn")]
    impl PartialEq<Symbol> for &syn::Path {
        fn eq(&self, word: &Symbol) -> bool {
            self.is_ident(word.0)
        }
    }

    impl core::fmt::Display for Symbol {
        fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            formatter.write_str(self.0)
        }
    }
}

pub use symbol::Symbol;
