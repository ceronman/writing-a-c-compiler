#[cfg(test)]
mod test;

use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::str::Chars;

#[derive(Copy, Clone, Debug)]
pub struct Span(pub usize, pub usize);

impl Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        Span(std::cmp::min(self.0, rhs.0), std::cmp::max(self.1, rhs.1))
    }
}

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
    If,
    Else,
    Switch,
    Case,
    Default,
    Goto,
    Do,
    While,
    For,
    Break,
    Continue,

    Return,

    Plus,
    PlusPlus,
    PlusEqual,
    Minus,
    MinusMinus,
    MinusEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,
    Percent,
    PercentEqual,

    Ampersand,
    AmpersandAmpersand,
    AmpersandEqual,
    Pipe,
    PipePipe,
    PipeEqual,
    Tilde,
    Circumflex,
    CircumflexEqual,

    Bang,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessLess,
    LessLessEqual,
    LessEqual,
    Greater,
    GreaterGreater,
    GreaterGreaterEqual,
    GreaterEqual,
    Question,

    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Colon,
    Semicolon,

    Eof,
    Error,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokenKind::Identifier => "identifier",
            TokenKind::Constant => "constant",
            TokenKind::Int => "'int'",
            TokenKind::Void => "'void'",
            TokenKind::If => "'if'",
            TokenKind::Else => "'else'",
            TokenKind::Switch => "'switch'",
            TokenKind::Case => "'case'",
            TokenKind::Default => "'default'",
            TokenKind::Goto => "'goto'",
            TokenKind::Do => "'do'",
            TokenKind::While => "'while'",
            TokenKind::For => "'for'",
            TokenKind::Break => "'break'",
            TokenKind::Continue => "'continue'",
            TokenKind::Return => "'return'",
            TokenKind::Plus => "'+'",
            TokenKind::PlusPlus => "'++'",
            TokenKind::PlusEqual => "'+='",
            TokenKind::Minus => "'-'",
            TokenKind::MinusMinus => "'--'",
            TokenKind::MinusEqual => "'-='",
            TokenKind::Star => "'*'",
            TokenKind::StarEqual => "'*='",
            TokenKind::Slash => "'/'",
            TokenKind::SlashEqual => "'/='",
            TokenKind::Percent => "'%'",
            TokenKind::PercentEqual => "'%='",
            TokenKind::Ampersand => "'&'",
            TokenKind::AmpersandAmpersand => "'&&'",
            TokenKind::AmpersandEqual => "'&='",
            TokenKind::Pipe => "'|'",
            TokenKind::PipePipe => "'||'",
            TokenKind::PipeEqual => "'|='",
            TokenKind::Tilde => "'~'",
            TokenKind::Circumflex => "'^'",
            TokenKind::CircumflexEqual => "'^='",
            TokenKind::Bang => "'!'",
            TokenKind::Equal => "'='",
            TokenKind::EqualEqual => "'=='",
            TokenKind::BangEqual => "'!='",
            TokenKind::Less => "'<'",
            TokenKind::LessLess => "'<<'",
            TokenKind::LessLessEqual => "'<<='",
            TokenKind::LessEqual => "'<='",
            TokenKind::Greater => "'>'",
            TokenKind::GreaterGreater => "'>>'",
            TokenKind::GreaterGreaterEqual => "'>>='",
            TokenKind::GreaterEqual => "'>='",
            TokenKind::Question => "'?'",
            TokenKind::OpenParen => "'('",
            TokenKind::CloseParen => "')'",
            TokenKind::OpenBrace => "'{'",
            TokenKind::CloseBrace => "'}'",
            TokenKind::Colon => "':'",
            TokenKind::Semicolon => "';'",
            TokenKind::Eof => "end of file",
            TokenKind::Error => "error token",
        };
        write!(f, "{}", s)
    }
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
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            '?' => TokenKind::Question,
            '+' => match self.peek() {
                Some('+') => self.eat_and(TokenKind::PlusPlus),
                Some('=') => self.eat_and(TokenKind::PlusEqual),
                _ => TokenKind::Plus,
            },
            '-' => match self.peek() {
                Some('-') => self.eat_and(TokenKind::MinusMinus),
                Some('=') => self.eat_and(TokenKind::MinusEqual),
                _ => TokenKind::Minus,
            },
            '*' => match self.peek() {
                Some('=') => self.eat_and(TokenKind::StarEqual),
                _ => TokenKind::Star,
            },
            '/' => match self.peek() {
                Some('=') => self.eat_and(TokenKind::SlashEqual),
                _ => TokenKind::Slash,
            },
            '%' => match self.peek() {
                Some('=') => self.eat_and(TokenKind::PercentEqual),
                _ => TokenKind::Percent,
            },
            '~' => TokenKind::Tilde,
            '&' => match self.peek() {
                Some('&') => self.eat_and(TokenKind::AmpersandAmpersand),
                Some('=') => self.eat_and(TokenKind::AmpersandEqual),
                _ => TokenKind::Ampersand,
            },
            '|' => match self.peek() {
                Some('|') => self.eat_and(TokenKind::PipePipe),
                Some('=') => self.eat_and(TokenKind::PipeEqual),
                _ => TokenKind::Pipe,
            },
            '^' => match self.peek() {
                Some('=') => self.eat_and(TokenKind::CircumflexEqual),
                _ => TokenKind::Circumflex,
            },
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
                Some('>') => match self.peek_next() {
                    Some('=') => self.eat_eat_and(TokenKind::GreaterGreaterEqual),
                    _ => self.eat_and(TokenKind::GreaterGreater),
                },
                Some('=') => self.eat_and(TokenKind::GreaterEqual),
                _ => TokenKind::Greater,
            },

            '<' => match self.peek() {
                Some('<') => match self.peek_next() {
                    Some('=') => self.eat_eat_and(TokenKind::LessLessEqual),
                    _ => self.eat_and(TokenKind::LessLess),
                },
                Some('=') => self.eat_and(TokenKind::LessEqual),
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
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "switch" => TokenKind::Switch,
            "case" => TokenKind::Case,
            "default" => TokenKind::Default,
            "goto" => TokenKind::Goto,
            "do" => TokenKind::Do,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
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

    fn eat_and(&mut self, kind: TokenKind) -> TokenKind {
        self.advance();
        kind
    }

    fn eat_eat_and(&mut self, kind: TokenKind) -> TokenKind {
        self.advance();
        self.advance();
        kind
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

    fn peek_next(&self) -> Option<char> {
        self.chars.clone().nth(1)
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
