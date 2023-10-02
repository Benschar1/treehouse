use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, ValueEnum};

use treehouse::grammar::types::Grammar;

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
            FileType::NodeTypes => todo!(), //serde_json::from_reader::<_, TopLevel>(file)?.to_string(),
        }
    );
    Ok(())
}
