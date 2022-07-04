extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DataStruct, Field, Fields, Lit, MetaNameValue};

pub fn imp_sql_insert_macro(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let fields_string = fields
        .iter()
        .enumerate()
        .fold(String::from(""), |acc, (i, field)| {
            if i != fields.len() - 1 {
                format!("{} {},", acc, convert_to_camel_case(field))
            } else {
                format!("{} {}", acc, convert_to_camel_case(field))
            }
        });

    let statements = fields.iter().map(|field| {
        let name = field.ident.clone().unwrap();
        quote! { let b = b.push_bind(item.#name);}
    });

    let gen = quote! {
        impl #struct_name {
            pub fn push_bindings<'args>(mut b: sqlx::query_builder::Separated<'_, 'args, sqlx::Sqlite, &'static str>, item: #struct_name) {
                #(
                    #statements
                )*
            }

            pub fn insert_query_start() -> String {
                format!("INSERT INTO {} ({}) ", stringify!(#struct_name), #fields_string)
            }
        }
    };

    gen.into()
}

fn convert_to_camel_case(field: &Field) -> String {
    use convert_case::{Case, Casing};
    let attr_list = field.clone().attrs.into_iter().collect::<Vec<_>>();

    let mut converted = field.ident.clone().unwrap().to_string().to_case(Case::Camel);
    for attr in attr_list {
        if let Some(renamed) = serde_renamed(&attr) {
            converted = renamed;
            break;
        }
    }

    converted
}

fn serde_renamed(attr: &Attribute) -> Option<String> {
    if attr.path.is_ident("serde") {
        if let Ok(arg) = attr.parse_args::<MetaNameValue>() {
            let ident = arg.path.get_ident()?;
            if ident == "rename" {
                if let Lit::Str(name) = arg.lit {
                    return Some(name.token().to_string());
                    // return name.parse().ok().map(|s: Ident| s.to_string());
                }
            }
            None
        } else {
            None
        }
    } else {
        None
    }
}
