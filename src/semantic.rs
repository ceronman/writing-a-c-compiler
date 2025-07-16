use crate::ast::{Constant, Expression, Node, NodeId, Program, Type};
use crate::error::Result;
use crate::symbol::Symbol;
use std::collections::{BTreeMap, HashMap};

mod id_resolution;
mod label_check;
mod type_check;

#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub struct SemanticData {
    pub symbols: BTreeMap<Symbol, SymbolData>,
    pub expression_types: HashMap<NodeId, Type>,
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
    Char(i32),
    UChar(i32),
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
