use crate::ast::{Node, Program};
use crate::error::Result;

mod id_resolution;
mod label_check;
mod type_check;

#[cfg(test)]
mod test;

pub fn validate(ast: Node<Program>) -> Result<Node<Program>> {
    let ast = id_resolution::check(ast)?;
    let ast = label_check::check(ast)?;
    let ast = type_check::check(ast)?;
    Ok(ast)
}
