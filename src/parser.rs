#[cfg(test)]
mod test;

use crate::ast::{
    BinaryOp, BlockItem, Declaration, Expression, Function, Program, Statement, UnaryOp,
};
use crate::lexer::{Lexer, Span, Token, TokenKind};
use crate::symbol::Symbol;
use std::error::Error;
use std::fmt::{Display, Formatter};

struct Parser<'src> {
    source: &'src str,
    current: Token,
    lexer: Lexer<'src>,
}

impl<'src> Parser<'src> {
    fn new(source: &'src str) -> Self {
        let mut lexer = Lexer::new(source);
        Parser {
            source,
            current: lexer.next(),
            lexer,
        }
    }

    fn program(&mut self) -> Result<Program> {
        let function_definition = self.function()?;
        self.expect(TokenKind::Eof)?;
        Ok(Program {
            function_definition,
        })
    }

    fn function(&mut self) -> Result<Function> {
        self.expect(TokenKind::Int)?;
        let name = self.identifier()?;
        self.expect(TokenKind::OpenParen)?;
        self.expect(TokenKind::Void)?;
        self.expect(TokenKind::CloseParen)?;
        self.expect(TokenKind::OpenBrace)?;
        let mut body = Vec::new();
        while self.current.kind != TokenKind::CloseBrace {
            body.push(self.block_item()?)
        }
        self.advance();
        Ok(Function { name, body })
    }

    fn block_item(&mut self) -> Result<BlockItem> {
        let block = if self.current.kind == TokenKind::Int {
            self.advance();
            let name = self.expect(TokenKind::Identifier)?;
            let name = name.slice(self.source).to_owned();
            let init = if self.current.kind == TokenKind::Equal {
                self.advance();
                Some(self.expression()?)
            } else {
                None
            };
            self.expect(TokenKind::Semicolon)?;
            BlockItem::Decl(Declaration { name, init })
        } else {
            BlockItem::Stmt(self.statement()?)
        };

        Ok(block)
    }

    fn statement(&mut self) -> Result<Statement> {
        let statement = match self.current.kind {
            TokenKind::Return => {
                self.advance();
                let expr = self.expression()?;
                self.expect(TokenKind::Semicolon)?;
                Statement::Return { expr }
            }
            TokenKind::Semicolon => {
                self.advance();
                Statement::Null
            }
            _ => {
                let expr = self.expression()?;
                self.expect(TokenKind::Semicolon)?;
                Statement::Expression(expr)
            }
        };

        Ok(statement)
    }

    fn expression(&mut self) -> Result<Expression> {
        self.expression_precedence(0)
    }

    fn expression_precedence(&mut self, min_precedence: u8) -> Result<Expression> {
        let mut expr = match self.current.kind {
            TokenKind::Constant => self.int()?,
            TokenKind::Identifier => self.var()?,
            TokenKind::Minus | TokenKind::Tilde | TokenKind::Bang => Expression::Unary {
                op: self.unary_op()?,
                expr: self.expression_precedence(12)?.into(),
            },
            TokenKind::OpenParen => {
                self.advance();
                let inner = self.expression()?;
                self.expect(TokenKind::CloseParen)?;
                inner
            }
            _ => {
                return Err(ParserError {
                    msg: format!(
                        "Expected expression, but found '{}'",
                        self.current.slice(self.source)
                    ),
                    span: self.current.span,
                });
            }
        };

        loop {
            let precedence = match self.current.kind {
                TokenKind::Equal => 1,
                TokenKind::PipePipe => 2,
                TokenKind::AmpersandAmpersand => 3,
                TokenKind::Pipe => 4,
                TokenKind::Circumflex => 5,
                TokenKind::Ampersand => 6,
                TokenKind::EqualEqual | TokenKind::BangEqual => 7,
                TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Less
                | TokenKind::LessEqual => 8,
                TokenKind::LessLess | TokenKind::GreaterGreater => 9,
                TokenKind::Plus | TokenKind::Minus => 10,
                TokenKind::Star | TokenKind::Slash | TokenKind::Percent => 11,
                _ => break,
            };

            if precedence < min_precedence {
                break;
            }

            expr = if let TokenKind::Equal = self.current.kind {
                self.advance();
                let left = expr;
                let right = self.expression_precedence(precedence)?;
                Expression::Assignment {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            } else {
                let op = self.binary_op()?;
                let left = expr;
                let right = self.expression_precedence(precedence + 1)?;

                Expression::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
        }

        Ok(expr)
    }

    fn binary_op(&mut self) -> Result<BinaryOp> {
        let op = match self.current.kind {
            TokenKind::Plus => BinaryOp::Add,
            TokenKind::Minus => BinaryOp::Subtract,
            TokenKind::Star => BinaryOp::Multiply,
            TokenKind::Slash => BinaryOp::Divide,
            TokenKind::Percent => BinaryOp::Reminder,
            TokenKind::Ampersand => BinaryOp::BinAnd,
            TokenKind::Pipe => BinaryOp::BinOr,
            TokenKind::Circumflex => BinaryOp::BinXor,
            TokenKind::Less => BinaryOp::LessThan,
            TokenKind::LessEqual => BinaryOp::LessOrEqualThan,
            TokenKind::LessLess => BinaryOp::ShiftLeft,
            TokenKind::Greater => BinaryOp::GreaterThan,
            TokenKind::GreaterEqual => BinaryOp::GreaterOrEqualThan,
            TokenKind::GreaterGreater => BinaryOp::ShiftRight,
            TokenKind::EqualEqual => BinaryOp::Equal,
            TokenKind::BangEqual => BinaryOp::NotEqual,
            TokenKind::AmpersandAmpersand => BinaryOp::And,
            TokenKind::PipePipe => BinaryOp::Or,

            _ => {
                return Err(ParserError {
                    msg: format!(
                        "Expected binary operator, but found '{}'",
                        self.current.slice(self.source)
                    ),
                    span: self.current.span,
                });
            }
        };
        self.advance();
        Ok(op)
    }

    fn unary_op(&mut self) -> Result<UnaryOp> {
        let op = match self.current.kind {
            TokenKind::Minus => UnaryOp::Negate,
            TokenKind::Tilde => UnaryOp::Complement,
            TokenKind::Bang => UnaryOp::Not,
            _ => {
                return Err(ParserError {
                    msg: format!(
                        "Expected unary operator, but found '{}'",
                        self.current.slice(self.source)
                    ),
                    span: self.current.span,
                });
            }
        };
        self.advance();
        Ok(op)
    }

    fn identifier(&mut self) -> Result<Symbol> {
        let token = self.expect(TokenKind::Identifier)?;
        Ok(Symbol::from(token.slice(self.source)))
    }

    fn var(&mut self) -> Result<Expression> {
        let name = self.identifier()?;
        Ok(Expression::Var(name))
    }

    fn int(&mut self) -> Result<Expression> {
        let span = self.current.span;
        let token = self.expect(TokenKind::Constant)?;
        let value = token.slice(self.source).parse().map_err(|e| ParserError {
            msg: format!("Constant parsing error: {e}"),
            span,
        })?;
        Ok(Expression::Constant(value))
    }

    fn advance(&mut self) {
        self.current = self.lexer.next();
    }

    fn expect(&mut self, expected: TokenKind) -> Result<Token> {
        let token = self.current;
        if token.kind == expected {
            self.advance();
            Ok(token)
        } else if token.kind == TokenKind::Eof {
            Err(ParserError {
                msg: "Unexpected end of file".to_owned(),
                span: token.span,
            })
        } else {
            Err(ParserError {
                msg: format!(
                    "Expected {expected:?}, but found '{}'",
                    token.slice(self.source)
                ),
                span: token.span,
            })
        }
    }
}

#[derive(Debug)]
pub struct ParserError {
    pub msg: String,
    pub span: Span,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{msg} at {span:?}", msg = self.msg, span = self.span)
    }
}

impl Error for ParserError {}

type Result<T> = std::result::Result<T, ParserError>;

pub fn parse(source: &str) -> Result<Program> {
    Parser::new(source).program()
}
