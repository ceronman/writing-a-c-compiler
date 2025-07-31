use crate::lexer::Span;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ErrorKind {
    Parse,
    Resolve,
    Type,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::Parse => write!(f, "Parse error"),
            ErrorKind::Resolve => write!(f, "Resolution error"),
            ErrorKind::Type => write!(f, "Type error"),
        }
    }
}

#[derive(Debug)]
pub struct CompilerError {
    pub kind: ErrorKind,
    pub msg: String,
    pub span: Span,
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{kind}: {msg} at {span:?}",
            kind = self.kind,
            msg = self.msg,
            span = self.span
        )
    }
}

impl std::error::Error for CompilerError {}

pub type Result<T> = std::result::Result<T, CompilerError>;
