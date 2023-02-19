/// A macro that generates getters and setters for different structs (common,
/// account_all etc.) inside the `standard_format` crate
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, FieldsUnnamed};

pub fn imp_enum_variant_deserialize_macro(ast: &syn::DeriveInput) -> TokenStream {
    let enum_name = &ast.ident;
    let variants = match &ast.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("expected an enum"),
    };

    let match_statements = variants.into_iter().map(|variant| {
        // let attrs: Vec<_> = variant.attrs.iter().filter_map(get_evd_attr).collect();
        let name = &variant.ident;
        let right_side = match &variant.fields {
            syn::Fields::Named(_) => panic!("Unsupported"),
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                if unnamed.len() != 1 {
                    panic!("Enum variant can only have one field")
                }
                quote!(Ok(Self::#name(Deserialize::deserialize(d)?)))
            }
            syn::Fields::Unit => quote!(Ok(Self::#name)),
        };

        quote! {
            stringify!(#name) => #right_side,
        }
    });

    let gen = quote! {
        impl VariantDeserializer for #enum_name {
            fn deserialize_variant<'de, D>(discim: &'static str, d: D) -> core::result::Result<Self, D::Error>
            where
                Self: Sized,
                D: serde::de::Deserializer<'de>
            {
                match discim {
                    #(#match_statements)*
                    _ => panic!("Incorrect usage")
                }
            }
        }
    };
    gen.into()
}
