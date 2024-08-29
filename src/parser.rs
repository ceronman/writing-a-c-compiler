#[cfg(test)]
mod test;

use crate::ast::{BinaryOp, Expression, Function, Program, Statement, UnaryOp};
use crate::lexer::{Lexer, Token, TokenKind};

use crate::symbol::Symbol;
use anyhow::{anyhow, Result};

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
        let body = self.statement()?;
        self.expect(TokenKind::CloseBrace)?;
        Ok(Function { name, body })
    }

    fn statement(&mut self) -> Result<Statement> {
        self.expect(TokenKind::Return)?;
        let expr = self.expression()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Statement::Return { expr })
    }

    fn expression(&mut self) -> Result<Expression> {
        self.expression_precedence(0)
    }

    fn expression_precedence(&mut self, min_precedence: u8) -> Result<Expression> {
        let mut expr = match self.current.kind {
            TokenKind::Constant => self.int()?,
            TokenKind::Minus | TokenKind::Tilde | TokenKind::Bang => Expression::Unary {
                op: self.unary_op()?,
                expr: self.expression_precedence(11)?.into(),
            },
            TokenKind::OpenParen => {
                self.advance();
                let inner = self.expression()?;
                self.expect(TokenKind::CloseParen)?;
                inner
            }
            _ => {
                return Err(anyhow!(
                    "Expected expression, but found '{}' at {:?}",
                    self.current.slice(self.source),
                    self.current.span
                ))
            }
        };

        loop {
            let Ok(op) = self.binary_op() else {
                break;
            };

            use BinaryOp::*;
            let precedence = match op {
                Or => 1,
                And => 2,
                BinOr => 3,
                BinXor => 4,
                BinAnd => 5,
                Equal | NotEqual => 6,
                GreaterThan | GreaterOrEqualThan | LessThan | LessOrEqualThan => 7,
                ShiftLeft | ShiftRight => 8,
                Add | Subtract => 9,
                Multiply | Divide | Reminder => 10,
            };

            if precedence < min_precedence {
                break;
            }
            let left = expr;
            self.advance();
            let right = self.expression_precedence(precedence + 1)?;

            expr = Expression::Binary {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
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
                return Err(anyhow!(
                    "Expected binary operator, but found '{}' at {:?}",
                    self.current.slice(self.source),
                    self.current.span
                ))
            }
        };
        Ok(op)
    }

    fn unary_op(&mut self) -> Result<UnaryOp> {
        let op = match self.current.kind {
            TokenKind::Minus => UnaryOp::Negate,
            TokenKind::Tilde => UnaryOp::Complement,
            TokenKind::Bang => UnaryOp::Not,
            _ => {
                return Err(anyhow!(
                    "Expected unary operator, but found '{}' at {:?}",
                    self.current.slice(self.source),
                    self.current.span
                ))
            }
        };
        self.advance();
        Ok(op)
    }

    fn identifier(&mut self) -> Result<Symbol> {
        let token = self.expect(TokenKind::Identifier)?;
        Ok(Symbol::from(token.slice(self.source)))
    }

    fn int(&mut self) -> Result<Expression> {
        let token = self.expect(TokenKind::Constant)?;
        let value = token.slice(self.source).parse()?;
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
        } else {
            Err(anyhow!(
                "Expected {expected:?}, but found '{}' at {:?}",
                token.slice(self.source),
                token.span
            ))
        }
    }
}

pub fn parse(source: &str) -> Result<Program> {
    Parser::new(source).program()
}
