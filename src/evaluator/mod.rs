mod object;
pub mod environment;

use object::Object;
use environment::{EnvRc};
use crate::parser::{Program, Statement, Expression, Prefix, Infix};
use std::rc::Rc;

pub fn run_program<'a>(program: Program<'a>, env: EnvRc<'a>) -> Result<Object<'a>, String> {
    Ok(eval_statements_unwrap(program.statements, env)?)
}

fn eval_statements_unwrap<'a>(stmts: Vec<Statement<'a>>, env: EnvRc<'a>) -> Result<Object<'a>, String> {
    let mut result = Object::Null;
    for stmt in stmts {

        match eval_statement(stmt, Rc::clone(&env))? {
            Object::Return(i) => return Ok(*i),
            i                 => result = i,
        }
    }
    Ok(result)
}

fn eval_statements<'a>(stmts: Vec<Statement<'a>>, env: EnvRc<'a>) -> Result<Object<'a>, String> {
    let mut result = Object::Null;
    for stmt in stmts {

        match eval_statement(stmt, Rc::clone(&env))? {
            i @ Object::Return(_) => return Ok(i),
            i                     => result = i,
        }
    }
    Ok(result)
}

fn eval_statement<'a>(stmt: Statement<'a>, env: EnvRc<'a>) -> Result<Object<'a>, String> {
    Ok(match stmt {
        Statement::ExpressionStatement(i) => eval_expression(i, env)?,
        Statement::ReturnStatement(i)     => Object::Return(Box::new(eval_expression(i, env)?)),
        Statement::LetStatement(i, a)     => {
            env.borrow_mut().set(i, eval_expression(a, Rc::clone(&env))?);
            Object::Null
        }
    })
}

fn eval_expression<'a>(exp: Expression<'a>, env: EnvRc<'a>) -> Result<Object<'a>, String> {
    Ok(match exp {
        Expression::Int(i)                   => Object::Integer(i), 
        Expression::Bool(i)                  => Object::Boolean(i),
        Expression::Ident(i)                 => env.borrow_mut().get(i)?,
        Expression::PrefixExpression(i, e)   => eval_prefix_expression(i, *e, env)?,
        Expression::InfixExpression(i, e, a) => eval_infix_expression(*i, e, *a, env)?,
        Expression::IfExpression(i, e, a)    => eval_if_expression(*i, e, a, env)?,
        Expression::FunctionLiteral(i, a)    => Object::Function(i, a, env),
        Expression::CallExpression(i, a)     => eval_call_expression(*i, a, env)?
    })
}

fn eval_call_expression<'a>(exp: Expression<'a>, args: Option<Vec<Expression<'a>>>, env: EnvRc<'a>) -> Result<Object<'a>, String> {
    let obj = eval_expression(exp, Rc::clone(&env))?;

    let args = if let Some(i) = args {
        Some(eval_expressions(i, env)?)
    } else {None};

    Ok(apply_function(obj, args)?)
}

fn apply_function<'a>(obj: Object<'a>, args: Option<Vec<Object<'a>>>) -> Result<Object<'a>, String> {
    let params;
    let env;
    let statements;

    if let Object::Function(i, a, b) = obj {
        params = i;
        statements = a;
        env = b;
    } else {
        return Err(format!("Runtime error: {} is not a function", obj))
    };

    let extended_env = extend_function_env(params, env, args)?;
    Ok(eval_statements_unwrap(statements, extended_env)?)
}

fn extend_function_env<'a>(params: Option<Vec<&'a str>>, env: EnvRc<'a>, args: Option<Vec<Object<'a>>>) -> Result<EnvRc<'a>, String> {
    let env = environment::new_enclosed_environment(env);
    match (params, args) {
        (Some(i), Some(e)) => {
            if i.len() != e.len() {
                return Err(format!("Runtime error: Expected {} arguments, got {} parameters", i.len(), e.len()))
            }
            for (i, param) in i.iter().enumerate() {
                env.borrow_mut().set(param, e[i].clone())
            }
            Ok(env)
        },
        (None, None) => return Ok(env),
        _            => return Err(format!("Runtime error: Function has the wrong number of parameters"))
    }
}

fn eval_expressions<'a>(args: Vec<Expression<'a>>, env: EnvRc<'a>) -> Result<Vec<Object<'a>>, String> {
    let mut objs = Vec::new();
    for arg in args.iter() {
        objs.push(eval_expression(arg.clone(), Rc::clone(&env))?)
    }

    Ok(objs)
}

fn eval_prefix_expression<'a>(prefix: Prefix, exp: Expression<'a>, env: EnvRc<'a>) -> Result<Object<'a>, String> {
    Ok(match prefix {
        Prefix::Not         => eval_not_prefix(eval_expression(exp, env)?)?,
        Prefix::PrefixMinus => eval_minus_prefix(eval_expression(exp, env)?)?
    })
}

fn eval_not_prefix<'a>(obj: Object) -> Result<Object<'a>, String> {
    if let Object::Boolean(i) = obj {
        Ok(Object::Boolean(!i))
    } else {
        Err(format!("Runtime error: !{}", obj))
    }
}

fn eval_minus_prefix<'a>(obj: Object) -> Result<Object<'a>, String> {
    if let Object::Integer(i) = obj {
        Ok(Object::Integer(-i))
    } else {
        Err(format!("Runtime error: -{}", obj))
    }
}

fn eval_infix_expression<'a>(left: Expression<'a>, infix: Infix, right: Expression<'a>, env: EnvRc<'a>) -> Result<Object<'a>, String> {
    let left  = eval_expression(left, Rc::clone(&env))?;
    let right = eval_expression(right, env)?;

    Ok(match infix {
        Infix::Plus        => Object::Integer(is_integer(left)? + is_integer(right)?),
        Infix::Minus       => Object::Integer(is_integer(left)? - is_integer(right)?),
        Infix::Multiply    => Object::Integer(is_integer(left)? * is_integer(right)?),
        Infix::Divide      => Object::Integer(is_integer(left)? / is_integer(right)?),
        Infix::LessThan    => Object::Boolean(is_integer(left)? < is_integer(right)?),
        Infix::GreaterThan => Object::Boolean(is_integer(left)? > is_integer(right)?),
        Infix::Equal       => Object::Boolean(left == right),
        Infix::NotEqual    => Object::Boolean(left != right),
    })
}

fn eval_if_expression<'a>(condition: Expression<'a>, consequence: Vec<Statement<'a>>, alternative: Option<Vec<Statement<'a>>>, env: EnvRc<'a>) -> Result<Object<'a>, String> {
    let condition = eval_expression(condition, Rc::clone(&env))?;
    
    if let Object::Boolean(c) = condition {
        Ok(if c {
            eval_statements(consequence, Rc::clone(&env))?
        } else if let Some(a) = alternative {
            eval_statements(a, env)?
        } else {
            Object::Null
        })
    } else {
        Err(format!("Runtime error: Expected boolean, found {}", condition))
    }
}

fn is_integer(obj: Object) -> Result<i64, String> {
    if let Object::Integer(i) = obj {
        Ok(i)
    } else {
        Err(format!("Runtime error: expected integer, found {}", obj))
    }
}
