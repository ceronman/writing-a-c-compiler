mod lexer;

use anyhow::{bail, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
struct Options {
    filename: PathBuf,

    #[arg(long)]
    lex: bool,

    #[arg(long)]
    parse: bool,

    #[arg(long)]
    codegen: bool,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let preprocessed = run_preprocessor(&options.filename)?;

    let source = fs::read_to_string(&preprocessed)?;
    let tokens = lexer::lex(&source)?;
    if options.lex {
        println!("{:#?}", tokens);
        return Ok(());
    }

    let asm_path = compile(&options.filename)?;
    assemble_and_link(&asm_path, &options.filename)?;
    Ok(())
}

fn run_preprocessor(filename: &Path) -> Result<tempfile::TempPath> {
    let output_path = tempfile::Builder::new()
        .prefix("preprocessed")
        .suffix(".i")
        .tempfile()?
        .into_temp_path();

    let output = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(filename)
        .arg("-o")
        .arg(&output_path)
        .output()?;

    if !output.status.success() {
        bail!("{}", String::from_utf8(output.stderr)?)
    }

    Ok(output_path)
}

fn compile(path: &Path) -> Result<tempfile::TempPath> {
    let output_path = tempfile::Builder::new()
        .prefix("assembly")
        .suffix(".s")
        .tempfile()?
        .into_temp_path();

    let output = Command::new("gcc")
        .arg("-S")
        .arg("-O")
        .arg("-fno-asynchronous-unwind-tables")
        .arg("-fcf-protection=none")
        .arg(path)
        .arg("-o")
        .arg(&output_path)
        .output()?;

    if !output.status.success() {
        bail!("{}", String::from_utf8(output.stderr)?)
    }

    Ok(output_path)
}

fn assemble_and_link(path: &Path, source: &Path) -> Result<()> {
    let output = Command::new("gcc")
        .arg(path)
        .arg("-o")
        .arg(source.file_stem().unwrap())
        .output()?;

    if !output.status.success() {
        bail!("{}", String::from_utf8(output.stderr)?)
    }

    Ok(())
}
