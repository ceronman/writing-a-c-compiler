use crate::ast;
use crate::parser;
use crate::semantic;
use crate::tacky;

#[allow(dead_code)]
pub fn dump_ast(src: &str) -> String {
    let ast = parser::parse(src).unwrap();
    ast::pretty::dump(&ast).unwrap()
}

#[allow(dead_code)]
pub fn dump_tacky(src: &str) -> String {
    let ast = parser::parse(src).unwrap();
    let (ast, semantic_data) = semantic::validate(ast).unwrap();
    let tacky = tacky::emit(&ast, semantic_data);
    tacky::pretty::pp(&tacky).unwrap().trim().to_owned()
}

#[allow(dead_code)]
pub fn dedent(tree: &str) -> String {
    tree.trim()
        .lines()
        .map(|l| l.strip_prefix("        ").unwrap_or(l))
        .collect::<Vec<_>>()
        .join("\n")
}

#[allow(dead_code)]
pub fn remove_annotation(src: &str) -> String {
    src.lines()
        .filter(|l| !l.trim().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[allow(dead_code)]
pub fn annotate(src: &str, error: &crate::error::CompilerError) -> String {
    let mut result = String::new();
    let mut offset = 0;
    let mut annotated = false;
    for line in src.split_inclusive('\n') {
        result.push_str(line);
        if !annotated && offset + line.len() > error.span.0 {
            let start = error.span.0 - offset;
            let start = if start > 2 { start - 2 } else { 0 };
            let len = error.span.1 - error.span.0;
            let annotation = format!("{}//{} {}\n", " ".repeat(start), "^".repeat(len), error.msg);
            result.push_str(&annotation);
            annotated = true
        }
        offset += line.len();
    }
    if !annotated {
        result.push_str(&format!("\n// {}", error.msg));
    }
    result
}

// TODO: Implement pretty for asm
