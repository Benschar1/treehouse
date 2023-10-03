use syn::{parse_quote, token::Brace, Item, ItemEnum};

use super::types::{Child, Node};
use crate::utils::{make_type_enum, IntoIdent};

pub fn child_to_enum(name: &str, child: &Child) -> ItemEnum {
    make_type_enum(name, child.named_node_types())
}

pub fn gen_type(node: &Node) -> syn::ItemMod {
    let type_name = &node.bare_node.type_name;

    let items = match &node.other {
        super::types::NodeType::Internal { fields, children } => {
            let child_type_name_str = format!("{type_name}Children");
            let child_type_name = child_type_name_str.id();

            let field_types = fields
                .iter()
                .map(|(name, child)| child_to_enum(&format!("{type_name}{name}"), child));

            let field_type_names = field_types.clone().map(|t| t.ident);

            let mut defs = vec![Item::Struct(parse_quote!(
                pub struct Type {
                    pub children: #child_type_name,
                    #(pub #field_type_names: #field_type_names),*
                }
            ))];
            defs.push(Item::Enum(child_to_enum(&child_type_name_str, children)));
            defs.extend(field_types.map(Item::Enum));
            defs
        }
        super::types::NodeType::Supertype { subtypes } => {
            vec![Item::Enum(make_type_enum(
                "Type",
                subtypes.iter().map(|t| t.type_name.clone()).collect(),
            ))]
        }
    };
    syn::ItemMod {
        attrs: vec![],
        vis: parse_quote!(pub),
        unsafety: None,
        mod_token: parse_quote!(mod),
        ident: type_name.id(),
        content: Some((Brace::default(), items)),
        semi: None,
    }
}
