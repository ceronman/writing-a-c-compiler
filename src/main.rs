mod asm;
mod ast;
mod emitter;
mod error;
mod lexer;
mod parser;
mod pretty;
mod semantic;
mod symbol;
mod tacky;
mod tempfile;

#[cfg(feature = "test_gen")]
mod testgen;

use crate::pretty::{pp_tacky, pretty_print_ast};
use crate::tempfile::TempPath;
use anyhow::{bail, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<()> {
    let options = parse_args();

    let preprocessed = run_preprocessor(&options.filename)?;
    let source = fs::read_to_string(preprocessed.as_path())?;

    #[cfg(feature = "test_gen")]
    {
        if let Flag::Lex = options.flag {
            testgen::generate_lexer_tests(&options.filename, &source)?;
        }

        if let Flag::Parse = options.flag {
            lexer::tokenize(&source); // Skip lexing errors
            testgen::generate_parser_tests(&options.filename, &source)?;
        }

        if let Flag::Validate = options.flag {
            lexer::tokenize(&source); // skip lexing errors
            parser::parse(&source)?; // skip parsing errors
            testgen::generate_resolver_tests(&options.filename, &source)?;
        }

        if let Flag::Tacky = options.flag {
            lexer::tokenize(&source); // skip lexing errors
            let ast = parser::parse(&source)?; // skip parsing errors
            semantic::validate(ast)?; // skip resolution errors
            testgen::generate_tacky_tests(&options.filename, &source)?;
        }
    }

    if let Flag::Lex = options.flag {
        let tokens = lexer::tokenize(&source);
        println!("{tokens:#?}");
        return Ok(());
    }

    let ast = match parser::parse(&source) {
        Ok(ast) => ast,
        Err(error) => {
            let annotated = pretty::annotate(&source, &error);
            panic!("{annotated}")
        }
    };
    if let Flag::Parse = options.flag {
        print!("{}", pretty_print_ast(&ast)?);
        return Ok(());
    }

    let (validated_ast, semantic_data) = match semantic::validate(ast) {
        Ok(ast) => ast,
        Err(error) => {
            let annotated = pretty::annotate(&source, &error);
            panic!("{annotated}")
        }
    };

    if let Flag::Validate = options.flag {
        println!("{}", pretty_print_ast(&validated_ast)?);
        println!("{semantic_data:?}");
        return Ok(());
    }

    let tacky = tacky::emit(&validated_ast, semantic_data);
    if let Flag::Tacky = options.flag {
        println!("{}", pp_tacky(&tacky)?);
        return Ok(());
    }

    let asm = asm::generate(&tacky);
    if let Flag::Codegen = options.flag {
        println!("{asm:#?}");
        return Ok(());
    }

    let asm_path = emitter::emit_code(&options.filename, &asm)?;
    if let Flag::Emit = options.flag {
        println!("{}", fs::read_to_string(asm_path.as_path())?);
        return Ok(());
    }

    match options.flag {
        Flag::Assemble => assemble(asm_path.as_path()),
        Flag::AssembleAndLink => assemble_and_link(asm_path.as_path()),
        _ => unreachable!(),
    }
}

struct Options {
    filename: PathBuf,
    flag: Flag,
}

enum Flag {
    Assemble,
    AssembleAndLink,
    Lex,
    Parse,
    Validate,
    Tacky,
    Codegen,
    Emit,
}

fn parse_args() -> Options {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let args: Vec<_> = args.iter().map(|s| s.as_str()).collect();
    let (path, flag) = match args[..] {
        ["--lex", path] => (path, Flag::Lex),
        ["--parse", path] => (path, Flag::Parse),
        ["--validate", path] => (path, Flag::Validate),
        ["--tacky", path] => (path, Flag::Tacky),
        ["--codegen", path] => (path, Flag::Codegen),
        ["--emit", path] => (path, Flag::Emit),
        ["-c", path] => (path, Flag::Assemble),
        [path] => (path, Flag::AssembleAndLink),
        _ => {
            eprintln!("Error: incorrect number of arguments");
            eprintln!("{}", args.join(" "));
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

fn assemble(path: &Path) -> Result<()> {
    let output = Command::new("gcc")
        .arg("-c")
        .arg(path)
        .arg("-o")
        .arg(path.with_extension("o"))
        .output()?;

    if !output.status.success() {
        bail!("{}", String::from_utf8(output.stderr)?)
    }

    Ok(())
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
