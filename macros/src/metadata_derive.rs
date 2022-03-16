extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

pub fn imp_to_packet_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl ToPacket for #name {
            fn get_all_data(&self) -> &[u8] {
                self.inner.get_ref()
            }
        }
    };
    gen.into()
}