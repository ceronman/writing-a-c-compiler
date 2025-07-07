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
    IntConstant(IntKind),
    DoubleConstant,
    CharLiteral,

    Char,
    Int,
    Long,
    Void,
    Signed,
    Unsigned,
    Double,

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
    Static,
    Extern,

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
    OpenBracket,
    CloseBracket,
    Colon,
    Semicolon,
    Comma,

    Eof,
    Error,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IntKind {
    Int,
    Uint,
    Long,
    ULong,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokenKind::Identifier => "identifier",
            TokenKind::IntConstant(IntKind::Int) => "constant",
            TokenKind::IntConstant(IntKind::Uint) => "unsigned constant",
            TokenKind::IntConstant(IntKind::Long) => "long constant",
            TokenKind::IntConstant(IntKind::ULong) => "unsigned long constant",
            TokenKind::DoubleConstant => "double constant",
            TokenKind::CharLiteral => "character literal",
            TokenKind::Char => "'char'",
            TokenKind::Int => "'int'",
            TokenKind::Long => "'long'",
            TokenKind::Void => "'void'",
            TokenKind::Signed => "'signed'",
            TokenKind::Unsigned => "'unsigned'",
            TokenKind::Double => "'double'",
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
            TokenKind::Static => "'static'",
            TokenKind::Extern => "'extern'",
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
            TokenKind::OpenBracket => "'['",
            TokenKind::CloseBracket => "']'",
            TokenKind::OpenBrace => "'{'",
            TokenKind::CloseBrace => "'}'",
            TokenKind::Colon => "':'",
            TokenKind::Semicolon => "';'",
            TokenKind::Comma => "','",
            TokenKind::Eof => "end of file",
            TokenKind::Error => "error token",
        };
        write!(f, "{s}")
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
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            ',' => TokenKind::Comma,
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
            '0'..='9' | '.' => self.constant(c),
            '\'' => self.char_literal(),
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

    fn eat_numbers(&mut self) {
        while let Some('0'..='9') = self.peek() {
            self.advance();
        }
    }

    fn fraction(&mut self) -> TokenKind {
        self.eat_numbers();
        if let Some('e' | 'E') = self.peek() {
            self.advance();
            if let Some('+' | '-') = self.peek() {
                self.advance();
            }
            if !matches!(self.peek(), Some('0'..='9')) {
                return TokenKind::Error;
            }
            self.eat_numbers()
        }
        TokenKind::DoubleConstant
    }

    fn constant(&mut self, first: char) -> TokenKind {
        let kind = if first == '.' {
            self.fraction()
        } else {
            self.eat_numbers();
            match (self.peek(), self.peek_next()) {
                (Some('.'), _) => {
                    self.advance();
                    self.fraction()
                }
                (Some('e' | 'E'), _) => self.fraction(),
                (Some('u') | Some('U'), Some('l') | Some('L')) => {
                    self.advance();
                    self.advance();
                    TokenKind::IntConstant(IntKind::ULong)
                }
                (Some('l') | Some('L'), Some('u') | Some('U')) => {
                    self.advance();
                    self.advance();
                    TokenKind::IntConstant(IntKind::ULong)
                }
                (Some('l') | Some('L'), _) => {
                    self.advance();
                    TokenKind::IntConstant(IntKind::Long)
                }
                (Some('u') | Some('U'), _) => {
                    self.advance();
                    TokenKind::IntConstant(IntKind::Uint)
                }
                _ => TokenKind::IntConstant(IntKind::Int),
            }
        };

        match self.peek() {
            Some(c) if c.is_alphanumeric() => TokenKind::Error,
            Some('.' | '_') => TokenKind::Error,
            _ => kind,
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
            "long" => TokenKind::Long,
            "void" => TokenKind::Void,
            "signed" => TokenKind::Signed,
            "unsigned" => TokenKind::Unsigned,
            "double" => TokenKind::Double,
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
            "static" => TokenKind::Static,
            "extern" => TokenKind::Extern,
            _ => TokenKind::Identifier,
        }
    }
    
    fn char_literal(&mut self) -> TokenKind {
        // The opening single quote is already consumed by token_kind
        
        let Some(c) = self.peek() else {
            return TokenKind::Error;
        };
        
        match self.peek() {
            Some('\\') => {
                self.advance(); // Consume the backslash
                let escape = self.peek();

                match escape {
                    Some('\'') | Some('"') | Some('?') | Some('\\') |
                    Some('a') | Some('b') | Some('f') | Some('n') |
                    Some('r') | Some('v') => {
                        self.advance(); // Consume the escape character
                    }
                    _ => return TokenKind::Error, // Invalid escape sequence
                }
            }
            None | Some('\n') => return TokenKind::Error,
            Some(c) if !c.is_ascii() => return TokenKind::Error,
            _ => {
                self.advance();
            }
        }
        
        // Check for closing single quote
        if !self.eat('\'') {
            return TokenKind::Error;
        }
        
        TokenKind::CharLiteral
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
