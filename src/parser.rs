use crate::ast::{Expression, Function, Program, Statement, UnaryOp};
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
        let expr = match self.current.kind {
            TokenKind::Constant => self.int()?,
            TokenKind::Minus | TokenKind::Tilde => Expression::Unary {
                op: self.unary_op()?,
                expr: self.expression()?.into(),
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
        Ok(expr)
    }

    fn unary_op(&mut self) -> Result<UnaryOp> {
        let op = match self.current.kind {
            TokenKind::Minus => UnaryOp::Negate,
            TokenKind::Tilde => UnaryOp::Complement,
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
