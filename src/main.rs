use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, ValueEnum};

use treehouse::grammar::types::Grammar;
use treehouse::node_types::codegen::gen_type;

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
    // let x = Node {
    //     bare_node: BareNode::named("FuncDef"),
    //     other: treehouse::node_types::types::NodeType::Internal {
    //         fields: HashMap::from_iter(vec![
    //             (
    //                 "name".into(),
    //                 Child {
    //                     multiple: false,
    //                     required: true,
    //                     node_types: vec![BareNode::named("Ident")],
    //                 },
    //             ),
    //             (
    //                 "params".into(),
    //                 Child {
    //                     multiple: true,
    //                     required: false,
    //                     node_types: vec![
    //                         BareNode::named("RegularParam"),
    //                         BareNode::named("SelfParam"),
    //                         BareNode::named("DefaultParam"),
    //                         BareNode::named("VarArgParam"),
    //                     ],
    //                 },
    //             ),
    //             (
    //                 "return_type".into(),
    //                 Child {
    //                     multiple: false,
    //                     required: true,
    //                     node_types: vec![BareNode::named("Type")],
    //                 },
    //             ),
    //         ]),
    //         children: Child {
    //             multiple: false,
    //             required: false,
    //             node_types: vec![BareNode::named("Attr")],
    //         },
    //     },
    // };

    // let y = gen_type(&x);

    // println!("\n\nFile:\n{:#?}\n\n{}", y, prettyplease::unparse(&y));

    let cli = Cli::parse();
    let file = File::open(cli.file)?;
    println!(
        "{}",
        match cli.file_type {
            FileType::Grammar => serde_json::from_reader::<_, Grammar>(file)?.to_string(),
            FileType::NodeTypes => {
                let treehouse::node_types::types::TopLevel(types) = serde_json::from_reader(file)?;
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
