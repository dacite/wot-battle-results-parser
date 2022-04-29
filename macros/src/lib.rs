use proc_macro::TokenStream;

mod field_access_derive;
mod metadata_derive;

#[proc_macro_derive(FieldAccess, attributes(custom_parser))]
pub fn field_access_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    field_access_derive::imp_field_access_macro(&ast)
}


#[proc_macro_derive(ToPacket)]
pub fn to_packet_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    metadata_derive::imp_to_packet_macro(&ast)
}
