use proc_macro::TokenStream;

mod event_debug_derive;
// mod field_access_derive;
// mod metadata_derive;
// mod sql_insert_derive;
mod version_derive;

// #[proc_macro_derive(FieldAccess, attributes(custom_parser))]
// pub fn field_access_macro_derive(input: TokenStream) -> TokenStream {
//     let ast = syn::parse(input).unwrap();
//     field_access_derive::imp_field_access_macro(&ast)
// }

// #[proc_macro_derive(PacketMetadata)]
// pub fn to_packet_macro_derive(input: TokenStream) -> TokenStream {
//     let ast = syn::parse(input).unwrap();
//     metadata_derive::imp_to_packet_macro(&ast)
// }

#[proc_macro_derive(EventPrinter, attributes(event_debug))]
pub fn to_event_printer_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    event_debug_derive::imp_event_debug_macro(&ast)
}

#[proc_macro_derive(Version, attributes(version))]
pub fn to_version_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    version_derive::imp_version_macro(&ast)
}

// #[proc_macro_derive(SqlInsert, attributes(version))]
// pub fn to_sql_insert_derive(input: TokenStream) -> TokenStream {
//     let ast = syn::parse(input).unwrap();
//     sql_insert_derive::imp_sql_insert_macro(&ast)
// }
