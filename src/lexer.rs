#[cfg(test)]
mod test;

use std::str::Chars;

#[derive(Copy, Clone, Debug)]
pub struct Span(pub usize, pub usize);

#[derive(Copy, Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    Constant,

    Int,
    Void,
    Return,

    Plus,
    PlusPlus,
    Minus,
    MinusMinus,
    Star,
    Slash,
    Percent,

    Ampersand,
    AmpersandAmpersand,
    Pipe,
    PipePipe,
    Tilde,
    Circumflex,

    Bang,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessLess,
    LessEqual,
    Greater,
    GreaterGreater,
    GreaterEqual,

    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,

    Eof,
    Error,
}

pub struct Lexer<'src> {
    source: &'src str,
    chars: Chars<'src>,
    start: usize,
    offset: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            chars: source.chars(),
            start: 0,
            offset: 0,
        }
    }

    pub fn next(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.offset;
        let c = self.advance();

        Token {
            kind: self.token_kind(c),
            span: Span(self.start, self.offset),
        }
    }

    fn token_kind(&mut self, c: Option<char>) -> TokenKind {
        let Some(c) = c else { return TokenKind::Eof };
        match c {
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            ';' => TokenKind::Semicolon,
            '+' => {
                if self.eat('+') {
                    TokenKind::PlusPlus
                } else {
                    TokenKind::Plus
                }
            }
            '-' => {
                if self.eat('-') {
                    TokenKind::MinusMinus
                } else {
                    TokenKind::Minus
                }
            }
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '~' => TokenKind::Tilde,
            '&' => {
                if self.eat('&') {
                    TokenKind::AmpersandAmpersand
                } else {
                    TokenKind::Ampersand
                }
            }
            '|' => {
                if self.eat('|') {
                    TokenKind::PipePipe
                } else {
                    TokenKind::Pipe
                }
            }
            '^' => TokenKind::Circumflex,
            '=' => {
                if self.eat('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }
            '!' => {
                if self.eat('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }
            '>' => match self.peek() {
                Some('>') => {
                    self.advance();
                    TokenKind::GreaterGreater
                }
                Some('=') => {
                    self.advance();
                    TokenKind::GreaterEqual
                }
                _ => TokenKind::Greater,
            },

            '<' => match self.peek() {
                Some('<') => {
                    self.advance();
                    TokenKind::LessLess
                }
                Some('=') => {
                    self.advance();
                    TokenKind::LessEqual
                }
                _ => TokenKind::Less,
            },
            '0'..='9' => self.constant(),
            c if c == '_' || c.is_alphabetic() => self.identifier(),
            _ => TokenKind::Error,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                return;
            };
        }
    }

    fn constant(&mut self) -> TokenKind {
        while let Some('0'..='9') = self.peek() {
            self.advance();
        }
        match self.peek() {
            Some(c) if c.is_alphanumeric() => TokenKind::Error,
            _ => TokenKind::Constant,
        }
    }

    fn identifier(&mut self) -> TokenKind {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        match &self.source[self.start..self.offset] {
            "int" => TokenKind::Int,
            "void" => TokenKind::Void,
            "return" => TokenKind::Return,
            _ => TokenKind::Identifier,
        }
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.chars.next() {
            self.offset += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    fn eat(&mut self, expected: char) -> bool {
        match self.peek() {
            Some(c) if c == expected => {
                self.advance();
                true
            }
            _ => false,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }
}

impl Token {
    pub fn slice(self, src: &str) -> &str {
        &src[self.span.0..self.span.1]
    }
}

pub fn tokenize(source: &str) -> Vec<TokenKind> {
    let mut result = Vec::new();
    let mut lexer = Lexer::new(source);
    loop {
        let token = lexer.next();
        match token.kind {
            TokenKind::Error => panic!(
                "Unexpected character '{}' at {:?}",
                token.slice(source),
                token.span
            ),
            TokenKind::Eof => break,
            _ => result.push(token.kind),
        }
    }
    result
}
