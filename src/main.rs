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

use crate::pretty::{pretty_print_ast, pretty_print_tacky};
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

        if let Flag::Interpret = options.flag {
            lexer::tokenize(&source); // skip lexing errors
            let ast = parser::parse(&source)?; // skip parsing errors
            semantic::validate(ast)?; // skip resolution errors
            testgen::generate_interpreter_tests(&options.filename, &source)?;
            return Ok(());
        }
    }

    if let Flag::Lex = options.flag {
        let tokens = lexer::tokenize(&source);
        println!("{tokens:#?}");
        return Ok(());
    }

    let ast = parser::parse(&source)?;
    if let Flag::Parse = options.flag {
        print!("{}", pretty_print_ast(&ast)?);
        return Ok(());
    }

    let validated_ast = match semantic::validate(ast) {
        Ok(ast) => ast,
        Err(error) => {
            let annotated = pretty::annotate(&source, &error);
            panic!("{annotated}")
        }
    };

    if let Flag::Validate = options.flag {
        print!("{}", pretty_print_ast(&validated_ast)?);
        return Ok(());
    }

    let tacky = tacky::emit(&validated_ast);
    if let Flag::Tacky = options.flag {
        println!("{}", pretty_print_tacky(tacky)?);
        return Ok(());
    }

    if let Flag::Interpret = options.flag {
        let result = tacky::interpreter::run(&tacky);
        std::process::exit(result as i32);
    }

    let asm = asm::generate(&tacky);
    if let Flag::Codegen = options.flag {
        println!("{asm:#?}");
        return Ok(());
    }

    let asm_path = emitter::emit_code(&options.filename, &asm)?;
    if let Flag::Asm = options.flag {
        println!("{}", fs::read_to_string(asm_path.as_path())?);
        return Ok(());
    }

    assemble_and_link(asm_path.as_path())?;
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
    Validate,
    Tacky,
    Interpret,
    Codegen,
    Asm,
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
        ["--asm", path] => (path, Flag::Asm),
        ["--interpret", "--tacky", path] => (path, Flag::Interpret),
        [path] => (path, Flag::None),
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
