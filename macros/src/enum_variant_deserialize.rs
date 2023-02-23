/// A macro that generates getters and setters for different structs (common,
/// account_all etc.) inside the `standard_format` crate
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DataEnum, FieldsUnnamed, Ident};

pub fn imp_enum_variant_deserialize_macro(ast: &syn::DeriveInput) -> TokenStream {
    let enum_name = &ast.ident;
    let variants = match &ast.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("expected an enum"),
    };

    let match_statements = variants.into_iter().map(|variant| {
        let attrs: Vec<_> = variant.attrs.iter().filter_map(get_args).collect();
        let name = &variant.ident;
        let right_side = match &variant.fields {
            syn::Fields::Named(_) => panic!("Unsupported"),
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                if unnamed.len() > 1 {
                    panic!("Enum variant can only have one field");
                }
                match attrs.get(0){
                    Some(s) if s == "delegate" => quote!(Ok(Self::#name(crate::packet_parser::prelude::from_slice(input, context.get_version())?))),
                    Some(s) if s == "manual" => quote!(unreachable!("You should've specified manual parser for {}", stringify!(#name))),
                    Some(_) => panic!("Unknown args"),
                    None => quote!(Ok(Self::#name(crate::packet_parser::prelude::from_slice_prim(input, context.get_version())?)))
                }
            }
            syn::Fields::Unit => quote!(Ok(Self::#name)),
        };

        quote! {
            stringify!(#name) => #right_side,
        }
    });

    let gen = quote! {
        impl VariantDeserializer for #enum_name {
            fn deserialize_variant(discrim: &'static str, input: &[u8], context: &crate::Context) -> core::result::Result<Self, crate::PacketError>
            where
                Self: Sized
            {
                let result = match discrim {
                    #(#match_statements)*
                    _ => panic!("{} is not found in match statement", discrim)
                };

                result.map_err(|err: PacketError| PacketError::EntityPropertyError {
                    entity_type: #enum_name::entity_type(),
                    property:    discrim.into(),
                    root_cause:  err.to_string(),
                })
            }
        }
    };
    gen.into()
}

fn get_args(attr: &Attribute) -> Option<String> {
    if attr.path.is_ident("variant_de") {
        let arg = attr.parse_args::<Ident>().unwrap();
        Some(arg.to_string())
    } else {
        None
    }
}
