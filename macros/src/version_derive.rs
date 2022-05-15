/// A macro that generates getters and setters for different structs (common,
/// account_all etc.) inside the `standard_format` crate
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DataStruct, ExprArray, Fields, Ident, LitStr};

pub fn imp_version_macro(ast: &syn::DeriveInput) -> TokenStream {
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
        for attr in attr_list {
            if let Some(arg) = get_version_arg(&attr) {
                args.push(arg);
            }
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

    let gen = quote! {
        impl Version for #struct_name {
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
        }
    };

    gen.into()
}

fn get_version_arg(attr: &Attribute) -> Option<ExprArray> {
    if attr.path.is_ident("version") {
        let arg: ExprArray = attr.parse_args().unwrap();
        return Some(arg);
    } else {
        return None;
    }
}
