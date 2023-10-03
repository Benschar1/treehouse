use proc_macro2::Span;
use quote::quote;
use syn::{parse2, Ident, ItemEnum};

pub fn make_type_enum(name: &str, variants: Vec<String>) -> ItemEnum {
    let name_id = name.id();
    let variant_ids = variants.iter().map(IntoIdent::id);

    let tokens = quote!(
        pub enum #name_id {
            #(#variant_ids(#variant_ids)),*
        }
    );

    let panic_msg = format!("Failed to parse the following as an enum: {tokens:?}");

    parse2(tokens).expect(&panic_msg)
}

pub trait IntoIdent {
    fn id(&self) -> Ident;
}

impl IntoIdent for String {
    fn id(&self) -> Ident {
        Ident::new(&self, Span::call_site())
    }
}

impl IntoIdent for &str {
    fn id(&self) -> Ident {
        Ident::new(&self, Span::call_site())
    }
}
