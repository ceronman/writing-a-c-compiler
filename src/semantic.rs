use crate::ast::{Node, Program};
use crate::error::Result;

mod id_resolution;
mod label_check;
#[cfg(test)]
mod test;

pub fn validate(ast: Node<Program>) -> Result<Node<Program>> {
    let ast = id_resolution::resolve(ast)?;
    let ast = label_check::check(ast)?;
    Ok(ast)
}
