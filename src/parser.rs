#[cfg(test)]
mod test;

use crate::ast::{
    AssignOp, BinaryOp, Block, BlockItem, Constant, Declaration, Expression, ForInit,
    FunctionDeclaration, FunctionType, Identifier, Node, PostfixOp, Program, Statement,
    StorageClass, SwitchLabels, Type, UnaryOp, VarDeclaration,
};
use crate::error::{CompilerError, ErrorKind, Result};
use crate::lexer::{Lexer, Span, Token, TokenKind};
use crate::symbol::Symbol;

struct Parser<'src> {
    source: &'src str,
    current: Token,
    next: Token,
    lexer: Lexer<'src>,
    node_id: u32,
}

impl<'src> Parser<'src> {
    fn new(source: &'src str) -> Self {
        let mut lexer = Lexer::new(source);
        Parser {
            source,
            current: lexer.next(),
            next: lexer.next(),
            lexer,
            node_id: 0,
        }
    }

    fn program(&mut self) -> Result<Node<Program>> {
        let begin = self.current.span;
        let mut declarations = Vec::new();
        while self.current.kind != TokenKind::Eof {
            declarations.push(self.declaration()?);
        }
        let end = self.expect(TokenKind::Eof)?.span;
        Ok(self.node(begin + end, Program { declarations }))
    }

    fn block(&mut self) -> Result<Node<Block>> {
        let begin = self.current.span;
        self.expect(TokenKind::OpenBrace)?;
        let mut items = Vec::new();
        while self.current.kind != TokenKind::CloseBrace {
            items.push(self.block_item()?)
        }
        let end = self.current.span;
        self.advance();
        Ok(self.node(begin + end, Block { items }))
    }

    fn block_item(&mut self) -> Result<BlockItem> {
        let block = if self.is_specifier() {
            BlockItem::Decl(self.declaration()?)
        } else {
            BlockItem::Stmt(self.statement()?)
        };
        Ok(block)
    }

    fn declaration(&mut self) -> Result<Node<Declaration>> {
        let begin = self.current.span;
        let (ty, storage_class) = self.type_and_storage()?;
        let name = self.identifier()?;

        if self.matches(TokenKind::OpenParen) {
            let (params, types) = if let TokenKind::Void = self.current.kind {
                self.advance();
                self.expect(TokenKind::CloseParen)?;
                (vec![], vec![])
            } else {
                let mut params = Vec::new();
                let mut types = Vec::new();
                loop {
                    types.push(*self.type_specifier()?.data);
                    params.push(self.identifier()?);

                    if self.matches(TokenKind::Comma) {
                        continue;
                    }
                    self.expect(TokenKind::CloseParen)?;
                    break;
                }
                (params, types)
            };
            let function_ty = FunctionType {
                params: types,
                ret: ty.data,
            };
            if let TokenKind::OpenBrace = self.current.kind {
                let body = self.block()?;
                Ok(self.node(
                    begin + body.span,
                    Declaration::Function(FunctionDeclaration {
                        name,
                        params,
                        body: Some(body),
                        ty: function_ty,
                        storage_class,
                    }),
                ))
            } else {
                let end = self.expect(TokenKind::Semicolon)?.span;
                Ok(self.node(
                    begin + end,
                    Declaration::Function(FunctionDeclaration {
                        name,
                        params,
                        body: None,
                        ty: function_ty,
                        storage_class,
                    }),
                ))
            }
        } else {
            let init = if self.matches(TokenKind::Equal) {
                Some(self.expression()?)
            } else {
                None
            };
            let end = self.expect(TokenKind::Semicolon)?.span;
            Ok(self.node(
                begin + end,
                Declaration::Var(VarDeclaration {
                    name,
                    init,
                    ty: *ty.data,
                    storage_class,
                }),
            ))
        }
    }

    fn is_specifier(&self) -> bool {
        matches!(
            self.current.kind,
            TokenKind::Int | TokenKind::Long | TokenKind::Extern | TokenKind::Static
        )
    }

    fn type_specifier(&mut self) -> Result<Node<Type>> {
        let mut types = Vec::new();
        let begin = self.current.span;
        let mut end = self.current.span;
        while let TokenKind::Int | TokenKind::Long = self.current.kind {
            types.push(self.current.kind);
            end = self.current.span;
            self.advance()
        }
        self.type_from_list(begin, end, &types)
    }

    fn type_from_list(
        &mut self,
        begin: Span,
        end: Span,
        types: &[TokenKind],
    ) -> Result<Node<Type>> {
        if types.is_empty() {
            return Err(CompilerError {
                kind: ErrorKind::Parse,
                msg: "Expected type specifier".into(),
                span: self.current.span,
            });
        }
        let ty = match types {
            [TokenKind::Int] => Type::Int,
            [TokenKind::Long] => Type::Long,
            [TokenKind::Long, TokenKind::Int] => Type::Long,
            [TokenKind::Int, TokenKind::Long] => Type::Long,
            _ => {
                return Err(CompilerError {
                    kind: ErrorKind::Parse,
                    msg: "Invalid type specifier".to_string(),
                    span: begin + end,
                })
            }
        };
        Ok(self.node(begin + end, ty))
    }

    fn type_and_storage(&mut self) -> Result<(Node<Type>, Option<Node<StorageClass>>)> {
        let begin = self.current.span;
        let mut end = self.current.span;
        let mut types = Vec::new();
        let mut storage: Option<Node<StorageClass>> = None;
        loop {
            let token = self.current;
            match token.kind {
                TokenKind::Int | TokenKind::Long => {
                    types.push(token.kind);
                    end = self.current.span;
                    self.advance();
                }
                TokenKind::Static | TokenKind::Extern => {
                    if storage.is_some() {
                        return Err(CompilerError {
                            kind: ErrorKind::Parse,
                            msg: "Duplicated storage class in declaration".to_string(),
                            span: token.span,
                        });
                    }
                    let s = match token.kind {
                        TokenKind::Static => StorageClass::Static,
                        TokenKind::Extern => StorageClass::Extern,
                        _ => unreachable!(),
                    };
                    storage = Some(self.node(token.span, s));
                    self.advance();
                }
                _ => break,
            }
        }
        let ty = self.type_from_list(begin, end, &types)?;
        Ok((ty, storage))
    }

    fn statement(&mut self) -> Result<Node<Statement>> {
        match self.current.kind {
            TokenKind::Return => self.return_stmt(),
            TokenKind::If => self.if_stmt(),
            TokenKind::Switch => self.switch_stmt(),
            TokenKind::Semicolon => self.null_stmt(),
            TokenKind::Goto => self.goto_stmt(),
            TokenKind::While => self.while_stmt(),
            TokenKind::Do => self.do_while_stmt(),
            TokenKind::For => self.for_stmt(),
            TokenKind::Continue => self.continue_stmt(),
            TokenKind::Break => self.break_stmt(),
            TokenKind::OpenBrace => self.compound_stmt(),
            TokenKind::Case => self.case_stmt(),
            TokenKind::Default => self.default_stmt(),
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

    fn while_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        self.expect(TokenKind::While)?;
        self.expect(TokenKind::OpenParen)?;
        let cond = self.expression()?;
        self.expect(TokenKind::CloseParen)?;
        let body = self.statement()?;
        Ok(self.node(
            begin + body.span,
            Statement::While {
                cond,
                body,
                label: "dummy".into(),
            },
        ))
    }

    fn do_while_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        self.expect(TokenKind::Do)?;
        let body = self.statement()?;
        self.expect(TokenKind::While)?;
        self.expect(TokenKind::OpenParen)?;
        let cond = self.expression()?;
        self.expect(TokenKind::CloseParen)?;
        let end = self.expect(TokenKind::Semicolon)?.span;
        Ok(self.node(
            begin + end,
            Statement::DoWhile {
                cond,
                body,
                label: "dummy".into(),
            },
        ))
    }

    fn for_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        self.expect(TokenKind::For)?;
        self.expect(TokenKind::OpenParen)?;
        let init = match self.current.kind {
            TokenKind::Semicolon => {
                self.advance();
                ForInit::None
            }
            // TODO: unify with self.specifier()
            TokenKind::Int | TokenKind::Long | TokenKind::Static | TokenKind::Extern => {
                let Node { span, data, .. } = self.declaration()?;
                let Declaration::Var(decl) = *data else {
                    return Err(CompilerError {
                        kind: ErrorKind::Parse,
                        msg: "Expected variable declaration, but found function declaration".into(),
                        span,
                    });
                };
                ForInit::Decl(self.node(span, decl))
            }
            _ => {
                let expr = self.expression()?;
                self.expect(TokenKind::Semicolon)?;
                ForInit::Expr(expr)
            }
        };

        let cond = if self.current.kind == TokenKind::Semicolon {
            None
        } else {
            Some(self.expression()?)
        };
        self.expect(TokenKind::Semicolon)?;

        let post = if self.current.kind == TokenKind::CloseParen {
            None
        } else {
            Some(self.expression()?)
        };
        self.expect(TokenKind::CloseParen)?;
        let body = self.statement()?;
        Ok(self.node(
            begin + body.span,
            Statement::For {
                init,
                cond,
                post,
                body,
                label: "dummy".into(),
            },
        ))
    }

    fn continue_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.expect(TokenKind::Continue)?.span;
        let end = self.expect(TokenKind::Semicolon)?.span;
        Ok(self.node(begin + end, Statement::Continue("dummy".into())))
    }

    fn break_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.expect(TokenKind::Break)?.span;
        let end = self.expect(TokenKind::Semicolon)?.span;
        Ok(self.node(begin + end, Statement::Break("dummy".into())))
    }

    fn compound_stmt(&mut self) -> Result<Node<Statement>> {
        let block = self.block()?;
        Ok(self.node(block.span, Statement::Compound(block)))
    }

    fn goto_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        self.expect(TokenKind::Goto)?;
        let label = self.identifier()?;
        let end = self.expect(TokenKind::Semicolon)?.span;
        Ok(self.node(begin + end, Statement::Goto(label)))
    }

    fn labeled_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        let name = self.identifier()?;
        self.expect(TokenKind::Colon)?;
        let stmt = self.statement()?;
        Ok(self.node(begin + stmt.span, Statement::Labeled { name, body: stmt }))
    }

    fn case_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        self.expect(TokenKind::Case)?;
        let value = self.expression()?;
        self.expect(TokenKind::Colon)?;
        let stmt = self.statement()?;
        Ok(self.node(
            begin + stmt.span,
            Statement::Case {
                value,
                body: stmt,
                label: "dummy".into(),
            },
        ))
    }

    fn default_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        self.expect(TokenKind::Default)?;
        self.expect(TokenKind::Colon)?;
        let stmt = self.statement()?;
        Ok(self.node(
            begin + stmt.span,
            Statement::Default {
                body: stmt,
                label: "dummy".into(),
            },
        ))
    }

    fn expression_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        let expr = self.expression_precedence(0, "statement")?;
        let end = self.expect(TokenKind::Semicolon)?.span;
        Ok(self.node(begin + end, Statement::Expression(expr)))
    }

    fn null_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        let end = self.expect(TokenKind::Semicolon)?.span;
        Ok(self.node(begin + end, Statement::Null))
    }

    fn if_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        self.expect(TokenKind::If)?;
        self.expect(TokenKind::OpenParen)?;
        let cond = self.expression()?;
        self.expect(TokenKind::CloseParen)?;
        let then_stmt = self.statement()?;
        let else_stmt = if self.matches(TokenKind::Else) {
            Some(self.statement()?)
        } else {
            None
        };
        let end = else_stmt.as_ref().unwrap_or(&then_stmt).span;
        Ok(self.node(
            begin + end,
            Statement::If {
                cond,
                then_stmt,
                else_stmt,
            },
        ))
    }

    fn switch_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        self.expect(TokenKind::Switch)?;
        self.expect(TokenKind::OpenParen)?;
        let cond = self.expression()?;
        self.expect(TokenKind::CloseParen)?;
        let body = self.statement()?;
        Ok(self.node(
            begin + body.span,
            Statement::Switch {
                expr: cond,
                body,
                labels: SwitchLabels::default(),
            },
        ))
    }

    fn return_stmt(&mut self) -> Result<Node<Statement>> {
        let begin = self.current.span;
        self.expect(TokenKind::Return)?;
        let expr = self.expression()?;
        let end = self.expect(TokenKind::Semicolon)?.span;
        Ok(self.node(begin + end, Statement::Return(expr)))
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
            TokenKind::Constant | TokenKind::LongConstant => self.constant()?,
            TokenKind::Identifier => {
                if self.next.kind == TokenKind::OpenParen {
                    self.function_call()?
                } else {
                    self.var()?
                }
            }
            TokenKind::Minus
            | TokenKind::Tilde
            | TokenKind::Bang
            | TokenKind::PlusPlus
            | TokenKind::MinusMinus => self.unary_expression()?,
            TokenKind::OpenParen => {
                let begin = self.current.span;
                self.advance();
                if let Ok(target) = self.type_specifier() {
                    self.expect(TokenKind::CloseParen)?;
                    let expr = self.expression_precedence(13, "expression")?;
                    self.node(begin + expr.span, Expression::Cast { target, expr })
                } else {
                    let inner = self.expression()?;
                    let end = self.expect(TokenKind::CloseParen)?.span;
                    self.node(begin + end, *inner.data)
                }
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
                expr = self.node(expr.span + op.span, Expression::Postfix { op, expr });
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

            expr = if self.matches(TokenKind::Question) {
                let cond = expr;
                let then_expr = self.expression()?;
                self.expect(TokenKind::Colon)?;
                let else_expr = self.expression_precedence(precedence, "expression")?;
                self.node(
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
                self.node(
                    left.span + right.span,
                    Expression::Assignment { op, left, right },
                )
            } else {
                let op = self.binary_op()?;
                let left = expr;
                let right = self.expression_precedence(precedence + 1, "expression")?;

                self.node(
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
        Ok(self.node(op.span + expr.span, Expression::Unary { op, expr }))
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
        Ok(self.node(span, op))
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
        Ok(self.node(span, op))
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
        Ok(self.node(span, op))
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
        Ok(self.node(span, op))
    }

    fn identifier(&mut self) -> Result<Node<Identifier>> {
        let token = self.expect(TokenKind::Identifier)?;
        let symbol = Symbol::from(token.slice(self.source));
        Ok(self.node(token.span, Identifier { symbol }))
    }

    fn function_call(&mut self) -> Result<Node<Expression>> {
        let name = self.identifier()?;
        let mut args = Vec::new();
        self.expect(TokenKind::OpenParen)?;
        if self.current.kind != TokenKind::CloseParen {
            loop {
                args.push(self.expression()?);
                if self.matches(TokenKind::Comma) {
                    continue;
                }
                break;
            }
        }
        self.expect(TokenKind::CloseParen)?;
        Ok(self.node(name.span, Expression::FunctionCall { name, args }))
    }

    fn var(&mut self) -> Result<Node<Expression>> {
        let name = self.identifier()?;
        Ok(self.node(name.span, Expression::Var(name.data.symbol)))
    }

    fn constant(&mut self) -> Result<Node<Expression>> {
        let token = self.current;
        let constant = match token.kind {
            TokenKind::Constant => {
                let value: i64 = token
                    .slice(self.source)
                    .parse()
                    .map_err(|_| CompilerError {
                        kind: ErrorKind::Parse,
                        msg: "Integer constant out of range".into(),
                        span: token.span,
                    })?;
                if value > i32::MAX as i64 {
                    Constant::Long(value)
                } else {
                    Constant::Int(value as i32)
                }
            }
            TokenKind::LongConstant => Constant::Long(
                token
                    .slice(self.source)
                    .strip_suffix(&['l', 'L'])
                    .unwrap()
                    .parse()
                    .map_err(|_| CompilerError {
                        kind: ErrorKind::Parse,
                        msg: "Long integer constant out of range".into(),
                        span: token.span,
                    })?,
            ),
            _ => {
                return Err(CompilerError {
                    kind: ErrorKind::Parse,
                    msg: format!(
                        "Expected constant but found '{}'",
                        self.current.slice(self.source)
                    ),
                    span: token.span,
                });
            }
        };
        self.advance();
        Ok(self.node(token.span, Expression::Constant(constant)))
    }

    fn advance(&mut self) {
        self.current = self.next;
        self.next = self.lexer.next();
    }

    fn matches(&mut self, expected: TokenKind) -> bool {
        if self.current.kind == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, expected: TokenKind) -> Result<Token> {
        let token = self.current;
        if token.kind == expected {
            self.advance();
            Ok(token)
        } else {
            let found = if token.kind == TokenKind::Eof {
                "end of file"
            } else {
                &format!("'{}'", token.slice(self.source))
            };

            Err(CompilerError {
                kind: ErrorKind::Parse,
                msg: format!("Expected {expected}, but found {found}"),
                span: token.span,
            })
        }
    }

    fn node<T>(&mut self, span: Span, data: T) -> Node<T> {
        self.node_id += 1;
        Node {
            id: self.node_id,
            span,
            data: Box::new(data),
        }
    }
}

pub fn parse(source: &str) -> Result<Node<Program>> {
    Parser::new(source).program()
}
