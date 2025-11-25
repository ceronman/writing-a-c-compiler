use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Symbol(Rc<str>);

impl Symbol {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        Symbol(Rc::from(value))
    }
}

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        Symbol(Rc::from(value))
    }
}
