use crate::lexer::Token;

#[derive(Debug)]
pub enum Expression<'a> {
    Ident(&'a str),
    Int(u64),
    Bool(bool),
    IfExpression(Box<Expression<'a>>, Vec<Statement<'a>>, Option<Vec<Statement<'a>>>),
    FunctionLiteral(Option<Vec<&'a str>>, Vec<Statement<'a>>),
    CallExpression(Box<Expression<'a>>, Option<Vec<Expression<'a>>>),
    PrefixExpression(&'a Token<'a>, Box<Expression<'a>>),
    InfixExpression(Box<Expression<'a>>, &'a Token<'a>, Box<Expression<'a>>)
}

#[derive(Debug)]
pub enum Statement<'a> {
    LetStatement(&'a str, Expression<'a>),
    ReturnStatement(Expression<'a>),
    ExpressionStatement(Expression<'a>)
}

#[derive(Debug)]
pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>
}