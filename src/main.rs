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

mod alignment;
mod optimization;
#[cfg(feature = "test_gen")]
mod testgen;

use crate::asm::ir::Program;
use crate::optimization::OptimizationFlags;
use crate::tempfile::TempPath;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::process::Command;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
        print!("{}", ast::pretty::dump(&ast)?);
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
        println!("{}", ast::pretty::dump(&validated_ast)?);
        println!("{semantic_data:#?}");
        return Ok(());
    }

    let tacky = tacky::emit(&validated_ast, semantic_data);
    if let Flag::Tacky = options.flag {
        println!("{}", tacky::pretty::pp(&tacky)?);
        println!("{:#?}", tacky.semantics);
        return Ok(());
    }

    let tacky = optimization::optimize(tacky, &options.optimization);
    if let Flag::OptimizedTacky = options.flag {
        println!("{}", tacky::pretty::pp(&tacky)?);
        println!("{:#?}", tacky.semantics);
        for program in tacky.top_level {
            if let tacky::TopLevel::Function(f) = program {
                println!("CFG Function: {}", f.name);
                let cfg = optimization::cfg::tacky_to_cfg(&f.body);
                println!("{cfg:?}");
            }
        }
        return Ok(());
    }

    let asm = asm::generate(&tacky);
    if let Flag::Codegen = options.flag {
        println!("{}", asm::pretty::pp(&asm)?);
        return Ok(());
    }

    match options.flag {
        Flag::Emit => write_assembly_to_stdout(&asm),
        Flag::GenerateAssemblyOnly => write_assembly_only(&options.filename, &asm),
        Flag::Assemble => assemble(&options.filename, &asm),
        Flag::AssembleAndLink => assemble_and_link(&options.filename, &asm),
        _ => unreachable!(),
    }
}

struct Options {
    filename: PathBuf,
    flag: Flag,
    optimization: OptimizationFlags,
}

enum Flag {
    GenerateAssemblyOnly,
    Assemble,
    AssembleAndLink,
    Lex,
    Parse,
    Validate,
    Tacky,
    OptimizedTacky,
    Codegen,
    Emit,
}

fn parse_args() -> Options {
    let mut args: Vec<_> = std::env::args().skip(1).collect();
    let mut optimization = OptimizationFlags::default();
    if let Some(i) = args.iter().position(|arg| arg == "--fold-constants") {
        optimization.fold_constants = true;
        args.remove(i);
    }
    if let Some(i) = args.iter().position(|arg| arg == "--propagate-copies") {
        optimization.propagate_copies = true;
        args.remove(i);
    }

    if let Some(i) = args
        .iter()
        .position(|arg| arg == "--eliminate-unreachable-code")
    {
        optimization.eliminate_unreachable_code = true;
        args.remove(i);
    }

    if let Some(i) = args.iter().position(|arg| arg == "--eliminate-dead-stores") {
        optimization.eliminate_dead_stores = true;
        args.remove(i);
    }

    if let Some(i) = args.iter().position(|arg| arg == "--optimize") {
        optimization.optimize = true;
        args.remove(i);
    }

    let args: Vec<_> = args.iter().map(|s| s.as_str()).collect();

    let (path, flag) = match args[..] {
        ["--lex", path] => (path, Flag::Lex),
        ["--parse", path] => (path, Flag::Parse),
        ["--validate", path] => (path, Flag::Validate),
        ["--tacky", path] => (path, Flag::Tacky),
        ["--optimized-tacky", path] => (path, Flag::OptimizedTacky),
        ["--codegen", path] => (path, Flag::Codegen),
        ["--emit", path] => (path, Flag::Emit),
        ["-s" | "-S", path] => (path, Flag::GenerateAssemblyOnly),
        ["-c", path] => (path, Flag::Assemble),
        [path] => (path, Flag::AssembleAndLink),
        _ => {
            eprintln!("Error: incorrect number of arguments");
            eprintln!("{}", args.join(" "));
            eprintln!(
                "Usage: compiler [ --lex | --parse | --validate | --codegen | --emit | -s | -c ] <FILENAME>"
            );
            std::process::exit(1);
        }
    };
    Options {
        filename: PathBuf::from(path),
        flag,
        optimization,
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
        return Err(String::from_utf8(output.stderr)?.into());
    }

    Ok(output_path)
}

fn write_assembly_to_stdout(program: &Program) -> Result<()> {
    let output = &mut std::io::stdout();
    emitter::emit_program(output, program)?;
    Ok(())
}

fn write_assembly_only(path: &Path, program: &Program) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.with_extension("s"))?;
    let output = &mut BufWriter::new(file);
    emitter::emit_program(output, program)?;
    Ok(())
}

fn assemble(path: &Path, program: &Program) -> Result<()> {
    let assembler_code_path = TempPath::new(path.with_extension("s"));
    {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(assembler_code_path.as_path())?;
        let output = &mut BufWriter::new(file);
        emitter::emit_program(output, program)?;
    }
    let output = Command::new("gcc")
        .arg("-c")
        .arg(assembler_code_path.as_path())
        .arg("-o")
        .arg(path.with_extension("o"))
        .output()?;

    if !output.status.success() {
        return Err(String::from_utf8(output.stderr)?.into());
    }

    Ok(())
}

fn assemble_and_link(path: &Path, program: &Program) -> Result<()> {
    let assembler_code_path = TempPath::new(path.with_extension("s"));
    {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(assembler_code_path.as_path())?;
        let output = &mut BufWriter::new(file);
        emitter::emit_program(output, program)?;
    }

    let output = Command::new("gcc")
        .arg(assembler_code_path.as_path())
        .arg("-o")
        .arg(path.with_extension(""))
        .output()?;

    if !output.status.success() {
        return Err(String::from_utf8(output.stderr)?.into());
    }

    Ok(())
}
