use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Grammar {
    pub name: String,
    pub rules: HashMap<String, Rule>,
    pub supertypes: Vec<String>,
}

impl Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("name: {}", self.name);
        let rules = self
            .rules
            .iter()
            .map(|(k, v)| format!("{k} -> {v}"))
            .collect::<Vec<_>>()
            .join("\n\n");
        write!(f, "{name}\n\n{rules}")
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Rule {
    //in json grammar.json line 80?
    #[serde(rename = "BLANK")]
    Blank,

    #[serde(rename = "STRING")]
    String {
        value: String,
    },

    #[serde(rename = "PATTERN")]
    Regex {
        value: String,
    },

    #[serde(rename = "SEQ")]
    Sequence {
        members: Vec<Box<Rule>>,
    },

    #[serde(rename = "CHOICE")]
    Alternative {
        members: Vec<Box<Rule>>,
    },

    #[serde(rename = "REPEAT")]
    Repeat {
        content: Box<Rule>,
    },

    #[serde(rename = "REPEAT1")]
    Repeat1 {
        content: Box<Rule>,
    },

    #[serde(rename = "TOKEN")]
    Token {
        content: Box<Rule>,
    },

    #[serde(rename = "IMMEDIATE_TOKEN")]
    ImmediateToken {
        content: Box<Rule>,
    },

    // can't find an example, not sure what name is
    Optional(Box<Rule>),

    #[serde(rename = "SYMBOL")]
    Symbol {
        name: String,
    },

    #[serde(rename = "PREC")]
    Precedence {
        value: isize,
        content: Box<Rule>,
    },

    #[serde(rename = "ALIAS")]
    Alias {
        content: Box<Rule>,
        named: bool,
        value: String,
    },

    #[serde(rename = "PREC_DYNAMIC")]
    DynPrec {
        value: isize,
        content: Box<Rule>,
    },

    #[serde(rename = "PREC_LEFT")]
    LeftPrec {
        value: isize,
        content: Box<Rule>,
    },

    #[serde(rename = "PREC_RIGHT")]
    RightPrec {
        value: isize,
        content: Box<Rule>,
    },

    #[serde(rename = "FIELD")]
    Field {
        name: String,
        content: Box<Rule>,
    },
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Rule::Blank => String::new(),
            Rule::String { value } => format!("{value:?}"),
            Rule::Regex { value } => format!("r{value:?}"),
            Rule::Sequence { members } => members
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            Rule::Alternative { members } => members
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<_>>()
                .join(" | "),
            Rule::Repeat { content } => format!("{content}*"),
            Rule::Repeat1 { content } => format!("{content}+"),
            Rule::Token { content } => format!("token({content})"),
            Rule::ImmediateToken { content } => format!("imm_token({content})"),
            Rule::Optional(x) => format!("{x}?"),
            Rule::Symbol { name } => name.clone(),
            Rule::Precedence { value, content } => format!("p({value}, {content})"),
            Rule::Alias {
                content,
                named,
                value,
            } => format!("{}alias({value}, {content})", if *named { "n" } else { "" }),
            Rule::DynPrec { value, content } => format!("dp({value}, {content})"),
            Rule::LeftPrec { value, content } => format!("lp({value}, {content})"),
            Rule::RightPrec { value, content } => format!("rp({value}, {content})"),
            Rule::Field { name, content } => format!("field({name}, {content})"),
        };
        write!(f, "{}", s)
    }
}
