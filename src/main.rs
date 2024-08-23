mod asm;
mod ast;
mod codegen;
mod emitter;
mod lexer;
mod parser;
mod tempfile;

use crate::tempfile::TempPath;
use anyhow::{bail, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<()> {
    let options = parse_args();

    let preprocessed = run_preprocessor(&options.filename)?;

    let source = fs::read_to_string(preprocessed.as_path())?;
    if let Flag::Lex = options.flag {
        lexer::verify(&source);
        return Ok(());
    }

    let ast = parser::parse(&source)?;
    if let Flag::Parse = options.flag {
        println!("{ast:#?}");
        return Ok(());
    }

    let ir = codegen::generate_ir(ast)?;
    if let Flag::Codegen = options.flag {
        println!("{ir:#?}");
        return Ok(());
    }

    let asm = emitter::emit_code(&options.filename, &ir)?;
    if let Flag::Asm = options.flag {
        println!("{}", fs::read_to_string(asm.as_path())?);
        return Ok(());
    }

    assemble_and_link(asm.as_path())?;
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
    Asm,
}

fn parse_args() -> Options {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let args: Vec<_> = args.iter().map(|s| s.as_str()).collect();
    let (path, flag) = match args[..] {
        ["--lex", path] => (path, Flag::Lex),
        ["--parse", path] => (path, Flag::Parse),
        ["--codegen", path] => (path, Flag::Codegen),
        ["--asm", path] => (path, Flag::Asm),
        [path] => (path, Flag::None),
        _ => {
            eprintln!("Error: incorrect number of arguments");
            eprintln!("Usage: compiler [ --lex | --parse | --codegen | --asm ] <FILENAME>");
            std::process::exit(1);
        }
    };
    Options {
        filename: PathBuf::from(path),
        flag,
    }
}

fn run_preprocessor(filename: &Path) -> Result<TempPath> {
    let output_path = TempPath::new(filename.with_extension("i"));
    let output = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(filename)
        .arg("-o")
        .arg(output_path.as_path())
        .output()?;

    if !output.status.success() {
        bail!("{}", String::from_utf8(output.stderr)?)
    }

    Ok(output_path)
}

fn assemble_and_link(path: &Path) -> Result<()> {
    let output = Command::new("gcc")
        .arg(path)
        .arg("-o")
        .arg(path.with_extension(""))
        .output()?;

    if !output.status.success() {
        bail!("{}", String::from_utf8(output.stderr)?)
    }

    Ok(())
}
