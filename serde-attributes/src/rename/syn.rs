use syn::{Error as SynError, Expr, ExprLit, Lit, LitStr, Meta, MetaList};

use crate::symbol::{DESERIALIZE, SERIALIZE, Symbol};

use super::{Rename, RenameIndependent};

/// [Ref](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/symbol.rs#L23)
pub const RENAME: Symbol = Symbol("rename");

impl Rename {
    /// [Ref](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/attr.rs#L319-L333)
    pub fn try_from_meta<'a>(meta: &'a Meta, attr_name: Symbol) -> Result<Self, FromMetaError<'a>> {
        match meta {
            Meta::NameValue(meta_name_value) if meta_name_value.path == attr_name => {
                match &meta_name_value.value {
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit), ..
                    }) => Ok(Self::Normal(lit.value())),
                    expr => Err(FromMetaError::MetaNameValueExprTypeMismatch(expr)),
                }
            }
            Meta::List(meta_list) if meta_list.path == attr_name => {
                let mut ser_name = None;
                let mut de_name = None;

                meta_list.parse_nested_meta(|parse_nested_meta| {
                    if parse_nested_meta.path == SERIALIZE {
                        let value = parse_nested_meta.value()?;
                        let lit: LitStr = value.parse()?;
                        ser_name = Some(lit.value());
                    } else if parse_nested_meta.path == DESERIALIZE {
                        let value = parse_nested_meta.value()?;
                        let lit: LitStr = value.parse()?;
                        de_name = Some(lit.value());
                    } else {
                        return Err(parse_nested_meta.error(format_args!(
                            "malformed {0} attribute, expected `{0}(serialize = ..., deserialize = ...)`",
                            attr_name,
                        )));
                    }
                    Ok(())
                }).map_err(|err| FromMetaError::MetaListTypeMismatch(meta_list, err))?;

                match (ser_name, de_name) {
                    (None, None) => Err(FromMetaError::AtLeastOneOfSerAndDe),
                    (None, Some(de_name)) => {
                        Ok(Self::Independent(RenameIndependent::Deserialize(de_name)))
                    }
                    (Some(ser_name), None) => {
                        Ok(Self::Independent(RenameIndependent::Serialize(ser_name)))
                    }
                    (Some(ser_name), Some(de_name)) => {
                        Ok(Self::Independent(RenameIndependent::Both {
                            serialize: ser_name,
                            deserialize: de_name,
                        }))
                    }
                }
            }
            meta => Err(FromMetaError::MetaTypeOrPathMismatch(meta)),
        }
    }
}

/// [Ref](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/attr.rs#L319-L333)
impl<'a> core::convert::TryFrom<&'a Meta> for Rename {
    type Error = FromMetaError<'a>;

    fn try_from(meta: &'a Meta) -> Result<Self, Self::Error> {
        Self::try_from_meta(meta, RENAME)
    }
}

pub enum FromMetaError<'a> {
    MetaTypeOrPathMismatch(&'a Meta),
    MetaNameValueExprTypeMismatch(&'a Expr),
    MetaListTypeMismatch(&'a MetaList, SynError),
    AtLeastOneOfSerAndDe,
}
impl<'a> core::fmt::Debug for FromMetaError<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MetaTypeOrPathMismatch(_) => write!(f, "MetaTypeOrPathMismatch"),
            Self::MetaNameValueExprTypeMismatch(_) => write!(f, "MetaNameValueExprTypeMismatch"),
            Self::MetaListTypeMismatch(_, _) => write!(f, "MetaListTypeMismatch"),
            Self::AtLeastOneOfSerAndDe => write!(f, "AtLeastOneOfSerAndDe"),
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
