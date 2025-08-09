use crate::ast::{Constant, Expression, FunctionTypeSpec, Node, NodeId, Program, TypeSpec};
use crate::error::Result;
use crate::symbol::Symbol;
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
    Void,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionType {
    pub params: Vec<Type>,
    pub ret: Box<Type>,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub alignment: u8,
    pub size: usize,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone)]
pub struct StructField {
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

    pub fn is_struct(&self) -> bool {
        matches!(self, Type::Struct(_))
    }

    pub fn is_incomplete_struct(&self, type_table: &TypeTable) -> bool {
        self.is_struct() && !self.is_complete(type_table)
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

    pub fn is_complete(&self, type_table: &TypeTable) -> bool {
        match self {
            Type::Void => false,
            Type::Struct(name) => type_table.structs.contains_key(name),
            _ => true,
        }
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, Type::Pointer(_))
    }

    pub fn is_pointer_to_void(&self) -> bool {
        matches!(self, Type::Pointer(inner) if inner.is_void())
    }

    pub fn is_pointer_to_incomplete(&self, type_table: &TypeTable) -> bool {
        matches!(self, Type::Pointer(inner) if !inner.is_complete(&type_table))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Type::Array(_, _))
    }

    pub fn size(&self) -> usize {
        match self {
            Type::Char | Type::UChar | Type::SChar => 1,
            Type::Int => 4,
            Type::UInt => 4,
            Type::Long => 8,
            Type::ULong => 8,
            Type::Double => 8,
            Type::Function(_) => panic!("Size of a function type"),
            Type::Pointer(_) => 8,
            Type::Array(ty, size) => ty.size() * size,
            Type::Void => 1,
            Type::Struct(_) => todo!(),
        }
    }

    pub fn is_signed(&self) -> bool {
        match self {
            Type::Int | Type::Long | Type::Char | Type::SChar => true,
            Type::UInt | Type::ULong | Type::Double | Type::Pointer(_) | Type::UChar => false,
            Type::Function(_) => panic!("Function types don't have a sign"),
            Type::Array(_, _) => panic!("Arrays don't have a sign"),
            Type::Void => panic!("Void doesn't have a sign"),
            Type::Struct(_) => panic!("Struct doesn't have a sign"),
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
}

#[derive(Debug, Clone, Default)]
pub struct TypeTable {
    pub structs: HashMap<Symbol, StructDef>,
}

#[derive(Debug, Clone)]
pub struct SemanticData {
    pub symbols: BTreeMap<Symbol, SymbolData>,
    pub expression_types: HashMap<NodeId, Type>,
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
    pub fn switch_cases(&self, expr: &Node<Expression>) -> &SwitchCases {
        self.switch_cases
            .get(&expr.id)
            .expect("Switch without cases")
    }
}

pub fn validate(ast: Node<Program>) -> Result<(Node<Program>, SemanticData)> {
    let ast = id_resolution::check(ast)?;
    let ast = label_check::check(ast)?;
    let semantic_data = type_check::check(&ast)?;
    Ok((ast, semantic_data))
}
