use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt as _};
use alloc::vec::Vec;

use super::{
    input::Input,
    output::{SerdeEnum, SerdeEnumCategory},
};

pub struct InputWrapper(pub Input);

impl ToTokens for InputWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let input = &self.0;

        let de_enum = SerdeEnum::new(input, SerdeEnumCategory::De);
        let de_enum_ident = de_enum.ident();

        let token = quote! {
            #de_enum
        };
        tokens.append_all(token);

        //
        let de_untagged_enum_ident = format_ident!("{}Untagged", de_enum_ident);
        let de_untagged_enum_other_variant = if let Some(default_variant) = &input.default_variant {
            if let Some(r#type) = &default_variant.r#type {
                quote! {
                    __Other(#r#type),
                }
            } else {
                quote! {
                    __Other(String),
                }
            }
        } else {
            quote!()
        };
        let token = quote! {
            #[derive(serde::Deserialize)]
            #[serde(untagged)]
            enum #de_untagged_enum_ident {
                __Enum(#de_enum_ident),
                #de_untagged_enum_other_variant
            }
        };
        tokens.append_all(token);

        //
        let impl_ident = &input.ident;

        //
        let impl_variants = &input
            .variants
            .iter()
            .map(|variant| {
                let ident = &variant.ident;
                quote! {
                    #de_enum_ident::#ident => #impl_ident::#ident,
                }
            })
            .collect::<Vec<_>>();
        let impl_default_variant = if let Some(default_variant) = &input.default_variant {
            let ident = &default_variant.ident;
            if default_variant.r#type.is_some() {
                quote! {
                    #de_untagged_enum_ident::__Other(v) => #impl_ident::#ident(v)
                }
            } else {
                quote! {
                    #de_untagged_enum_ident::__Other(_) => #impl_ident::#ident
                }
            }
        } else {
            quote!()
        };

        //
        let token = quote! {
            impl<'de> serde::Deserialize<'de> for #impl_ident {
                fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
                where D: serde::Deserializer<'de>
                {
                    let value = match <#de_untagged_enum_ident as serde::Deserialize>::deserialize(deserializer)? {
                        #de_untagged_enum_ident::__Enum(e) => match e {
                            #(#impl_variants)*
                        },
                        #impl_default_variant
                    };

                    Ok(value)
                }
            }
        };
        tokens.append_all(token);

        //
        let token = quote! {
            // https://docs.serde.rs/serde/de/trait.IntoDeserializer.html
            impl ::core::str::FromStr for #impl_ident {
                type Err = serde::de::value::Error;

                fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                    use serde::{Deserialize as _, de::IntoDeserializer as _};

                    Self::deserialize(s.into_deserializer())
                }
            }
        };
        tokens.append_all(token);

        //
        let token = quote! {
            impl ::core::convert::TryFrom<String> for #impl_ident {
                type Error = serde::de::value::Error;

                fn try_from(value: String) -> ::core::result::Result<Self, Self::Error> {
                    value.parse()
                }
            }

            impl ::core::convert::TryFrom<&str> for #impl_ident {
                type Error = serde::de::value::Error;

                fn try_from(value: &str) -> ::core::result::Result<Self, Self::Error> {
                    value.parse()
                }
            }
        };
        tokens.append_all(token);
    }
}
