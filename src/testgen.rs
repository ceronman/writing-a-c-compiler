use std::fs::OpenOptions;
use std::io::Write;
use std::panic;
use std::path::{Path, PathBuf};

use crate::lexer;
use anyhow::Result;

pub fn generate_lexer_tests(path: &Path, source: &str) -> Result<()> {
    let output = PathBuf::from(file!());
    let output = output
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("generated_lexer_tests.rs");
    let name = path.file_stem().unwrap().to_str().unwrap();
    let mut file = OpenOptions::new().create(true).append(true).open(output)?;
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
            writeln!(file, "fn test_failure_{name}() {{")?;
            writeln!(file, "    tokenize(r#\"")?;
            writeln!(file, "{indented}")?;
            writeln!(file, "    \"#);")?;
            writeln!(file, "}}")?;
        }
    }

    Ok(())
}