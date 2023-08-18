extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(EnumLength)]
pub fn derive_enum_length(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let len = match ast.data {
        syn::Data::Enum(enum_item) => enum_item.variants.len(),
        _ => panic!("EnumLength only works on Enums"),
    };
    let expanded = quote! {
        impl EnumLength for #name {
            const LENGTH: usize = #len;
        }
    };
    expanded.into()
}
