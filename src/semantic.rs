use crate::ast::{Constant, Expression, FunctionTypeSpec, Node, NodeId, Program, TypeSpec};
use crate::error::Result;
use crate::symbol::Symbol;
use crate::tacky;
use std::collections::{BTreeMap, HashMap};

mod id_resolution;
mod label_check;
mod type_check;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    Char,
    SChar,
    UChar,
    Int,
    UInt,
    Long,
    ULong,
    Double,
    Function(FunctionType),
    Pointer(Box<Type>),
    Array(Box<Type>, usize),
    Struct(Symbol),
    Union(Symbol),
    Void,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionType {
    pub params: Vec<Type>,
    pub ret: Box<Type>,
}

#[derive(Debug, Clone)]
pub struct AggregateType {
    pub kind: AggregateKind,
    pub alignment: u8,
    pub size: usize,
    pub fields: Vec<Field>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AggregateKind {
    Struct,
    Union,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: Symbol,
    pub ty: Type,
    pub offset: usize,
}

impl Type {
    pub fn is_char(&self) -> bool {
        matches!(self, Type::Char | Type::SChar | Type::UChar)
    }

    pub fn is_int(&self) -> bool {
        matches!(
            self,
            Type::Char
                | Type::SChar
                | Type::UChar
                | Type::Int
                | Type::UInt
                | Type::Long
                | Type::ULong
        )
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Type::Double)
    }

    pub fn is_void(&self) -> bool {
        matches!(self, Type::Void)
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Type::Function(_))
    }

    pub fn is_aggregate(&self) -> bool {
        matches!(self, Type::Struct(_) | Type::Union(_))
    }

    pub fn is_incomplete_aggregate(&self, semantics: &SemanticData) -> bool {
        self.is_aggregate() && !self.is_complete(semantics)
    }

    pub fn is_arithmetic(&self) -> bool {
        matches!(
            self,
            Type::Char
                | Type::SChar
                | Type::UChar
                | Type::Int
                | Type::UInt
                | Type::Long
                | Type::ULong
                | Type::Double
        )
    }

    pub fn is_scalar(&self) -> bool {
        matches!(
            self,
            Type::Char
                | Type::SChar
                | Type::UChar
                | Type::Int
                | Type::UInt
                | Type::Long
                | Type::ULong
                | Type::Double
                | Type::Pointer(_)
        )
    }

    pub fn is_complete(&self, semantics: &SemanticData) -> bool {
        match self {
            Type::Void => false,
            Type::Struct(name) | Type::Union(name) => {
                matches!(
                    semantics.type_table.type_defs.get(name),
                    Some(TypeEntry::Complete(_))
                )
            }
            _ => true,
        }
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, Type::Pointer(_))
    }

    pub fn is_pointer_to_void(&self) -> bool {
        matches!(self, Type::Pointer(inner) if inner.is_void())
    }

    pub fn is_pointer_to_incomplete(&self, semantics: &SemanticData) -> bool {
        matches!(self, Type::Pointer(inner) if !inner.is_complete(semantics))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Type::Array(_, _))
    }

    pub fn is_signed(&self) -> bool {
        match self {
            Type::Int | Type::Long | Type::Char | Type::SChar => true,
            Type::UInt | Type::ULong | Type::Double | Type::Pointer(_) | Type::UChar => false,
            _ => panic!("{self:?} does not have a sign"),
        }
    }
}

impl TypeSpec {
    pub fn ty(&self) -> Type {
        match self {
            TypeSpec::Char => Type::Char,
            TypeSpec::SChar => Type::SChar,
            TypeSpec::UChar => Type::UChar,
            TypeSpec::Int => Type::Int,
            TypeSpec::UInt => Type::UInt,
            TypeSpec::Long => Type::Long,
            TypeSpec::ULong => Type::ULong,
            TypeSpec::Double => Type::Double,
            TypeSpec::Function(ty) => Type::Function(ty.ty()),
            TypeSpec::Pointer(ty) => Type::Pointer(ty.ty().into()),
            TypeSpec::Array(ty, size) => Type::Array(ty.ty().into(), *size),
            TypeSpec::Struct(tag) => Type::Struct(tag.symbol.clone()),
            TypeSpec::Union(tag) => Type::Union(tag.symbol.clone()),
            TypeSpec::Void => Type::Void,
        }
    }
}

impl FunctionTypeSpec {
    fn ty(&self) -> FunctionType {
        FunctionType {
            params: self.params.iter().map(|t| t.ty()).collect(),
            ret: self.ret.ty().into(),
        }
    }
}

impl Constant {
    pub fn from_char(c: char, ty: &Type) -> Constant {
        match ty {
            Type::Char | Type::SChar => Constant::Char(c as i8),
            Type::UChar => Constant::UChar(c as u8),
            _ => panic!("Cannot convert char to {ty:?}"),
        }
    }

    pub fn ty(&self) -> Type {
        match self {
            Constant::Char(_) => Type::Char,
            Constant::UChar(_) => Type::UChar,
            Constant::Int(_) => Type::Int,
            Constant::UInt(_) => Type::UInt,
            Constant::Long(_) => Type::Long,
            Constant::ULong(_) => Type::ULong,
            Constant::Double(_) => Type::Double,
        }
    }

    pub fn cast(&self, target: &Type) -> Option<Constant> {
        let c = match (self, target) {
            (c, Type::Char | Type::SChar) if c.is_int() => Constant::Char(c.as_u64() as i8),
            (c, Type::UChar) if c.is_int() => Constant::UChar(c.as_u64() as u8),
            (c, Type::Int) if c.is_int() => Constant::Int(c.as_u64() as i32),
            (c, Type::UInt) if c.is_int() => Constant::UInt(c.as_u64() as u32),
            (c, Type::Long) if c.is_int() => Constant::Long(c.as_u64() as i64),
            (c, Type::ULong) if c.is_int() => Constant::ULong(c.as_u64()),
            (c, Type::Double) if c.is_int() => Constant::Double(c.as_u64() as f64),
            (Constant::Double(value), Type::Int) => Constant::Int(*value as i32),
            (Constant::Double(value), Type::Char | Type::SChar) => Constant::Char(*value as i8),
            (Constant::Double(value), Type::UChar) => Constant::UChar(*value as u8),
            (Constant::Double(value), Type::UInt) => Constant::UInt(*value as u32),
            (Constant::Double(value), Type::Long) => Constant::Long(*value as i64),
            (Constant::Double(value), Type::ULong) => Constant::ULong(*value as u64),
            (Constant::Double(value), Type::Double) => Constant::Double(*value),
            (
                Constant::Int(0) | Constant::Long(0) | Constant::UInt(0) | Constant::ULong(0),
                Type::Pointer(_),
            ) => Constant::ULong(0),
            _ => return None,
        };
        Some(c)
    }

    pub fn to_static_init(&self) -> StaticInit {
        match self {
            Constant::Int(v) => StaticInit::Int(*v),
            Constant::UInt(v) => StaticInit::UInt(*v),
            Constant::Long(v) => StaticInit::Long(*v),
            Constant::ULong(v) => StaticInit::ULong(*v),
            Constant::Double(v) => StaticInit::Double(*v),
            Constant::Char(v) => StaticInit::Char(*v),
            Constant::UChar(v) => StaticInit::UChar(*v),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TypeEntry {
    Incomplete(AggregateKind),
    Complete(AggregateType),
}

// TODO: Move TypeTable to SemanticData
#[derive(Debug, Clone, Default)]
pub struct TypeTable {
    pub type_defs: HashMap<Symbol, TypeEntry>,
}

#[derive(Debug, Clone, Default)]
pub struct SemanticData {
    pub symbols: BTreeMap<Symbol, SymbolData>,
    pub strings: HashMap<Symbol, Symbol>,
    pub expression_types: HashMap<NodeId, Type>,
    pub assignment_common_types: HashMap<NodeId, Type>,
    pub type_table: TypeTable,
    pub implicit_casts: HashMap<NodeId, Type>,
    pub pointer_decays: HashMap<NodeId, Type>,
    pub switch_cases: HashMap<NodeId, SwitchCases>,
}

#[derive(Debug, Clone)]
pub struct SymbolData {
    pub ty: Type,
    pub attrs: Attributes,
}

#[derive(Debug, Clone)]
pub enum Attributes {
    Function {
        defined: bool,
        global: bool,
    },
    Static {
        initial_value: InitialValue,
        global: bool,
    },
    Const {
        init: StaticInit,
    },
    Local,
}

#[derive(Clone, Debug)]
pub enum InitialValue {
    Tentative,
    Initial(Vec<StaticInit>),
    NoInitializer,
}

impl InitialValue {
    pub fn single(init: StaticInit) -> InitialValue {
        InitialValue::Initial(vec![init])
    }
}

#[derive(Clone, Debug)]
pub enum StaticInit {
    Char(i8),
    UChar(u8),
    Int(i32),
    UInt(u32),
    Long(i64),
    ULong(u64),
    Double(f64),
    ZeroInit(usize),
    String {
        symbol: Symbol,
        null_terminated: bool,
    },
    Pointer(Symbol),
}

#[derive(Debug, Clone)]
pub struct SwitchCases {
    pub expr_ty: Type,
    pub values: Vec<(Constant, Symbol)>,
    pub default: Option<Symbol>,
}

impl SemanticData {
    pub fn expr_type(&self, expr: &Node<Expression>) -> &Type {
        self.expression_types
            .get(&expr.id)
            .expect("Expression without type")
    }

    pub fn assignment_common_type(&self, expr: &Node<Expression>) -> &Type {
        self.assignment_common_types
            .get(&expr.id)
            .expect("Expression without type")
    }

    pub fn get_aggregate(&self, name: &Symbol) -> &AggregateType {
        let Some(TypeEntry::Complete(type_def)) = self.type_table.type_defs.get(name) else {
            panic!("Type {name} is unknown or incomplete",);
        };
        type_def
    }

    pub fn switch_cases(&self, expr: &Node<Expression>) -> &SwitchCases {
        self.switch_cases
            .get(&expr.id)
            .expect("Switch without cases")
    }

    pub fn make_string(&mut self, s: &Symbol) -> Symbol {
        let existing_constant = self.strings.get(s);
        match existing_constant {
            Some(name) => name.clone(),
            None => {
                let name = format!("string.{}", self.strings.len());
                self.strings.insert(s.clone(), name.clone());
                self.symbols.insert(
                    name.clone(),
                    SymbolData {
                        ty: Type::Array(Type::Char.into(), s.len() + 1),
                        attrs: Attributes::Const {
                            init: StaticInit::String {
                                symbol: s.clone(),
                                null_terminated: true,
                            },
                        },
                    },
                );
                name
            }
        }
    }

    pub fn symbol_ty(&self, symbol: &Symbol) -> &Type {
        &self.symbols.get(symbol).expect("Symbol without type").ty
    }

    pub fn val_ty(&self, val: &tacky::Val) -> Type {
        match val {
            tacky::Val::Constant(c) => c.ty().clone(),
            tacky::Val::Var(name) => self.symbol_ty(name).clone(),
        }
    }

    pub fn is_signed(&self, val: &tacky::Val) -> bool {
        match val {
            tacky::Val::Constant(Constant::Int(_) | Constant::Long(_) | Constant::Char(_)) => true,
            tacky::Val::Constant(Constant::UInt(_) | Constant::ULong(_) | Constant::UChar(_)) => {
                false
            }
            tacky::Val::Constant(Constant::Double(_)) => true,
            tacky::Val::Var(name) => self.symbol_ty(name).is_signed(),
        }
    }

    pub fn common_type(&self, ty1: &Type, ty2: &Type) -> Type {
        // Char types are treated as ints
        let ty1 = if ty1.is_char() { &Type::Int } else { ty1 };
        let ty2 = if ty2.is_char() { &Type::Int } else { ty2 };

        let result = if ty1 == ty2 {
            ty1
        } else if ty1.is_double() || ty2.is_double() {
            &Type::Double
        } else if ty1.size(&self) == ty2.size(&self) {
            if ty1.is_signed() { ty2 } else { ty1 }
        } else if ty1.size(&self) > ty2.size(&self) {
            ty1
        } else {
            ty2
        };
        result.clone()
    }
}

pub fn validate(ast: Node<Program>) -> Result<(Node<Program>, SemanticData)> {
    let ast = id_resolution::check(ast)?;
    let ast = label_check::check(ast)?;
    let semantic_data = type_check::check(&ast)?;
    Ok((ast, semantic_data))
}
