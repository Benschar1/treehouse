use proc_macro2::Span;
use quote::quote;
use syn::{parse2, Ident, ItemEnum};

use convert_case::{Case, Casing};

pub fn make_type_enum(name: &str, variants: Vec<String>) -> ItemEnum {
    println!(
        "{name}: {}",
        variants
            .iter()
            .map(|v| format!("{v:?}"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let name_id = name.to_case(Case::Pascal).id();
    let variant_ids = variants.iter().map(|v| v.to_case(Case::Pascal).id());

    let tokens = quote!(
        pub enum #name_id {
            #(#variant_ids(#variant_ids)),*
        }
    );

    let panic_msg = format!("Failed to parse the following as an enum:\n{tokens}");

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
