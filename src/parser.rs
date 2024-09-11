#[cfg(test)]
mod test;

use crate::ast::{
    AssignOp, BinaryOp, BlockItem, Declaration, Expression, Function, Identifier, Node, PostfixOp,
    Program, Statement, UnaryOp,
};
use crate::error::{CompilerError, ErrorKind, Result};
use crate::lexer::{Lexer, Token, TokenKind};
use crate::symbol::Symbol;

struct Parser<'src> {
    source: &'src str,
    current: Token,
    next: Token,
    lexer: Lexer<'src>,
}

impl<'src> Parser<'src> {
    fn new(source: &'src str) -> Self {
        let mut lexer = Lexer::new(source);
        Parser {
            source,
            current: lexer.next(),
            next: lexer.next(),
            lexer,
        }
    }

    fn program(&mut self) -> Result<Node<Program>> {
        let begin = self.current.span;
        let function_definition = self.function()?;
        let end = self.expect(TokenKind::Eof, "end of file")?.span;
        Ok(Node::from(
            begin + end,
            Program {
                function_definition,
            },
        ))
    }

    fn function(&mut self) -> Result<Node<Function>> {
        let begin = self.current.span;
        self.expect(TokenKind::Int, "'int'")?;
        let name = self.identifier()?;
        self.expect(TokenKind::OpenParen, "'('")?;
        self.expect(TokenKind::Void, "'void'")?;
        self.expect(TokenKind::CloseParen, "')'")?;
        self.expect(TokenKind::OpenBrace, "'{'")?;
        let mut body = Vec::new();
        while self.current.kind != TokenKind::CloseBrace {
            body.push(self.block_item()?)
        }
        let end = self.current.span;
        self.advance();
        Ok(Node::from(begin + end, Function { name, body }))
    }

    fn block_item(&mut self) -> Result<BlockItem> {
        let block = if self.current.kind == TokenKind::Int {
            BlockItem::Decl(self.declaration()?)
        } else {
            BlockItem::Stmt(self.statement()?)
        };
        Ok(block)
    }

    fn declaration(&mut self) -> Result<Node<Declaration>> {
        let begin = self.current.span;
        self.expect(TokenKind::Int, "type")?;
        let name = self.identifier()?;
        let init = if self.current.kind == TokenKind::Equal {
            self.advance();
            Some(self.expression()?)
        } else {
            None
        };
        let end = self.expect(TokenKind::Semicolon, "';'")?.span;
        Ok(Node::from(begin + end, Declaration { name, init }))
    }

    fn statement(&mut self) -> Result<Node<Statement>> {
        match self.current.kind {
            TokenKind::Return => self.return_stmt(),
            TokenKind::If => self.if_stmt(),
            TokenKind::Semicolon => self.null_stmt(),
            TokenKind::Goto => self.goto_stmt(),
            TokenKind::Identifier => {
                if self.next.kind == TokenKind::Colon {
                    self.labeled_stmt()
                } else {
                    self.expression_stmt()
                }
            }
            _ => self.expression_stmt(),
        }
    }

    fn goto_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        self.expect(TokenKind::Goto, "'goto'")?;
        let label = self.identifier()?;
        let end = self.expect(TokenKind::Semicolon, "';'")?.span;
        Ok(Node::from(begin + end, Statement::Goto(label)))
    }

    fn labeled_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        let name = self.identifier()?;
        self.expect(TokenKind::Colon, "':'")?;
        let stmt = self.statement()?;
        Ok(Node::from(
            begin + stmt.span,
            Statement::Labeled { name, stmt },
        ))
    }

    fn expression_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        let expr = self.expression_precedence(0, "statement")?;
        let end = self.expect(TokenKind::Semicolon, "';'")?.span;
        Ok(Node::from(begin + end, Statement::Expression(expr)))
    }

    fn null_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        let end = self.expect(TokenKind::Semicolon, "';'")?.span;
        Ok(Node::from(begin + end, Statement::Null))
    }

    fn if_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        self.expect(TokenKind::If, "'if'")?;
        self.expect(TokenKind::OpenParen, "'('")?;
        let cond = self.expression()?;
        self.expect(TokenKind::CloseParen, "')'")?;
        let then_stmt = self.statement()?;
        let else_stmt = if self.current.kind == TokenKind::Else {
            self.advance();
            Some(self.statement()?)
        } else {
            None
        };
        let end = else_stmt.as_ref().unwrap_or(&then_stmt).span;
        Ok(Node::from(
            begin + end,
            Statement::If {
                cond,
                then_stmt,
                else_stmt,
            },
        ))
    }

    fn return_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        self.expect(TokenKind::Return, "'return'")?;
        let expr = self.expression()?;
        let end = self.expect(TokenKind::Semicolon, "';'")?.span;
        Ok(Node::from(begin + end, Statement::Return { expr }))
    }

    fn expression(&mut self) -> Result<Node<Expression>> {
        self.expression_precedence(0, "expression")
    }

    fn expression_precedence(
        &mut self,
        min_precedence: u8,
        error_kind: &str,
    ) -> Result<Node<Expression>> {
        let mut expr = match self.current.kind {
            TokenKind::Constant => self.int()?,
            TokenKind::Identifier => self.var()?,
            TokenKind::Minus
            | TokenKind::Tilde
            | TokenKind::Bang
            | TokenKind::PlusPlus
            | TokenKind::MinusMinus => self.unary_expression()?,
            TokenKind::OpenParen => {
                let begin = self.current.span;
                self.advance();
                let inner = self.expression()?;
                let end = self.expect(TokenKind::CloseParen, "')'")?.span;
                Node::from(begin + end, *inner.data)
            }
            _ => {
                return Err(CompilerError {
                    kind: ErrorKind::Parse,
                    msg: format!(
                        "Expected {error_kind}, but found '{}'",
                        self.current.slice(self.source)
                    ),
                    span: self.current.span,
                });
            }
        };

        loop {
            // Postfix operator parsing
            if let TokenKind::PlusPlus | TokenKind::MinusMinus = self.current.kind {
                let precedence = 14;
                if precedence < min_precedence {
                    break;
                }
                let op = self.postfix_op()?;
                expr = Node::from(expr.span + op.span, Expression::Postfix { op, expr });
                continue;
            }

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
                TokenKind::Question => 2,
                TokenKind::PipePipe => 3,
                TokenKind::AmpersandAmpersand => 4,
                TokenKind::Pipe => 5,
                TokenKind::Circumflex => 6,
                TokenKind::Ampersand => 7,
                TokenKind::EqualEqual | TokenKind::BangEqual => 8,
                TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Less
                | TokenKind::LessEqual => 9,
                TokenKind::LessLess | TokenKind::GreaterGreater => 10,
                TokenKind::Plus | TokenKind::Minus => 11,
                TokenKind::Star | TokenKind::Slash | TokenKind::Percent => 12,
                _ => break,
            };

            if precedence < min_precedence {
                break;
            }

            expr = if let TokenKind::Question = self.current.kind {
                self.advance();
                let cond = expr;
                let then_expr = self.expression()?;
                self.expect(TokenKind::Colon, "':'")?;
                let else_expr = self.expression_precedence(precedence, "expression")?;
                Node::from(
                    cond.span + else_expr.span,
                    Expression::Conditional {
                        cond,
                        then_expr,
                        else_expr,
                    },
                )
            } else if let Ok(op) = self.assignment_op() {
                let left = expr;
                let right = self.expression_precedence(precedence, "expression")?;
                Node::from(
                    left.span + right.span,
                    Expression::Assignment { op, left, right },
                )
            } else {
                let op = self.binary_op()?;
                let left = expr;
                let right = self.expression_precedence(precedence + 1, "expression")?;

                Node::from(
                    left.span + right.span,
                    Expression::Binary { op, left, right },
                )
            }
        }

        Ok(expr)
    }

    fn unary_expression(&mut self) -> Result<Node<Expression>> {
        let op = self.unary_op()?;
        let expr = self.expression_precedence(13, "expression")?;
        Ok(Node::from(
            op.span + expr.span,
            Expression::Unary { op, expr },
        ))
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
                return Err(CompilerError {
                    kind: ErrorKind::Parse,
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
                return Err(CompilerError {
                    kind: ErrorKind::Parse,
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
            TokenKind::PlusPlus => UnaryOp::Increment,
            TokenKind::MinusMinus => UnaryOp::Decrement,
            _ => {
                return Err(CompilerError {
                    kind: ErrorKind::Parse,
                    msg: format!(
                        "Expected prefix unary operator, but found '{}'",
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

    fn postfix_op(&mut self) -> Result<Node<PostfixOp>> {
        let op = match self.current.kind {
            TokenKind::PlusPlus => PostfixOp::Increment,
            TokenKind::MinusMinus => PostfixOp::Decrement,
            _ => {
                return Err(CompilerError {
                    kind: ErrorKind::Parse,
                    msg: format!(
                        "Expected postfix unary operator, but found '{}'",
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
        let token = self.expect(TokenKind::Identifier, "identifier")?;
        let symbol = Symbol::from(token.slice(self.source));
        Ok(Node::from(token.span, Identifier { symbol }))
    }

    fn var(&mut self) -> Result<Node<Expression>> {
        let name = self.identifier()?;
        Ok(Node::from(name.span, Expression::Var(name.data.symbol)))
    }

    fn int(&mut self) -> Result<Node<Expression>> {
        let span = self.current.span;
        let token = self.expect(TokenKind::Constant, "constant")?;
        let value = token
            .slice(self.source)
            .parse()
            .map_err(|e| CompilerError {
                kind: ErrorKind::Parse,
                msg: format!("Constant parsing error: {e}"),
                span,
            })?;
        Ok(Node::from(span, Expression::Constant(value)))
    }

    fn advance(&mut self) {
        self.current = self.next;
        self.next = self.lexer.next();
    }

    fn expect(&mut self, expected: TokenKind, name: &str) -> Result<Token> {
        let token = self.current;
        if token.kind == expected {
            self.advance();
            Ok(token)
        } else if token.kind == TokenKind::Eof {
            Err(CompilerError {
                kind: ErrorKind::Parse,
                msg: "Unexpected end of file".to_owned(),
                span: token.span,
            })
        } else {
            Err(CompilerError {
                kind: ErrorKind::Parse,
                msg: format!(
                    "Expected {name}, but found '{}'",
                    token.slice(self.source)
                ),
                span: token.span,
            })
        }
    }
}

pub fn parse(source: &str) -> Result<Node<Program>> {
    Parser::new(source).program()
}
