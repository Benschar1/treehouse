use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TopLevel(pub Vec<Node>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    #[serde(flatten)]
    pub bare_node: BareNode,
    #[serde(flatten)]
    pub other: NodeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BareNode {
    #[serde(rename = "type")]
    pub type_name: String,
    pub named: bool,
}

impl BareNode {
    pub fn named(type_name: &str) -> Self {
        Self {
            type_name: type_name.into(),
            named: true,
        }
    }

    pub fn anon(type_name: &str) -> Self {
        Self {
            type_name: type_name.into(),
            named: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum NodeType {
    // Since both of Internal's fields have defaults, serde was treating
    // Supertype nodes as Internals and initializing `fields` and `children`
    // with their default values. Defining Supertype before Internal so
    // Supertype takes precedence is a temporary fix.
    Supertype {
        subtypes: Vec<BareNode>,
    },
    Internal {
        #[serde(default)]
        fields: HashMap<String, Child>,
        #[serde(default)]
        children: Child,
    },
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Child {
    pub multiple: bool,
    pub required: bool,
    #[serde(rename = "types")]
    pub node_types: Vec<BareNode>,
}

impl Child {
    pub fn named_node_types(&self) -> Vec<String> {
        self.node_types
            .iter()
            .map(|n| n.type_name.clone())
            .collect()
    }
}

pub fn named_bare_nodes<I>(nodes: I) -> Vec<String>
where
    I: IntoIterator,
    BareNode: From<I::Item>,
{
    nodes
        .into_iter()
        .filter_map(|node| {
            let node = BareNode::from(node);
            if node.named {
                Some(node.type_name.clone())
            } else {
                None
            }
        })
        .collect()
}
