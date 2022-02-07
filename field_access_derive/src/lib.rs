extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{Attribute, Data, DataStruct, Fields, Lit, Meta, MetaNameValue};



#[proc_macro_derive(FieldAccess, attributes(custom_parser))]
pub fn field_access_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    imp_field_access_macro(&ast)
}

fn imp_field_access_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let mut match_conditions = Vec::new();
    let mut get_statements = Vec::new();
    let mut set_statements = Vec::new();

    fields.iter().for_each(|field| {
        let name = field.ident.clone().unwrap();
        let name_lower_case = to_lower_case(&field.ident.clone().unwrap());
        let attr_list = field.clone().attrs.into_iter().next();
        match_conditions.push(quote! {
           stringify!(#name) | #name_lower_case
        });

        get_statements.push(quote! {
            &self.#name
        });

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
                     let result = serde_pickle::from_value(val);
                     if let Ok(value) = result {
                     self.#name = value;
                     return Ok(())
                 } else {
                     return Err(format!("Value conversion error on {}", index));
                 }


                 }
            });
        }
    });

    let gen = quote! {
        impl FieldAccess for #name {
            fn get(&self, index: &str) -> &WotValue {
                match index {
                    #(
                      #match_conditions => #get_statements,
                    )*
                    _ => &WotValue::OutOfBounds
                }
            }

            fn set(&mut self, index: &str, val: serde_pickle::Value) -> std::result::Result<(), String> {
                match index {
                    #(
                      #match_conditions => #set_statements,
                    )*
                    _ => return Err(format!("Struct does not have the member: /{}/", index))
                }
            }
        }
    };
    gen.into()
}

fn to_lower_case(x: &syn::Ident) -> String{
    format!("{}", x).replace("_", "")
}


fn get_value(attr: &Attribute) -> syn::Ident {
    if !attr.path.is_ident("custom_parser") {
        panic!("expected attribute path: custom_parser")
    }

    match attr.parse_meta().unwrap() {
        Meta::NameValue(MetaNameValue { lit: Lit::Str(lit_str), .. }) => {
            return syn::Ident::new(&lit_str.value(), lit_str.span());
        }
        _ => {
            panic!("expected #[custom = \"functionName\"]");
        }
    }
}