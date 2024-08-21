use anyhow::bail;
use logos::{Logos, Span};

#[derive(Logos, Debug, PartialEq, Eq)]
pub enum TokenKind {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Ignored,

    #[regex(r"[a-zA-Z_]\w*")]
    Identifier,

    #[regex(r"[0-9]+")]
    Constant,

    #[regex(r"[0-9]+[a-zA-Z_]+", |_| None)]
    BadIdentifier,

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
}

#[derive(Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

pub fn lex(source: &str) -> anyhow::Result<Vec<Token>> {
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
