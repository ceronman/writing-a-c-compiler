use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, panic};

use crate::{lexer, parser, pretty, resolver};
use anyhow::Result;

pub fn generate_lexer_tests(path: &Path, source: &str) -> Result<()> {
    let output = PathBuf::from(file!());
    let output = output
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("src/lexer/test.rs");
    if fs::read_to_string(&output)?.is_empty() {
        let mut file = OpenOptions::new().create(true).write(true).open(&output)?;
        writeln!(file, "use crate::lexer::tokenize;")?;
        writeln!(file, "use crate::lexer::TokenKind::*;")?;
    }
    let name = test_name(path);
    let mut file = OpenOptions::new().create(true).append(true).open(&output)?;
    let indented = source
        .lines()
        .map(|l| format!("        {l}"))
        .collect::<Vec<_>>()
        .join("\n");
    let result = panic::catch_unwind(|| lexer::tokenize(source));
    writeln!(file)?;
    match result {
        Ok(tokens) => {
            writeln!(file, "#[test]")?;
            writeln!(file, "fn test_{name}() {{")?;
            writeln!(file, "    let src = r#\"")?;
            writeln!(file, "{indented}")?;
            writeln!(file, "    \"#;")?;
            writeln!(file, "    let expected = vec!{tokens:?};")?;
            writeln!(file, "    assert_eq!(tokenize(src), expected);")?;
            writeln!(file, "}}")?;
        }
        Err(_) => {
            writeln!(file, "#[test]")?;
            writeln!(file, "#[should_panic]")?;
            writeln!(file, "fn test_{name}() {{")?;
            writeln!(file, "    tokenize(r#\"")?;
            writeln!(file, "{indented}")?;
            writeln!(file, "    \"#);")?;
            writeln!(file, "}}")?;
        }
    }

    Ok(())
}

pub fn generate_parser_tests(path: &Path, source: &str) -> Result<()> {
    let output = PathBuf::from(file!());
    let output = output
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("src/parser/test.rs");
    if fs::read_to_string(&output)?.is_empty() {
        let mut file = OpenOptions::new().create(true).write(true).open(&output)?;
        writeln!(
            file,
            r#"
use crate::parser::parse;
use crate::pretty::{{annotate, dedent, dump_ast, remove_annotation}};

fn assert_error(expected_annotated: &str) {{
    let clean_source = remove_annotation(expected_annotated);
    let Err(error) = parse(&clean_source) else {{
        panic!("No error produced!")
    }};
    let actual_annotated = annotate(&clean_source, &error);
    assert_eq!(actual_annotated, expected_annotated);
}}
"#
        )?;
    }
    let name = test_name(path);
    let mut file = OpenOptions::new().create(true).append(true).open(&output)?;
    let indented = indent(source);
    writeln!(file)?;
    let result = parser::parse(&indented);
    match result {
        Ok(ast) => {
            let tree = pretty::pretty_print_ast(&ast)?;
            let tree = indent(&tree);
            writeln!(file, "#[test]")?;
            writeln!(file, "fn test_{name}() {{")?;
            writeln!(file, "    let src = r#\"{indented}\"#;")?;
            writeln!(file, "    let expected = r#\"{tree}\"#;")?;
            writeln!(file, "    assert_eq!(dump_ast(src), dedent(expected));")?;
            writeln!(file, "}}")?;
        }
        Err(error) => {
            let annotated = pretty::annotate(&indented, &error);
            writeln!(file, "#[test]")?;
            writeln!(file, "fn test_{name}() {{")?;
            writeln!(file, "    assert_error(r#\"{annotated}\"#);")?;
            writeln!(file, "}}")?;
        }
    }

    Ok(())
}

pub fn generate_resolver_tests(path: &Path, source: &str) -> Result<()> {
    let output = PathBuf::from(file!());
    let output = output
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("src/resolver/test.rs");
    if fs::read_to_string(&output)?.is_empty() {
        let mut file = OpenOptions::new().create(true).write(true).open(&output)?;
        writeln!(
            file,
            r#"
use crate::parser::parse;
use crate::resolver::resolve;
use crate::pretty::{{annotate, remove_annotation}};

fn assert_error(expected_annotated: &str) {{
    let clean_source = remove_annotation(expected_annotated);
    let ast = parse(&clean_source).expect("Parse error");
    let Err(error) = resolve(ast) else {{
        panic!("No error produced!")
    }};
    let actual_annotated = annotate(&clean_source, &error);
    assert_eq!(actual_annotated, expected_annotated);
}}
"#
        )?;
    }
    let name = test_name(path);
    let mut file = OpenOptions::new().create(true).append(true).open(&output)?;
    let indented = indent(source);
    let result = resolver::resolve(parser::parse(&indented)?);
    match result {
        Ok(_) => {}
        Err(error) => {
            let annotated = pretty::annotate(&indented, &error);
            writeln!(file)?;
            writeln!(file, "#[test]")?;
            writeln!(file, "fn test_{name}() {{")?;
            writeln!(file, "    assert_error(r#\"{annotated}\"#);")?;
            writeln!(file, "}}")?;
        }
    }

    Ok(())
}

fn indent(s: &str) -> String {
    let indented = s
        .lines()
        .map(|l| format!("        {l}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("\n{indented}\n    ")
}

fn test_name(path: &Path) -> String {
    let components: Vec<_> = path
        .components()
        .map(|c| c.as_os_str().to_str().unwrap().to_owned())
        .collect();
    let index = components
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if c.starts_with("chapter") {
                Some(i)
            } else {
                None
            }
        })
        .next()
        .unwrap_or(3);
    let components = &components[index..];
    components.join("_").strip_suffix(".c").unwrap().to_owned()
}
