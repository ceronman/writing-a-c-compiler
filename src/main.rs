mod asm;
mod ast;
mod codegen;
mod emitter;
mod lexer;
mod parser;

use anyhow::{bail, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<()> {
    let options = parse_args();
    let preprocessed = run_preprocessor(&options.filename)?;

    let source = fs::read_to_string(&preprocessed)?;
    if let Flag::Lex = options.flag {
        lexer::verify(&source);
        return Ok(());
    }

    let ast = parser::parse(&source)?;
    if let Flag::Parse = options.flag {
        println!("{ast:#?}");
        return Ok(());
    }

    let assembly = codegen::generate_assembly(ast)?;
    if let Flag::Codegen = options.flag {
        println!("{assembly:#?}");
        return Ok(());
    }

    let asm_path = emitter::emit_code(&assembly)?;

    assemble_and_link(&asm_path, &options.filename)?;
    Ok(())
}

struct Options {
    filename: PathBuf,
    flag: Flag,
}
enum Flag {
    None,
    Lex,
    Parse,
    Codegen,
}

fn parse_args() -> Options {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let args: Vec<_> = args.iter().map(|s| s.as_str()).collect();
    let (path, flag) = match args[..] {
        ["--lex", path] => (path, Flag::Lex),
        ["--parse", path] => (path, Flag::Parse),
        ["--codegen", path] => (path, Flag::Codegen),
        [path] => (path, Flag::None),
        _ => {
            eprintln!("Error: incorrect number of arguments");
            eprintln!("Usage: compiler [ --lex | --parse | --codegen ] <FILENAME>");
            std::process::exit(1);
        }
    };
    Options {
        filename: PathBuf::from(path),
        flag,
    }
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

fn assemble_and_link(path: &Path, source: &Path) -> Result<()> {
    let output = Command::new("gcc")
        .arg(path)
        .arg("-o")
        .arg(source.with_extension(""))
        .output()?;

    if !output.status.success() {
        bail!("{}", String::from_utf8(output.stderr)?)
    }

    Ok(())
}
