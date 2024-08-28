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
        .join("src/lexer/test.rs");
    if !output.exists() {
        let mut file = OpenOptions::new().create(true).write(true).open(&output)?;
        writeln!(file, "use crate::lexer::tokenize;")?;
        writeln!(file, "use crate::lexer::TokenKind::*;")?;
    }
    let components: Vec<_> = path.components().map(|c| c.as_os_str().to_str().unwrap().to_owned()).collect();
    let components = &components[(components.len() - 3)..];
    let name = components.join("_").strip_suffix(".c").unwrap().to_owned();
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
