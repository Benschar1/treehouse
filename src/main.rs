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
