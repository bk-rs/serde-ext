use serde_rename_rule::{ParseError as RenameRuleParseError, RenameRule};
use syn::{Error as SynError, Expr, Meta, MetaList};

use crate::{
    Symbol,
    rename::{Rename, RenameIndependent, syn::FromMetaError as RenameFromMetaError},
};

use super::{RenameAll, RenameAllIndependent};

/// [Ref](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/symbol.rs#L24)
pub const RENAME_ALL: Symbol = Symbol("rename_all");

/// [Ref](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/attr.rs#L335-L364)
impl<'a> core::convert::TryFrom<&'a Meta> for RenameAll {
    type Error = FromMetaError<'a>;

    fn try_from(meta: &'a Meta) -> Result<Self, Self::Error> {
        match Rename::try_from_meta(meta, RENAME_ALL) {
            Ok(rename) => match rename {
                Rename::Normal(name) => RenameRule::from_rename_all_str(name.as_str())
                    .map(Self::Normal)
                    .map_err(FromMetaError::RenameRuleParseError),
                Rename::Independent(RenameIndependent::Serialize(ser_name)) => {
                    RenameRule::from_rename_all_str(ser_name.as_str())
                        .map(|x| Self::Independent(RenameAllIndependent::Serialize(x)))
                        .map_err(FromMetaError::RenameRuleParseError)
                }
                Rename::Independent(RenameIndependent::Deserialize(de_name)) => {
                    RenameRule::from_rename_all_str(de_name.as_str())
                        .map(|x| Self::Independent(RenameAllIndependent::Deserialize(x)))
                        .map_err(FromMetaError::RenameRuleParseError)
                }
                Rename::Independent(RenameIndependent::Both {
                    serialize: ser_name,
                    deserialize: de_name,
                }) => Ok(Self::Independent(RenameAllIndependent::Both {
                    serialize: RenameRule::from_rename_all_str(ser_name.as_str())
                        .map_err(FromMetaError::RenameRuleParseError)?,
                    deserialize: RenameRule::from_rename_all_str(de_name.as_str())
                        .map_err(FromMetaError::RenameRuleParseError)?,
                })),
            },
            Err(err) => match err {
                RenameFromMetaError::MetaTypeOrPathMismatch(meta) => {
                    Err(FromMetaError::MetaTypeOrPathMismatch(meta))
                }
                RenameFromMetaError::MetaNameValueExprTypeMismatch(expr) => {
                    Err(FromMetaError::MetaNameValueExprTypeMismatch(expr))
                }
                RenameFromMetaError::MetaListTypeMismatch(meta_list, err) => {
                    Err(FromMetaError::MetaListTypeMismatch(meta_list, err))
                }
                RenameFromMetaError::AtLeastOneOfSerAndDe => {
                    Err(FromMetaError::AtLeastOneOfSerAndDe)
                }
            },
        }
    }
}

pub enum FromMetaError<'a> {
    MetaTypeOrPathMismatch(&'a Meta),
    MetaNameValueExprTypeMismatch(&'a Expr),
    MetaListTypeMismatch(&'a MetaList, SynError),
    AtLeastOneOfSerAndDe,
    RenameRuleParseError(RenameRuleParseError),
}
impl<'a> core::fmt::Debug for FromMetaError<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MetaTypeOrPathMismatch(_) => write!(f, "MetaTypeOrPathMismatch"),
            Self::MetaNameValueExprTypeMismatch(_) => write!(f, "MetaNameValueExprTypeMismatch"),
            Self::MetaListTypeMismatch(_, _) => write!(f, "MetaListTypeMismatch"),
            Self::AtLeastOneOfSerAndDe => write!(f, "AtLeastOneOfSerAndDe"),
            Self::RenameRuleParseError(_) => write!(f, "RenameRuleParseError"),
        }
    }
}
impl<'a> core::fmt::Display for FromMetaError<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}
#[cfg(feature = "std")]
impl<'a> std::error::Error for FromMetaError<'a> {}
