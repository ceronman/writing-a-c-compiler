#[cfg(test)]
mod test;

use crate::ast::{
    AssignOp, BinaryOp, BlockItem, Declaration, Expression, Function, Identifier, Node, Program,
    Statement, UnaryOp,
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

    fn program(&mut self) -> Result<Node<Program>> {
        let start = self.current.span;
        let function_definition = self.function()?;
        self.expect(TokenKind::Eof)?;
        let span = Span(start.0, self.source.len());
        Ok(Node::from(
            span,
            Program {
                function_definition,
            },
        ))
    }

    fn function(&mut self) -> Result<Node<Function>> {
        let start = self.current.span;
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
        let end = self.current.span;
        Ok(Node::from(start + end, Function { name, body }))
    }

    fn block_item(&mut self) -> Result<Node<BlockItem>> {
        let start = self.current.span;
        let block = if self.current.kind == TokenKind::Int {
            self.advance();
            let name = self.identifier()?;
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
        let end = self.current.span;
        Ok(Node::from(start + end, block))
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

    fn expression(&mut self) -> Result<Node<Expression>> {
        self.expression_precedence(0)
    }

    fn expression_precedence(&mut self, min_precedence: u8) -> Result<Node<Expression>> {
        let mut expr = match self.current.kind {
            TokenKind::Constant => self.int()?,
            TokenKind::Identifier => self.var()?,
            TokenKind::Minus | TokenKind::Tilde | TokenKind::Bang => self.unary_expression()?,
            TokenKind::OpenParen => {
                let start = self.current.span;
                self.advance();
                let inner = self.expression()?;
                self.expect(TokenKind::CloseParen)?;
                let end = self.current.span;
                Node::from(start + end, *inner.data)
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
                TokenKind::Equal
                | TokenKind::PlusEqual
                | TokenKind::MinusEqual
                | TokenKind::StarEqual
                | TokenKind::SlashEqual
                | TokenKind::PercentEqual
                | TokenKind::AmpersandEqual
                | TokenKind::PipeEqual
                | TokenKind::CircumflexEqual
                | TokenKind::LessLessEqual
                | TokenKind::GreaterGreaterEqual => 1,
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

            expr = if let Ok(op) = self.assignment_op() {
                let left = expr;
                let right = self.expression_precedence(precedence)?;
                Node::from(
                    left.span + right.span,
                    Expression::Assignment { op, left, right },
                )
            } else {
                let op = self.binary_op()?;
                let left = expr;
                let right = self.expression_precedence(precedence + 1)?;

                Node::from(
                    left.span + right.span,
                    Expression::Binary { op, left, right },
                )
            }
        }

        Ok(expr)
    }

    fn unary_expression(&mut self) -> Result<Node<Expression>> {
        let start = self.current.span;
        let op = self.unary_op()?;
        let expr = self.expression_precedence(12)?;
        let end = self.current.span;
        Ok(Node::from(start + end, Expression::Unary { op, expr }))
    }

    fn assignment_op(&mut self) -> Result<Node<AssignOp>> {
        let op = match self.current.kind {
            TokenKind::Equal => AssignOp::Equal,
            TokenKind::PlusEqual => AssignOp::AddEqual,
            TokenKind::MinusEqual => AssignOp::SubEqual,
            TokenKind::StarEqual => AssignOp::MulEqual,
            TokenKind::SlashEqual => AssignOp::DivEqual,
            TokenKind::PercentEqual => AssignOp::ModEqual,
            TokenKind::AmpersandEqual => AssignOp::BitAndEqual,
            TokenKind::PipeEqual => AssignOp::BitOrEqual,
            TokenKind::CircumflexEqual => AssignOp::BitXorEqual,
            TokenKind::LessLessEqual => AssignOp::ShiftLeftEqual,
            TokenKind::GreaterGreaterEqual => AssignOp::ShiftRightEqual,
            _ => {
                return Err(ParserError {
                    msg: format!(
                        "Expected assignment operator, but found '{}'",
                        self.current.slice(self.source)
                    ),
                    span: self.current.span,
                });
            }
        };
        let span = self.current.span;
        self.advance();
        Ok(Node::from(span, op))
    }

    fn binary_op(&mut self) -> Result<Node<BinaryOp>> {
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
        let span = self.current.span;
        self.advance();
        Ok(Node::from(span, op))
    }

    fn unary_op(&mut self) -> Result<Node<UnaryOp>> {
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
        let span = self.current.span;
        self.advance();
        Ok(Node::from(span, op))
    }

    fn identifier(&mut self) -> Result<Node<Identifier>> {
        let token = self.expect(TokenKind::Identifier)?;
        let symbol = Symbol::from(token.slice(self.source));
        Ok(Node::from(token.span, Identifier { symbol }))
    }

    fn var(&mut self) -> Result<Node<Expression>> {
        let name = self.identifier()?;
        Ok(Node::from(name.span, Expression::Var(name.data.symbol)))
    }

    fn int(&mut self) -> Result<Node<Expression>> {
        let span = self.current.span;
        let token = self.expect(TokenKind::Constant)?;
        let value = token.slice(self.source).parse().map_err(|e| ParserError {
            msg: format!("Constant parsing error: {e}"),
            span,
        })?;
        Ok(Node::from(span, Expression::Constant(value)))
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

pub fn parse(source: &str) -> Result<Node<Program>> {
    Parser::new(source).program()
}
