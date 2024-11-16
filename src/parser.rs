#[cfg(test)]
mod test;

use crate::asm::ir::CondCode::P;
use crate::ast::{
    AssignOp, BinaryOp, Block, BlockItem, Constant, Declaration, Expression, ForInit,
    FunctionDeclaration, FunctionType, Identifier, Node, PostfixOp, Program, Statement,
    StorageClass, Type, UnaryOp, VarDeclaration,
};
use crate::error::{CompilerError, ErrorKind, Result};
use crate::lexer::{IntKind, Lexer, Span, Token, TokenKind};
use crate::symbol::Symbol;
use std::fmt::Pointer;

struct Parser<'src> {
    source: &'src str,
    current: Token,
    next: Token,
    lexer: Lexer<'src>,
    node_id: u32,
}

impl TokenKind {
    fn is_decl_specifier(&self) -> bool {
        self.is_type_specifier() || matches!(self, TokenKind::Extern | TokenKind::Static)
    }

    fn is_type_specifier(&self) -> bool {
        matches!(
            self,
            TokenKind::Double
                | TokenKind::Int
                | TokenKind::Long
                | TokenKind::Unsigned
                | TokenKind::Signed
        )
    }
}

enum Declarator {
    Identifier(Node<Identifier>),
    Pointer(Node<Declarator>),
    Function {
        params: Vec<Param>,
        declarator: Node<Declarator>,
    },
}

struct Param {
    ty: Node<Type>,
    declarator: Node<Declarator>,
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
        let block = if self.current.kind.is_decl_specifier() {
            BlockItem::Decl(self.declaration()?)
        } else {
            BlockItem::Stmt(self.statement()?)
        };
        Ok(block)
    }

    fn declaration(&mut self) -> Result<Node<Declaration>> {
        let begin = self.current.span;
        let (ty, storage_class) = self.type_and_storage()?;
        let declarator = self.parse_declarator()?;
        let (name, ty, params) = Self::process_declarator(declarator, ty)?;

        if let Type::Function(function_ty) = *ty.data {
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

    fn parse_declarator(&mut self) -> Result<Node<Declarator>> {
        let begin = self.current.span;
        if self.matches(TokenKind::Star) {
            let referenced = self.parse_declarator()?;
            Ok(self.node(begin + referenced.span, Declarator::Pointer(referenced)))
        } else {
            self.parse_direct_declarator()
        }
    }

    fn parse_simple_declarator(&mut self) -> Result<Node<Declarator>> {
        let begin = self.current.span;
        if self.matches(TokenKind::OpenParen) {
            let declarator = self.parse_declarator()?;
            let end = self.expect(TokenKind::CloseParen)?.span;
            Ok(self.node(begin + end, *declarator.data))
        } else {
            let identifier = self.identifier()?;
            Ok(self.node(identifier.span, Declarator::Identifier(identifier)))
        }
    }

    fn parse_direct_declarator(&mut self) -> Result<Node<Declarator>> {
        let begin = self.current.span;
        let simple = self.parse_simple_declarator()?;
        let mut end = simple.span;
        if self.matches(TokenKind::OpenParen) {
            let mut params = Vec::new();
            if self.matches(TokenKind::Void) {
                self.expect(TokenKind::CloseParen)?;
            } else {
                loop {
                    let param = self.parse_param()?;
                    params.push(param);
                    end = self.current.span;
                    if self.matches(TokenKind::Comma) {
                        continue;
                    }
                    self.expect(TokenKind::CloseParen)?;
                    break;
                }
            }
            Ok(self.node(
                begin + end,
                Declarator::Function {
                    params,
                    declarator: simple,
                },
            ))
        } else {
            Ok(simple)
        }
    }

    fn parse_param(&mut self) -> Result<Param> {
        let ty = self.type_specifier()?;
        let declarator = self.parse_declarator()?;
        Ok(Param { ty, declarator })
    }

    fn process_declarator(
        declarator: Node<Declarator>,
        base_type: Node<Type>,
    ) -> Result<(Node<Identifier>, Node<Type>, Vec<Node<Identifier>>)> {
        match *declarator.data {
            Declarator::Identifier(name) => Ok((name, base_type, vec![])),
            Declarator::Pointer(declarator) => {
                let derived_type = Node {
                    id: base_type.id,
                    span: base_type.span,
                    data: Box::new(Type::Pointer(base_type.data)),
                };
                Self::process_declarator(declarator, derived_type)
            }
            Declarator::Function { params, declarator } => {
                let mut param_names = Vec::new();
                let mut param_types = Vec::new();
                if let Declarator::Identifier(name) = *declarator.data {
                    for Param { ty, declarator } in params {
                        let (param_name, param_type, _) = Self::process_declarator(declarator, ty)?;
                        if let Type::Function(_) = param_type.as_ref() {
                            return Err(CompilerError {
                                kind: ErrorKind::Parse,
                                msg: "Function pointers in parameters are not supported".into(),
                                span: param_type.span,
                            });
                        }
                        param_names.push(param_name);
                        param_types.push(*param_type.data);
                    }
                    let function_type = FunctionType {
                        params: param_types,
                        ret: base_type.data,
                    };
                    let derived_type = Node {
                        id: base_type.id,
                        span: base_type.span,
                        data: Box::new(Type::Function(function_type)),
                    };
                    Ok((name, derived_type, param_names))
                } else {
                    Err(CompilerError {
                        kind: ErrorKind::Parse,
                        msg: "Can't apply additional derivations to a function type".into(),
                        span: declarator.span,
                    })
                }
            }
        }
    }

    fn type_specifier(&mut self) -> Result<Node<Type>> {
        let mut types = Vec::new();
        let begin = self.current.span;
        let mut end = self.current.span;
        while self.current.kind.is_type_specifier() {
            types.push(self.current.kind);
            end = self.current.span;
            self.advance()
        }
        self.type_from_list(begin + end, &types)
    }

    fn type_from_list(&mut self, span: Span, types: &[TokenKind]) -> Result<Node<Type>> {
        if let [TokenKind::Double] = types {
            return Ok(self.node(span, Type::Double));
        }

        let mut signed = 0;
        let mut unsigned = 0;
        let mut int = 0;
        let mut long = 0;

        for ty in types {
            match ty {
                TokenKind::Signed => signed += 1,
                TokenKind::Unsigned => unsigned += 1,
                TokenKind::Int => int += 1,
                TokenKind::Long => long += 1,
                TokenKind::Double => {
                    return Err(CompilerError {
                        kind: ErrorKind::Parse,
                        msg: "Invalid type specifier".into(),
                        span,
                    })
                }
                _ => unreachable!(),
            };
        }

        let ty = match (signed, unsigned, int, long) {
            (0, 1, 0 | 1, 1) => Type::ULong,
            (0, 1, 0 | 1, 0) => Type::UInt,
            (0 | 1, 0, 0 | 1, 1) => Type::Long,
            (0 | 1, 0, 1, 0) => Type::Int,
            (1, 0, 0, 0) => Type::Int,
            (0, 0, 0, 0) => {
                return Err(CompilerError {
                    kind: ErrorKind::Parse,
                    msg: "Expected type specifier".into(),
                    span,
                })
            }
            _ => {
                return Err(CompilerError {
                    kind: ErrorKind::Parse,
                    msg: "Invalid type specifier".into(),
                    span,
                })
            }
        };

        Ok(self.node(span, ty))
    }

    fn type_and_storage(&mut self) -> Result<(Node<Type>, Option<Node<StorageClass>>)> {
        let begin = self.current.span;
        let mut end = self.current.span;
        let mut types = Vec::new();
        let mut storage: Option<Node<StorageClass>> = None;
        loop {
            let token = self.current;
            match token.kind {
                kind if kind.is_type_specifier() => {
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
        let ty = self.type_from_list(begin + end, &types)?;
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
            // Checkin if declaration contains storage class specifier (static, extern) is done
            // during type checking for better error messages.
            _ if self.current.kind.is_decl_specifier() => {
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
                label: "dummy".into(),
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
            TokenKind::IntConstant(_) => self.int_constant()?,
            TokenKind::DoubleConstant => self.double_constant()?,
            TokenKind::Identifier => {
                if self.next.kind == TokenKind::OpenParen {
                    self.function_call()?
                } else {
                    self.var()?
                }
            }
            TokenKind::Star => self.dereference()?,
            TokenKind::Ampersand => self.address_of()?,
            TokenKind::Minus
            | TokenKind::Tilde
            | TokenKind::Bang
            | TokenKind::PlusPlus
            | TokenKind::MinusMinus => self.unary_expression()?,
            TokenKind::OpenParen => {
                let begin = self.current.span;
                self.advance();
                if self.current.kind.is_type_specifier() {
                    self.cast_expression(begin)?
                } else {
                    self.paren_expression(begin)?
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

    fn cast_expression(&mut self, begin: Span) -> Result<Node<Expression>> {
        let target = self.type_specifier()?;
        self.expect(TokenKind::CloseParen)?;
        let expr = self.expression_precedence(13, "expression")?;
        Ok(self.node(begin + expr.span, Expression::Cast { target, expr }))
    }

    fn paren_expression(&mut self, begin: Span) -> Result<Node<Expression>> {
        let inner = self.expression()?;
        let end = self.expect(TokenKind::CloseParen)?.span;
        Ok(self.node(begin + end, *inner.data))
    }

    fn dereference(&mut self) -> Result<Node<Expression>> {
        let op = self.expect(TokenKind::Star)?;
        let expr = self.expression_precedence(13, "expression")?;
        Ok(self.node(op.span + expr.span, Expression::Dereference(expr)))
    }

    fn address_of(&mut self) -> Result<Node<Expression>> {
        let op = self.expect(TokenKind::Ampersand)?;
        let expr = self.expression_precedence(13, "expression")?;
        Ok(self.node(op.span + expr.span, Expression::AddressOf(expr)))
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

    fn double_constant(&mut self) -> Result<Node<Expression>> {
        let token = self.expect(TokenKind::DoubleConstant)?;
        let lexeme = token.slice(self.source);
        let value: f64 = lexeme.parse().map_err(|e| CompilerError {
            kind: ErrorKind::Parse,
            msg: format!("Integer constant out of range: {e:?}"),
            span: token.span,
        })?;
        Ok(self.node(token.span, Expression::Constant(Constant::Double(value))))
    }

    fn int_constant(&mut self) -> Result<Node<Expression>> {
        let token = self.current;
        let TokenKind::IntConstant(kind) = token.kind else {
            return Err(CompilerError {
                kind: ErrorKind::Parse,
                msg: format!(
                    "Expected constant but found '{}'",
                    self.current.slice(self.source)
                ),
                span: token.span,
            });
        };
        self.advance();
        let lexeme = token.slice(self.source);
        let string_value = lexeme.strip_suffix(['l', 'L']).unwrap_or(lexeme);
        let string_value = string_value
            .strip_suffix(['u', 'U'])
            .unwrap_or(string_value);

        // Constant tokens don't have a sign, so u64 is the correct type
        let value: u64 = string_value.parse().map_err(|e| CompilerError {
            kind: ErrorKind::Parse,
            msg: format!("Integer constant out of range: {e:?}"),
            span: token.span,
        })?;

        let constant = match kind {
            IntKind::Int if value > i32::MAX as u64 => Constant::Long(value as i64),
            IntKind::Int => Constant::Int(value as i32),
            IntKind::Uint if value > u32::MAX as u64 => Constant::ULong(value),
            IntKind::Uint => Constant::UInt(value as u32),
            IntKind::Long => Constant::Long(value as i64),
            IntKind::ULong => Constant::ULong(value),
        };

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
