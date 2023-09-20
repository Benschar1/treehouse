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

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum NodeType {
    Internal {
        #[serde(default)]
        fields: HashMap<String, Child>,
        #[serde(default)]
        children: Child,
    },
    Supertype {
        subtypes: Vec<BareNode>,
    },
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Child {
    pub multiple: bool,
    pub required: bool,
    #[serde(rename = "types")]
    pub node_types: Vec<Box<BareNode>>,
}
