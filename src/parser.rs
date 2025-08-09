#[cfg(test)]
mod test;

use crate::ast::{
    AssignOp, BinaryOp, Block, BlockItem, Constant, Declaration, Expression, Field, ForInit,
    FunctionDeclaration, FunctionTypeSpec, Identifier, Initializer, Node, PostfixOp, Program,
    Statement, StorageClass, StructDeclaration, TypeSpec, UnaryOp, VarDeclaration,
};
use crate::error::{CompilerError, ErrorKind, Result};
use crate::lexer::{IntKind, Lexer, Span, Token, TokenKind};
use crate::symbol::Symbol;

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
                | TokenKind::Char
                | TokenKind::Int
                | TokenKind::Long
                | TokenKind::Unsigned
                | TokenKind::Signed
                | TokenKind::Void
                | TokenKind::Struct
        )
    }
}

enum Declarator {
    Identifier(Node<Identifier>),
    Pointer(Node<Declarator>),
    Array {
        size: usize,
        declarator: Node<Declarator>,
    },
    Function {
        params: Vec<Param>,
        declarator: Node<Declarator>,
    },
}

struct ProcessedDeclarator {
    name: Node<Identifier>,
    type_spec: Node<TypeSpec>,
    param_names: Vec<Node<Identifier>>,
}

struct Param {
    ty: Node<TypeSpec>,
    declarator: Node<Declarator>,
}

enum AbstractDeclarator {
    Pointer(Node<AbstractDeclarator>),
    Array(Node<AbstractDeclarator>, usize),
    Base,
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
        let (ty, storage_class) = if self.matches(TokenKind::Struct) {
            let name = self.identifier()?;
            if let TokenKind::Semicolon | TokenKind::OpenBrace = self.current.kind {
                let mut fields = Vec::new();
                if self.matches(TokenKind::OpenBrace) {
                    if self.matches(TokenKind::CloseBrace) {
                        return Err(CompilerError {
                            kind: ErrorKind::Parse,
                            msg: "Expected struct field but found '}}'".into(),
                            span: self.current.span,
                        });
                    }
                    while !self.matches(TokenKind::CloseBrace) {
                        let field = self.field()?;
                        fields.push(field);
                    }
                }
                let end = self.expect(TokenKind::Semicolon)?.span;
                return Ok(self.node(
                    begin + end,
                    Declaration::Struct(StructDeclaration { name, fields }),
                ));
            } else {
                (self.node(begin + name.span, TypeSpec::Struct(name)), None)
            }
        } else {
            self.type_and_storage()?
        };
        let declarator = self.parse_declarator()?;
        let processed = self.process_declarator(declarator, ty)?;

        if let TypeSpec::Function(function_ty) = *processed.type_spec.data {
            if let TokenKind::OpenBrace = self.current.kind {
                let body = self.block()?;
                Ok(self.node(
                    begin + body.span,
                    Declaration::Function(FunctionDeclaration {
                        name: processed.name,
                        params: processed.param_names,
                        body: Some(body),
                        type_spec: function_ty,
                        storage_class,
                    }),
                ))
            } else {
                let end = self.expect(TokenKind::Semicolon)?.span;
                Ok(self.node(
                    begin + end,
                    Declaration::Function(FunctionDeclaration {
                        name: processed.name,
                        params: processed.param_names,
                        body: None,
                        type_spec: function_ty,
                        storage_class,
                    }),
                ))
            }
        } else {
            let init = if self.matches(TokenKind::Equal) {
                Some(self.initializer()?)
            } else {
                None
            };
            let end = self.expect(TokenKind::Semicolon)?.span;
            Ok(self.node(
                begin + end,
                Declaration::Var(VarDeclaration {
                    name: processed.name,
                    init,
                    type_spec: processed.type_spec,
                    storage_class,
                }),
            ))
        }
    }

    fn field(&mut self) -> Result<Node<Field>> {
        let base_ty = self.type_specifier()?;
        let begin = base_ty.span;
        let declarator = self.parse_declarator()?;
        if let Declarator::Function { .. } = declarator.as_ref() {
            return Err(CompilerError {
                kind: ErrorKind::Parse,
                msg: "Structs can't have fields".into(),
                span: declarator.span,
            });
        }
        let ProcessedDeclarator {
            name, type_spec, ..
        } = self.process_declarator(declarator, base_ty)?;
        let end = self.expect(TokenKind::Semicolon)?.span;
        Ok(self.node(begin + end, Field { name, type_spec }))
    }

    fn initializer(&mut self) -> Result<Node<Initializer>> {
        let begin = self.current.span;
        if self.matches(TokenKind::OpenBrace) {
            let mut end;
            let mut initializers = Vec::new();
            loop {
                let inner = self.initializer()?;
                end = inner.span;
                initializers.push(inner);
                if self.matches(TokenKind::Comma) {
                    if self.matches(TokenKind::CloseBrace) {
                        break;
                    } else {
                        continue;
                    }
                }
                end = self.expect(TokenKind::CloseBrace)?.span;
                break;
            }
            Ok(self.node(begin + end, Initializer::Compound(initializers)))
        } else {
            let expr = self.expression()?;
            Ok(self.node(expr.span, Initializer::Single(expr)))
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
            if self.current.kind == TokenKind::Void && self.next.kind == TokenKind::CloseParen {
                // empty argument list foo(void)
                self.advance(); // consume void
                self.advance(); // consume close paren
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
        } else if self.matches(TokenKind::OpenBracket) {
            let mut declarator = simple;
            loop {
                let size = self.parse_array_index()?;
                end = self.current.span;
                self.expect(TokenKind::CloseBracket)?;
                declarator = self.node(begin + end, Declarator::Array { size, declarator });
                if !self.matches(TokenKind::OpenBracket) {
                    break;
                }
            }
            Ok(declarator)
        } else {
            Ok(simple)
        }
    }

    fn parse_array_index(&mut self) -> Result<usize> {
        let size_expr = self.expression()?;
        let Expression::Constant(size_const) = size_expr.as_ref() else {
            return Err(CompilerError {
                kind: ErrorKind::Parse,
                msg: "Array size should be a constant".into(),
                span: size_expr.span,
            });
        };
        if !size_const.is_int() {
            return Err(CompilerError {
                kind: ErrorKind::Parse,
                msg: "Array size should be an integer constant".into(),
                span: size_expr.span,
            });
        }
        Ok(size_const.as_u64() as usize)
    }

    fn parse_param(&mut self) -> Result<Param> {
        let ty = self.type_specifier()?;
        let declarator = self.parse_declarator()?;
        Ok(Param { ty, declarator })
    }

    fn process_declarator(
        &mut self,
        declarator: Node<Declarator>,
        type_spec: Node<TypeSpec>,
    ) -> Result<ProcessedDeclarator> {
        match *declarator.data {
            Declarator::Identifier(name) => Ok(ProcessedDeclarator {
                name,
                type_spec,
                param_names: vec![],
            }),
            Declarator::Pointer(declarator) => {
                let derived_type = self.node(
                    type_spec.span + declarator.span,
                    TypeSpec::Pointer(type_spec),
                );
                self.process_declarator(declarator, derived_type)
            }
            Declarator::Array {
                size,
                declarator: inner,
            } => {
                let derived_type = self.node(
                    type_spec.span + declarator.span,
                    TypeSpec::Array(type_spec, size),
                );
                self.process_declarator(inner, derived_type)
            }
            Declarator::Function { params, declarator } => {
                let mut param_names = Vec::new();
                let mut param_types = Vec::new();
                if let Declarator::Identifier(name) = *declarator.data {
                    for Param { ty, declarator } in params {
                        let processed = self.process_declarator(declarator, ty)?;
                        if let TypeSpec::Function { .. } = processed.type_spec.as_ref() {
                            return Err(CompilerError {
                                kind: ErrorKind::Parse,
                                msg: "Function pointers in parameters are not supported".into(),
                                span: processed.type_spec.span,
                            });
                        }
                        param_names.push(processed.name);
                        param_types.push(processed.type_spec);
                    }
                    let span = type_spec.span;
                    let function_type = FunctionTypeSpec {
                        params: param_types,
                        ret: type_spec,
                    };
                    let type_spec =
                        self.node(span + declarator.span, TypeSpec::Function(function_type));
                    Ok(ProcessedDeclarator {
                        name,
                        type_spec,
                        param_names,
                    })
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

    fn type_specifier(&mut self) -> Result<Node<TypeSpec>> {
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

    fn type_from_list(&mut self, span: Span, types: &[TokenKind]) -> Result<Node<TypeSpec>> {
        match types {
            [TokenKind::Struct] => {
                let name = self.identifier()?;
                return Ok(self.node(span, TypeSpec::Struct(name)));
            }
            [TokenKind::Double] => {
                return Ok(self.node(span, TypeSpec::Double));
            }
            [TokenKind::Void] => {
                return Ok(self.node(span, TypeSpec::Void));
            }
            _ => {}
        }

        let mut char = 0;
        let mut signed = 0;
        let mut unsigned = 0;
        let mut int = 0;
        let mut long = 0;

        for ty in types {
            match ty {
                TokenKind::Char => char += 1,
                TokenKind::Signed => signed += 1,
                TokenKind::Unsigned => unsigned += 1,
                TokenKind::Int => int += 1,
                TokenKind::Long => long += 1,
                TokenKind::Double | TokenKind::Void => {
                    return Err(CompilerError {
                        kind: ErrorKind::Parse,
                        msg: "Invalid type specifier".into(),
                        span,
                    });
                }
                _ => unreachable!(),
            };
        }

        let ty = match (signed, unsigned, char, int, long) {
            (0, 0, 1, 0, 0) => TypeSpec::Char,
            (1, 0, 1, 0, 0) => TypeSpec::SChar,
            (0, 1, 1, 0, 0) => TypeSpec::UChar,
            (0, 1, 0, 0 | 1, 1) => TypeSpec::ULong,
            (0, 1, 0, 0 | 1, 0) => TypeSpec::UInt,
            (0 | 1, 0, 0, 0 | 1, 1) => TypeSpec::Long,
            (0 | 1, 0, 0, 1, 0) => TypeSpec::Int,
            (1, 0, 0, 0, 0) => TypeSpec::Int,
            (0, 0, 0, 0, 0) => {
                return Err(CompilerError {
                    kind: ErrorKind::Parse,
                    msg: "Expected type specifier".into(),
                    span,
                });
            }
            _ => {
                return Err(CompilerError {
                    kind: ErrorKind::Parse,
                    msg: "Invalid type specifier".into(),
                    span,
                });
            }
        };

        Ok(self.node(span, ty))
    }

    fn type_and_storage(&mut self) -> Result<(Node<TypeSpec>, Option<Node<StorageClass>>)> {
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
        if self.matches(TokenKind::Semicolon) {
            let end = self.current.span;
            Ok(self.node(begin + end, Statement::Return(None)))
        } else {
            let expr = self.expression()?;
            let end = self.expect(TokenKind::Semicolon)?.span;
            Ok(self.node(begin + end, Statement::Return(Some(expr))))
        }
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
            TokenKind::Sizeof => self.sizeof()?,
            TokenKind::StringLiteral => self.string_literal()?,
            TokenKind::IntConstant(_) => self.int_constant()?,
            TokenKind::DoubleConstant => self.double_constant()?,
            TokenKind::CharLiteral => self.char_literal()?,
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

            if self.matches(TokenKind::OpenBracket) {
                let precedence = 14;
                if precedence < min_precedence {
                    break;
                }
                let index = self.expression()?;
                let end = self.expect(TokenKind::CloseBracket)?.span;
                expr = self.node(expr.span + end, Expression::Subscript(expr, index));
                continue;
            }

            if self.matches(TokenKind::Dot) {
                let precedence = 14;
                if precedence < min_precedence {
                    break;
                }
                let member = self.identifier()?;
                expr = self.node(
                    expr.span + member.span,
                    Expression::Dot {
                        structure: expr,
                        member,
                    },
                );
                continue;
            }

            if self.matches(TokenKind::Arrow) {
                let precedence = 14;
                if precedence < min_precedence {
                    break;
                }
                let member = self.identifier()?;
                expr = self.node(
                    expr.span + member.span,
                    Expression::Arrow {
                        pointer: expr,
                        member,
                    },
                );
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
        let base_ty = self.type_specifier()?;
        let declarator = self.abstract_declarator()?;
        self.expect(TokenKind::CloseParen)?;
        let target = Self::process_abstract_declaration(base_ty, declarator);
        let expr = self.expression_precedence(13, "expression")?;
        Ok(self.node(begin + expr.span, Expression::Cast { target, expr }))
    }

    fn abstract_declarator(&mut self) -> Result<Node<AbstractDeclarator>> {
        let begin = self.current.span;
        if self.matches(TokenKind::Star) {
            let declarator = self.abstract_declarator()?;
            Ok(self.node(
                begin + declarator.span,
                AbstractDeclarator::Pointer(declarator),
            ))
        } else if let TokenKind::OpenParen | TokenKind::OpenBracket = self.current.kind {
            self.direct_abstract_declarator()
        } else {
            Ok(self.node(begin, AbstractDeclarator::Base))
        }
    }

    fn direct_abstract_declarator(&mut self) -> Result<Node<AbstractDeclarator>> {
        // TODO: deduplicate
        if self.current.kind == TokenKind::OpenParen {
            let begin = self.expect(TokenKind::OpenParen)?.span;
            let mut decl = self.abstract_declarator()?;
            let mut end = self.expect(TokenKind::CloseParen)?.span;
            while self.matches(TokenKind::OpenBracket) {
                let size = self.parse_array_index()?;
                end = self.expect(TokenKind::CloseBracket)?.span;
                decl = self.node(begin + end, AbstractDeclarator::Array(decl, size));
            }
            Ok(self.node(begin + end, *decl.data))
        } else {
            let begin = self.current.span;
            let mut end = begin;
            let mut decl = self.node(begin, AbstractDeclarator::Base);
            while self.matches(TokenKind::OpenBracket) {
                let size = self.parse_array_index()?;
                end = self.expect(TokenKind::CloseBracket)?.span;
                decl = self.node(begin + end, AbstractDeclarator::Array(decl, size));
            }
            Ok(self.node(begin + end, *decl.data))
        }
    }

    fn process_abstract_declaration(
        base_ty: Node<TypeSpec>,
        declarator: Node<AbstractDeclarator>,
    ) -> Node<TypeSpec> {
        match *declarator.data {
            AbstractDeclarator::Pointer(inner) => {
                let derived_type = Node {
                    id: declarator.id,
                    span: declarator.span,
                    data: Box::new(TypeSpec::Pointer(base_ty)),
                };
                Self::process_abstract_declaration(derived_type, inner)
            }
            AbstractDeclarator::Array(inner, size) => {
                let derived_type = Node {
                    id: declarator.id,
                    span: declarator.span,
                    data: Box::new(TypeSpec::Array(base_ty, size)),
                };
                Self::process_abstract_declaration(derived_type, inner)
            }
            AbstractDeclarator::Base => base_ty,
        }
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

    fn sizeof(&mut self) -> Result<Node<Expression>> {
        let begin = self.current.span;
        self.expect(TokenKind::Sizeof)?;
        if self.current.kind == TokenKind::OpenParen && self.next.kind.is_type_specifier() {
            self.advance(); // consume open paren
            let base_ty = self.type_specifier()?;
            let declarator = self.abstract_declarator()?;
            let target = Self::process_abstract_declaration(base_ty, declarator);
            let end = self.expect(TokenKind::CloseParen)?.span;
            Ok(self.node(begin + end, Expression::SizeOfType(target)))
        } else {
            let expr = self.expression_precedence(13, "expression")?;
            let end = expr.span;
            Ok(self.node(begin + end, Expression::SizeOfExpr(expr)))
        }
    }

    fn char_literal(&mut self) -> Result<Node<Expression>> {
        let token = self.expect(TokenKind::CharLiteral)?;
        let lexeme = token.slice(self.source);
        let trimmed = &lexeme[1..lexeme.len() - 1];
        let mut chars = trimmed.chars();
        let Some(c) = self.next_character(&mut chars)? else {
            return Err(CompilerError {
                kind: ErrorKind::Parse,
                msg: "Invalid escape sequence".to_string(),
                span: self.current.span,
            });
        };
        Ok(self.node(token.span, Expression::Constant(Constant::Int(c as i32))))
    }

    fn string_literal(&mut self) -> Result<Node<Expression>> {
        let mut value = String::new();
        let begin = self.current.span;
        let mut end = self.current.span;
        while let TokenKind::StringLiteral = self.current.kind {
            let lexeme = self.current.slice(self.source);
            let trimmed = &lexeme[1..lexeme.len() - 1];
            let mut chars = trimmed.chars();
            while let Some(c) = self.next_character(&mut chars)? {
                value.push(c);
            }
            end = self.current.span;
            self.advance();
        }
        Ok(self.node(begin + end, Expression::String(value)))
    }

    fn next_character(&self, chars: &mut impl Iterator<Item = char>) -> Result<Option<char>> {
        if let Some(c) = chars.next() {
            let c = if c == '\\' {
                match chars.next() {
                    Some('\'') => '\'',
                    Some('"') => '"',
                    Some('?') => '?',
                    Some('\\') => '\\',
                    Some('a') => '\x07',
                    Some('b') => '\x08',
                    Some('f') => '\x0C',
                    Some('n') => '\n',
                    Some('r') => '\r',
                    Some('t') => '\t',
                    Some('v') => '\x0B',
                    Some(escape) => {
                        return Err(CompilerError {
                            kind: ErrorKind::Parse,
                            msg: format!("Invalid escape sequence: '\\{escape}'"),
                            span: self.current.span,
                        });
                    }
                    _ => {
                        return Err(CompilerError {
                            kind: ErrorKind::Parse,
                            msg: "Invalid escape sequence".to_string(),
                            span: self.current.span,
                        });
                    }
                }
            } else {
                c
            };
            Ok(Some(c))
        } else {
            Ok(None)
        }
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
