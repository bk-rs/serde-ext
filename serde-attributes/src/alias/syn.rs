use std::{convert::TryFrom, error, fmt};

use syn::{Lit, Meta};

use super::Alias;

/// [Ref](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/symbol.rs#L7)
pub const ALIAS: &str = "alias";

/// [Ref](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/attr.rs#L901-L906)
impl<'a> TryFrom<&'a Meta> for Alias {
    type Error = FromMetaError<'a>;

    fn try_from(meta: &'a Meta) -> Result<Self, Self::Error> {
        match meta {
            Meta::NameValue(ref meta_name_value) if meta_name_value.path.is_ident(ALIAS) => {
                match &meta_name_value.lit {
                    Lit::Str(ref s) => Ok(Self(s.value())),
                    lit => Err(FromMetaError::LitTypeMismatch(lit)),
                }
            }
            meta => Err(FromMetaError::MetaTypeOrPathMismatch(meta)),
        }
    }
}

pub enum FromMetaError<'a> {
    MetaTypeOrPathMismatch(&'a Meta),
    LitTypeMismatch(&'a Lit),
}
impl<'a> fmt::Debug for FromMetaError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MetaTypeOrPathMismatch(_) => write!(f, "MetaTypeOrPathMismatch"),
            Self::LitTypeMismatch(_) => write!(f, "LitTypeMismatch"),
        }
    }
}
impl<'a> fmt::Display for FromMetaError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl<'a> error::Error for FromMetaError<'a> {}
