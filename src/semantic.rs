use crate::ast::{Node, Program};
use crate::error::Result;

mod id_resolution;
mod label_check;
mod type_check;

#[cfg(test)]
mod test;

pub use type_check::SymbolTable;
pub use type_check::Attributes;
pub use type_check::InitialValue;

pub fn validate(ast: Node<Program>) -> Result<(Node<Program>, SymbolTable)> {
    let ast = id_resolution::check(ast)?;
    let ast = label_check::check(ast)?;
    let symbol_table = type_check::check(&ast)?;
    Ok((ast, symbol_table))
}
