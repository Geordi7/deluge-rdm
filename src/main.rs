use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use lalrpop_util::lalrpop_mod;

mod ast;
mod interpreter;
mod lsp;

lalrpop_mod!(pub parser);

#[derive(Parser)]
#[command(name = "deluge")]
struct Cli {
    /// Enable language server mode
    #[arg(long)]
    lsp: bool,

    /// Check file for errors but do not execute
    #[arg(long)]
    check: bool,

    /// Input file to run or check
    file: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.lsp {
        lsp::start().await?;
        return Ok(());
    }

    let file = cli
        .file
        .expect("an input file is required unless --lsp is used");
    let source = fs::read_to_string(file)?;

    let program = parser::FileParser::new()
        .parse(&source)
        .map_err(|e| anyhow::anyhow!(format!("{e:?}")))?;

    if cli.check {
        println!("Syntax OK");
    } else {
        interpreter::interpret(program);
    }

    Ok(())
}
