/// A macro that generates getters and setters for different structs (common,
/// account_all etc.) inside the `standard_format` crate
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DataStruct, Fields, Ident};

pub fn imp_event_debug_macro(ast: &syn::DeriveInput) -> TokenStream {
    let has_lifetime = if ast.generics.lt_token.is_some() {
        true
    } else {
        false
    };

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
        let name = field.ident.clone().unwrap();
        let attr_list = field.clone().attrs.into_iter().collect::<Vec<_>>();

        let mut args = Vec::new();
        for attr in attr_list {
            if let Some(arg) = get_event_debug_arg(&attr) {
                args.push(arg);
            }
        }

        if args.is_empty() {
            statements.push(quote! {
                format!("{}: {:?} ", stringify!(#name), &self.#name)
            });
            continue;
        }

        for arg in args {
            match arg.as_str() {
                "ignore" => {}
                "custom_debug" => {
                    statements.push(quote! {
                        format!("{}: {} ", stringify!(#name), &self.#name.to_debug_string(context))
                    });
                }
                "as_player" => {
                    statements.push(quote! {
                        format!("{} {} ", stringify!(#name), &context.entity_id_to_player(self.#name).unwrap_or(self.#name.to_string()))
                    });
                }
                _ => panic!("Unknown value given to event_printer_macro"),
            }
        }
    }
    let to_debug_string = quote! {
        fn to_debug_string(&self, context: &crate::BattleContext) -> String
            where
                Self: std::fmt::Debug,
            {
                let mut output = String::new();
                #(
                    output.push_str(&#statements);
                )*

                output
            }
    };

    let gen;

    if has_lifetime {
        gen = quote! {
            impl EventPrinter for #struct_name<'_> {
                #to_debug_string
            }
        };
    } else {
        gen = quote! {
            impl EventPrinter for #struct_name {
                #to_debug_string
            }
        };
    }
    gen.into()
}

fn get_event_debug_arg(attr: &Attribute) -> Option<String> {
    if attr.path.is_ident("event_debug") {
        let arg = attr.parse_args::<Ident>().unwrap();
        return Some(arg.to_string());
    } else {
        return None;
    }
}
