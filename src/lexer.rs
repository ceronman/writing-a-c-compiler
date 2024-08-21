use anyhow::{bail, Result};
use logos::{Logos, Span};

#[derive(Logos, Debug, PartialEq, Eq)]
pub enum TokenKind {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Ignored,

    #[regex(r"[a-zA-Z_]\w*")]
    Identifier,

    #[regex(r"[0-9]+")]
    Constant,

    #[token("int")]
    Int,

    #[token("void")]
    Void,

    #[token("return")]
    Return,

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token(";")]
    Semicolon,

    // Workaround to deal with the fact that some tests expect word boundaries
    // to be respected by the lexer. So, for example, "123foo" should be a
    // lexical error. Without this we would parse that as [Constant, Identifier]
    // instead of an error.
    #[regex(r"[0-9]+[a-zA-Z_]+", |_| None)]
    BadConstant,
}

#[derive(Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

pub fn lex(source: &str) -> Result<Vec<Token>> {
    let mut result = Vec::new();
    let mut lexer = TokenKind::lexer(source);
    while let Some(t) = lexer.next() {
        let span = lexer.span();
        let Ok(kind) = t else {
            bail!("Unexpected token {} at {:?}", lexer.slice(), lexer.span())
        };
        result.push(Token { kind, span })
    }
    Ok(result)
}
