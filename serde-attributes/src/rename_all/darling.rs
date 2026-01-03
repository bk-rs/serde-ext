use core::convert::TryFrom as _;

use darling_core::{Error as DarlingError, FromMeta};
use syn::Meta;

use super::{RenameAll, syn::FromMetaError};

impl FromMeta for RenameAll {
    fn from_meta(meta: &Meta) -> Result<Self, DarlingError> {
        Self::try_from(meta).map_err(|err| match err {
            FromMetaError::MetaTypeOrPathMismatch(meta) => match meta {
                Meta::Path(_) => DarlingError::unexpected_type("Meta::Path"),
                Meta::List(meta_list) => DarlingError::unknown_field_path(&meta_list.path),
                Meta::NameValue(meta_name_value) => {
                    DarlingError::unknown_field_path(&meta_name_value.path)
                }
            },
            FromMetaError::MetaNameValueExprTypeMismatch(expr) => {
                DarlingError::unexpected_expr_type(expr)
            }
            FromMetaError::MetaListTypeMismatch(_, _) => {
                DarlingError::unexpected_type("Meta::List")
            }
            FromMetaError::AtLeastOneOfSerAndDe => {
                DarlingError::custom("must be at least one the serialize and deserialize")
            }
            FromMetaError::RenameRuleParseError(err) => {
                DarlingError::custom(err.msg_for_rename_all())
            }
        })
    }
}
