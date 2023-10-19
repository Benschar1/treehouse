use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, ValueEnum};

use treehouse::grammar::types::Grammar;
use treehouse::node_types::codegen::gen_type;
use treehouse::node_types::types::{Node, NodeType};

#[derive(Parser)]
struct Cli {
    file_type: FileType,
    file: PathBuf,
}

#[derive(Clone, ValueEnum)]
enum FileType {
    #[value(alias = "g")]
    Grammar,

    #[value(alias = "t")]
    NodeTypes,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let file = File::open(cli.file)?;
    println!(
        "{}",
        match cli.file_type {
            FileType::Grammar => serde_json::from_reader::<_, Grammar>(file)?.to_string(),
            FileType::NodeTypes => {
                let treehouse::node_types::types::TopLevel(types) = serde_json::from_reader(file)?;

                let mut supertypes = vec![];
                let mut internals = vec![];

                for t in types.as_slice() {
                    // println!("type: {t:?}");
                    match &t.other {
                        NodeType::Internal { fields, children } => {
                            // println!("internal: {}", t.bare_node.type_name);
                            internals.push((&t.bare_node, fields, children))
                        }
                        NodeType::Supertype { subtypes } => {
                            // println!("supertype: {}", t.bare_node.type_name);
                            supertypes.push((&t.bare_node, subtypes))
                        }
                    }
                }

                // println!("Supertypes:");
                // for (supertype, subtypes) in supertypes {
                //     println!(
                //         "  {}{}\x1b[0m",
                //         if supertype.named { "\x1b[32;1m" } else { "" },
                //         supertype.type_name
                //     );
                //     for subtype in subtypes {
                //         println!(
                //             "    | {}{}\x1b[0m",
                //             if subtype.named { "\x1b[32;1m" } else { "" },
                //             subtype.type_name
                //         );
                //     }
                // }

                let rust_file = syn::File {
                    shebang: None,
                    attrs: vec![],
                    items: types.iter().map(|n| syn::Item::Mod(gen_type(n))).collect(),
                };
                prettyplease::unparse(&rust_file)
            }
        }
    );
    Ok(())
}
