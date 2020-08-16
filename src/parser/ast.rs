use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Expression<'a> {
    Ident(&'a str),
    Int(i64),
    Bool(bool),
    IfExpression(Box<Expression<'a>>, Vec<Statement<'a>>, Option<Vec<Statement<'a>>>),
    FunctionLiteral(Option<Vec<&'a str>>, Vec<Statement<'a>>),
    CallExpression(Box<Expression<'a>>, Option<Vec<Expression<'a>>>),
    PrefixExpression(Prefix, Box<Expression<'a>>),
    InfixExpression(Box<Expression<'a>>, Infix, Box<Expression<'a>>)
}

#[derive(PartialEq, Clone)]
pub enum Prefix {
    PrefixMinus,
    Not
}

impl fmt::Display for Prefix {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Prefix::Not         => write!(f, "!"),
            Prefix::PrefixMinus => write!(f, "-")
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan
}


impl fmt::Display for Infix {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Infix::Plus        => write!(f, "+"),
            Infix::Minus       => write!(f, "-"),
            Infix::Multiply    => write!(f, "*"),
            Infix::Divide      => write!(f, "/"),
            Infix::LessThan    => write!(f, "<"),
            Infix::GreaterThan => write!(f, ">"),
            Infix::Equal       => write!(f, "=="),
            Infix::NotEqual    => write!(f, "!=")
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Statement<'a> {
    LetStatement(&'a str, Expression<'a>),
    ReturnStatement(Expression<'a>),
    ExpressionStatement(Expression<'a>)
}

pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>
}