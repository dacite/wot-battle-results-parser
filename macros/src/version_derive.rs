/// A macro that generates getters and setters for different structs (common,
/// account_all etc.) inside the `standard_format` crate
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DataStruct, ExprArray, Fields, Path};

pub fn imp_version_macro(ast: &syn::DeriveInput) -> TokenStream {
    let has_lifetime = ast.generics.lt_token.is_some();

    let struct_name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let mut statements = Vec::new();

    for field in fields {
        let attr_list = field.clone().attrs.into_iter().collect::<Vec<_>>();

        let mut args = Vec::new();
        let mut skip_field = false;
        for attr in attr_list {
            if let Some(arg) = get_version_arg(&attr) {
                args.push(arg);
            }
            if is_serde_skip(&attr) {
                skip_field = true;
            }
        }

        if skip_field {
            continue;
        }

        if args.is_empty() {
            statements.push(quote! {
                VersionInfo::All,
            });
            continue;
        }

        let version = args.into_iter().next().unwrap();
        statements.push(quote! { VersionInfo::Version(#version), });
    }

    let version = quote! {
        fn version() -> VersionInfo {
            VersionInfo::Struct(&[
                #(
                    #statements
                  )*
            ])
        }

        fn name() -> &'static str {
            stringify!(#struct_name)
        }
    };

    let gen = if has_lifetime {
        quote! {
            impl Version for #struct_name<'_> {
                #version
            }
        }
    } else {
        quote! {
            impl Version for #struct_name {
                #version
            }
        }
    };

    gen.into()
}

fn get_version_arg(attr: &Attribute) -> Option<ExprArray> {
    if attr.path.is_ident("version") {
        let arg: ExprArray = attr.parse_args().unwrap();
        Some(arg)
    } else {
        None
    }
}

fn is_serde_skip(attr: &Attribute) -> bool {
    if attr.path.is_ident("serde") {
        if let Ok(arg) = attr.parse_args::<Path>() {
            if let Some(ident) = arg.get_ident() {
                let iden_string = ident.to_string();

                (iden_string.contains("skip_deserializing") || iden_string.contains("skip"))
                    && !iden_string.contains("skip_serializing")
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}
