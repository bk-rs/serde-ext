use syn::{Expr, ExprLit, Lit, Meta};

use crate::Symbol;

use super::Alias;

/// [Ref](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/symbol.rs#L7)
pub const ALIAS: Symbol = Symbol("alias");

/// [Ref](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/attr.rs#L901-L906)
impl<'a> core::convert::TryFrom<&'a Meta> for Alias {
    type Error = FromMetaError<'a>;

    fn try_from(meta: &'a Meta) -> Result<Self, Self::Error> {
        match meta {
            Meta::NameValue(meta_name_value) if meta_name_value.path == ALIAS => {
                match &meta_name_value.value {
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }) => Ok(Self(lit.value())),
                    expr => Err(FromMetaError::MetaNameValueExprTypeMismatch(expr)),
                }
            }
            meta => Err(FromMetaError::MetaTypeOrPathMismatch(meta)),
        }
    }
}

pub enum FromMetaError<'a> {
    MetaTypeOrPathMismatch(&'a Meta),
    MetaNameValueExprTypeMismatch(&'a Expr),
}
impl<'a> core::fmt::Debug for FromMetaError<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MetaTypeOrPathMismatch(_) => write!(f, "MetaTypeOrPathMismatch"),
            Self::MetaNameValueExprTypeMismatch(_) => write!(f, "MetaNameValueExprTypeMismatch"),
        }
    }
}
impl<'a> core::fmt::Display for FromMetaError<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl<'a> core::error::Error for FromMetaError<'a> {}
