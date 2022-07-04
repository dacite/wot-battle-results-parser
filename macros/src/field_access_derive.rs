/// A macro that generates getters and setters for different structs (common,
/// account_all etc.) inside the `standard_format` crate
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DataStruct, Fields, Lit, Meta, MetaNameValue};

pub fn imp_field_access_macro(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let mut match_conditions = Vec::new();
    let mut get_statements = Vec::new();
    let mut set_statements = Vec::new();
    let mut type_names = Vec::new();

    fields.iter().for_each(|field| {
        let name = field.ident.clone().unwrap();
        let type_name = field.ty.clone();
        let name_lower_case = to_lower_case(&field.ident.clone().unwrap());
        let attr_list = field.clone().attrs.into_iter().next();

        match_conditions.push(quote! {
           stringify!(#name) | #name_lower_case
        });

        get_statements.push(quote! {
            Some(serde_json::to_value(self.#name.clone()).unwrap())
        });

        type_names.push(quote! {&#type_name});

        if let Some(attr) = attr_list {
            let attr_val = get_value(&attr);
            set_statements.push(quote! {
                {
                    self.#name = self.#attr_val(val);
                    return Ok(())
                }

            });
        } else {
            set_statements.push(quote! {
                {
                    let result = serde_pickle::from_value::<WotValue>(val)?;
                    if let Ok(value) = serde_json::to_value(result) {
                        self.#name = serde_json::from_value(value).unwrap();
                        return Ok(())
                    } else {
                        return anyhow::Result::Err(anyhow::anyhow!("Value conversion error on {}", index));
                    }
                }
            });
        }
    });

    let gen = quote! {
        impl FieldAccess for #struct_name {
            fn get(&self, index: &str) -> Option<serde_json::Value> {
                match index {
                    #(
                      #match_conditions => #get_statements,
                    )*
                    _ => None
                }
            }

            fn set(&mut self, index: &str, val: serde_pickle::Value) -> anyhow::Result<()> {
                match index {
                    #(
                      #match_conditions => #set_statements,
                    )*
                    _ => return anyhow::Result::Err(anyhow::anyhow!("Struct {} does not have the member: /{}/", stringify!(#struct_name), index))
                }
            }
        }
    };
    gen.into()
}

/// to lower case from snake case
fn to_lower_case(x: &syn::Ident) -> String {
    format!("{}", x).replace('_', "")
}

/// Get the value given to the attribute of the form:
/// `#[customer_parser = "some value"]`
fn get_value(attr: &Attribute) -> syn::Ident {
    if !attr.path.is_ident("custom_parser") {
        panic!("expected attribute path: custom_parser")
    }

    match attr.parse_meta().unwrap() {
        Meta::NameValue(MetaNameValue {
            lit: Lit::Str(lit_str),
            ..
        }) => syn::Ident::new(&lit_str.value(), lit_str.span()),
        _ => {
            panic!("expected #[custom_parser = \"functionName\"]");
        }
    }
}
