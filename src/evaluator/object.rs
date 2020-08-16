use std::fmt;
use crate::parser::Statement;
use crate::evaluator::environment::{EnvRc};

#[derive(PartialEq, Clone)]
pub enum Object<'a> {
    Integer(i64),
    Boolean(bool),
    Function(Option<Vec<&'a str>>, Vec<Statement<'a>>, EnvRc<'a>),
    Return(Box<Object<'a>>),
    Null
}

impl<'a> fmt::Display for Object<'a> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(i)        => write!(f, "{}", i),
            Object::Boolean(i)        => write!(f, "{}", i),
            Object::Function(_, _, _) => write!(f, "fn"),
            Object::Return(i)         => write!(f, "{}", *i),
            Object::Null              => write!(f, "null")
        }
    }

}