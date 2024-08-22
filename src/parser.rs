use crate::ast::{Expression, Function, Program, Statement, Symbol};
use crate::lexer::{Lexer, Token, TokenKind};

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
        let value = self.int()?;
        Ok(Expression::Constant(value))
    }

    fn identifier(&mut self) -> Result<Symbol> {
        let token = self.expect(TokenKind::Identifier)?;
        Ok(Symbol::from(token.slice(self.source)))
    }

    fn int(&mut self) -> Result<i64> {
        let token = self.expect(TokenKind::Constant)?;
        Ok(token.slice(self.source).parse()?)
    }

    fn advance(&mut self) {
        self.current = self.lexer.next();
    }

    fn expect(&mut self, token_kind: TokenKind) -> Result<Token> {
        let token = self.current;
        if token.kind == token_kind {
            self.advance();
            Ok(token)
        } else {
            Err(anyhow!(
                "Expected {token_kind:?}, but found '{}' at {:?}",
                token.slice(self.source),
                token.span
            ))
        }
    }
}

pub fn parse(source: &str) -> Result<Program> {
    Parser::new(source).program()
}
