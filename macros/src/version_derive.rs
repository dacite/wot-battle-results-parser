/// A macro that generates getters and setters for different structs (common,
/// account_all etc.) inside the `standard_format` crate
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DataStruct, Expr, ExprArray, ExprCall, Fields, Path, Type};

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
                let Type::Path(path) = &field.ty else {
                    panic!("Unexpected type")
                };
                let ident = path
                    .path
                    .segments
                    .first()
                    .map(|path_segment| &path_segment.ident)
                    .expect("Expected type that is wrapped in Option");
                if ident != "Option" {
                    panic!("Field that is using the version attribute must be wrapped in Option")
                }
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
        match version {
            VersionArg::Single(version) => statements.push(quote! { VersionInfo::Version(#version), }),
            VersionArg::Range((range_begin, range_end)) => {
                statements.push(quote! { VersionInfo::VersionRange((#range_begin, #range_end)), })
            }
        }
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

enum VersionArg {
    Single(ExprArray),
    Range((ExprArray, ExprArray)),
}

fn get_version_arg(attr: &Attribute) -> Option<VersionArg> {
    if attr.path.is_ident("version") {
        if let Ok(arg) = attr.parse_args::<ExprArray>() {
            Some(VersionArg::Single(arg))
        } else {
            let arg: ExprCall = attr.parse_args().unwrap();

            let mut arguments = arg.args.into_pairs();
            let range_begin = arguments.next().unwrap().into_value();
            let Expr::Array(range_begin) = range_begin else { panic!("Unexpected argument to Version range")};

            let range_end = arguments.next().unwrap().into_value();
            let Expr::Array(range_end) = range_end else { panic!("Unexpected argument to Version range")};

            Some(VersionArg::Range((range_begin, range_end)))
        }
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
