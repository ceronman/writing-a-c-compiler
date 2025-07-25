use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, panic};

use crate::lexer::TokenKind;
use crate::{ast, lexer, parser, pretty, semantic, tacky};
use anyhow::Result;

pub fn generate_lexer_tests(path: &Path, source: &str) -> Result<()> {
    let chapter = test_chapter(path);
    let this_file = PathBuf::from(file!());
    let output = this_file
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("src/lexer/test/test_{chapter}.rs"));

    let parent_mod = this_file
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("src/lexer/test.rs"));

    if !fs::read_to_string(&parent_mod).unwrap_or("".into()).contains(&chapter) {
        let mut file = OpenOptions::new().create(true).append(true).open(&parent_mod)?;
        writeln!(file, "mod test_{chapter};")?;
    }

    if fs::read_to_string(&output).unwrap_or("".into()).is_empty() {
        let mut file = OpenOptions::new().create(true).truncate(true).write(true).open(&output)?;
        writeln!(file, "use crate::lexer::{{IntKind, tokenize}};")?;
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
    match result {
        Ok(tokens) => {
            if name.contains("invalid") {
                return Ok(());
            }
            writeln!(file)?;
            writeln!(file, "#[test]")?;
            writeln!(file, "fn test_{name}() {{")?;
            writeln!(file, "    let src = r#\"")?;
            writeln!(file, "{indented}")?;
            writeln!(file, "    \"#;")?;
            let expected: Vec<String> = tokens
                .iter()
                .map(|t| match t {
                    TokenKind::IntConstant(c) => format!("IntConstant(IntKind::{:?})", c),
                    _ => format!("{:?}", t),
                })
                .collect();
            writeln!(file, "    let expected = vec![{}];", expected.join(","))?;
            writeln!(file, "    assert_eq!(tokenize(src), expected);")?;
            writeln!(file, "}}")?;
        }
        Err(_) => {
            writeln!(file)?;
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
    let chapter = test_chapter(path);
    let this_file = PathBuf::from(file!());
    let output = this_file
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("src/parser/test/test_{chapter}.rs"));

    let parent_mod = this_file
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("src/parser/test.rs"));

    if !fs::exists(&parent_mod)? {
        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(&parent_mod)?;
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

fn assert_parse(src: &str, expected: &str) {{
    assert_eq!(dump_ast(src), dedent(expected));
}}
"#
        )?;
    }

    if !fs::read_to_string(&parent_mod)?.contains(&chapter) {
        let mut file = OpenOptions::new().create(true).append(true).open(&parent_mod)?;
        writeln!(file, "mod test_{chapter};")?;
    }
    if fs::read_to_string(&output).unwrap_or("".into()).is_empty() {
        let mut file = OpenOptions::new().create(true).truncate(true).write(true).open(&output)?;
        writeln!(file, "use super::{{assert_error, assert_parse}};")?;
    }
    let name = test_name(path);
    let mut file = OpenOptions::new().create(true).append(true).open(&output)?;
    let indented = indent(source);
    let result = parser::parse(&indented);
    match result {
        Ok(ast) => {
            let tree = ast::pretty::dump(&ast)?;
            let tree = indent(&tree);
            writeln!(file)?;
            writeln!(file, "#[test]")?;
            writeln!(file, "fn test_{name}() {{")?;
            writeln!(file, "    let src = r#\"{indented}\"#;")?;
            writeln!(file, "    let expected = r#\"{tree}\"#;")?;
            writeln!(file, "    assert_parse(src, expected);")?;
            writeln!(file, "}}")?;
        }
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

pub fn generate_resolver_tests(path: &Path, source: &str) -> Result<()> {
    let chapter = test_chapter(path);
    let this_file = PathBuf::from(file!());
    let output = this_file
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("src/semantic/test/test_{chapter}.rs"));

    let parent_mod = this_file
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("src/semantic/test.rs"));

    if !fs::exists(&parent_mod)? {
        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(&parent_mod)?;
        writeln!(
            file,
            r#"
use crate::parser::parse;
use crate::semantic::validate;
use crate::pretty::{{annotate, remove_annotation}};

fn assert_error(expected_annotated: &str) {{
    let clean_source = remove_annotation(expected_annotated);
    let ast = parse(&clean_source).expect("Parse error");
    let Err(error) = validate(ast) else {{
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
    let result = semantic::validate(parser::parse(&indented)?);
    match result {
        Ok(_) => {}
        Err(error) => {
            if !fs::read_to_string(&parent_mod)?.contains(&chapter) {
                let mut file = OpenOptions::new().create(true).append(true).open(&parent_mod)?;
                writeln!(file, "mod test_{chapter};")?;
            }
            if fs::read_to_string(&output).unwrap_or("".into()).is_empty() {
                let mut file = OpenOptions::new().create(true).write(true).open(&output)?;
                writeln!(file, "use super::{{assert_error}};")?;
            }
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

pub fn generate_tacky_tests(path: &Path, source: &str) -> Result<()> {
    let chapter = test_chapter(path);
    let this_file = PathBuf::from(file!());
    let output = this_file
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("src/tacky/test/test_{chapter}.rs"));

    let parent_mod = this_file
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("src/tacky/test.rs"));

    if !fs::read_to_string(&parent_mod).unwrap_or("".into()).contains(&chapter) {
        let mut file = OpenOptions::new().create(true).append(true).open(&parent_mod)?;
        writeln!(file, "mod test_{chapter};")?;
    }
    if fs::read_to_string(&output).unwrap_or("".into()).is_empty() {
        let mut file = OpenOptions::new().create(true).truncate(true).write(true).open(&output)?;
        writeln!(
            file,
            r#"
use crate::pretty::{{dedent, dump_tacky}};
"#
        )?;
    }
    let name = test_name(path);
    let mut file = OpenOptions::new().create(true).append(true).open(&output)?;
    let indented = indent(source);
    let ast = parser::parse(&indented)?;
    let (ast, semantics) = semantic::validate(ast)?;
    let tacky = tacky::emit(&ast, semantics);
    let expected = indent(&tacky::pretty::pp(&tacky)?);
    writeln!(file)?;
    writeln!(file, "#[test]")?;
    writeln!(file, "fn test_{name}() {{")?;
    writeln!(file, "    let src = r#\"{indented}\"#;")?;
    writeln!(file, "    let expected = r#\"{expected}\"#;")?;
    writeln!(file, "    assert_eq!(dump_tacky(src), dedent(expected));")?;
    writeln!(file, "}}")?;

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
                Some(i+1)
            } else {
                None
            }
        })
        .next()
        .unwrap();
    let components = &components[index..];
    components.join("_").strip_suffix(".c").unwrap().to_owned()
}

fn test_chapter(path: &Path) -> String {
    let components: Vec<_> = path
        .components()
        .map(|c| c.as_os_str().to_str().unwrap().to_owned())
        .collect();
    components
        .iter()
        .find(|c| c.starts_with("chapter"))
        .unwrap()
        .clone()
}
