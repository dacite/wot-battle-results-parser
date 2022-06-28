extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

pub fn imp_to_packet_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl <'pkt> PacketMetadata<'pkt> for #name<'pkt> {
            fn get_inner(&'pkt self) -> &'pkt [u8] {
                self.inner
            }
        }
    };
    gen.into()
}
