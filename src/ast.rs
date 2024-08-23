pub type Symbol = String;

#[derive(Debug)]
pub struct Program {
    pub function_definition: Function,
}

#[derive(Debug)]
pub struct Function {
    pub name: Symbol,
    pub body: Statement,
}

#[derive(Debug)]
pub enum Statement {
    Return { expr: Expression },
}

#[derive(Debug)]
pub enum Expression {
    Constant(i64),
    Unary { op: UnaryOp, expr: Box<Expression> },
}

#[derive(Debug)]
pub enum UnaryOp {
    Complement,
    Negate,
}
