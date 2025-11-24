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

use crate::asm::ir::Program;
use crate::emitter::TargetOs;
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

    if let Flag::Lex = options.flag {
        let tokens = lexer::tokenize(&source);
        println!("{tokens:#?}");
        return Ok(());
    }

    let ast = match parser::parse(&source) {
        Ok(ast) => ast,
        Err(error) => {
            let annotated = pretty::annotate(&source, &error);
            eprintln!("Parsing error:\n");
            eprintln!("{annotated}");
            std::process::exit(1);
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
            eprintln!("Semantic error:\n");
            eprintln!("{annotated}");
            std::process::exit(1);
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
        Flag::AssembleAndLink => assemble_and_link(&options.filename, &asm, &options),
        _ => unreachable!(),
    }
}

struct Options {
    filename: PathBuf,
    flag: Flag,
    optimization: OptimizationFlags,
    linker_arg: Option<String>,
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
    let program_name = std::env::args().next().unwrap_or_else(|| "compiler".to_string());

    fn print_help(program: &str) {
        eprintln!("Usage: {program} [FLAGS] <FILENAME>\n");
        eprintln!("Pipeline inspection flags (pick one):");
        eprintln!("  --lex                Print lexer tokens");
        eprintln!("  --parse              Print parsed AST");
        eprintln!("  --validate           Print validated AST and semantic info");
        eprintln!("  --tacky              Print TACKY IR");
        eprintln!("  --optimized-tacky    Print optimized TACKY IR");
        eprintln!("  --codegen            Print generated assembly (pretty)");
        eprintln!("  --emit               Emit assembly to stdout");
        eprintln!("  -s | -S              Generate assembly .s file only");
        eprintln!("  -c                   Generate object file .o only");
        eprintln!("  (default)            Assemble and link to executable\n");
        eprintln!("Optimization flags (can be combined):");
        eprintln!("  --optimize           Turn on all optimizations");
        eprintln!("  --fold-constants");
        eprintln!("  --propagate-copies");
        eprintln!("  --eliminate-unreachable-code");
        eprintln!("  --eliminate-dead-stores");
        eprintln!("  --trace              Enable debug optimizer passes\n");
        eprintln!("Linking:");
        eprintln!("  -l<NAME>             Pass a single -l flag to linker");
        eprintln!("General:");
        eprintln!("  -h, --help           Show this help and exit");
    }

    fn consume_flag(args: &mut Vec<String>, name: &str) -> bool {
        if let Some(i) = args.iter().position(|arg| arg == name) {
            args.remove(i);
            true
        } else {
            false
        }
    }

    let mut args: Vec<_> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help(&program_name);
        std::process::exit(0);
    }

    let mut optimization = OptimizationFlags::default();
    let linker_arg = args
        .iter()
        .position(|arg| arg.starts_with("-l"))
        .map(|arg| args.remove(arg).clone());

    if consume_flag(&mut args, "--fold-constants") {
        optimization.fold_constants = true;
    }
    if consume_flag(&mut args, "--propagate-copies") {
        optimization.propagate_copies = true;
    }
    if consume_flag(&mut args, "--eliminate-unreachable-code") {
        optimization.eliminate_unreachable_code = true;
    }
    if consume_flag(&mut args, "--eliminate-dead-stores") {
        optimization.eliminate_dead_stores = true;
    }
    if consume_flag(&mut args, "--optimize") {
        optimization.optimize = true;
    }
    if consume_flag(&mut args, "--trace") {
        optimization.trace = true;
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
            eprintln!("Error: incorrect arguments");
            print_help(&program_name);
            std::process::exit(1);
        }
    };
    Options {
        filename: PathBuf::from(path),
        flag,
        optimization,
        linker_arg,
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
    emitter::emit_program(output, program, current_target())?;
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
    emitter::emit_program(output, program, current_target())?;
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
        emitter::emit_program(output, program, current_target())?;
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

fn assemble_and_link(path: &Path, program: &Program, options: &Options) -> Result<()> {
    let assembler_code_path = TempPath::new(path.with_extension("s"));
    {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(assembler_code_path.as_path())?;
        let output = &mut BufWriter::new(file);
        emitter::emit_program(output, program, current_target())?;
    }

    let mut gcc = Command::new("gcc");
    gcc.arg(assembler_code_path.as_path())
        .arg("-o")
        .arg(path.with_extension(""));

    if let Some(linked) = &options.linker_arg {
        gcc.arg(linked);
    }

    let output = gcc.output()?;

    if !output.status.success() {
        return Err(String::from_utf8(output.stderr)?.into());
    }

    Ok(())
}

fn current_target() -> TargetOs {
    if cfg!(target_os = "macos") {
        TargetOs::MacOs
    } else {
        TargetOs::Linux
    }
}
