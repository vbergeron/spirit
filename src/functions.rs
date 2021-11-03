use crate::{AST, FALSE, TRUE};

fn numeric_op(lhs: AST, rhs: AST, f: fn(i64, i64) -> i64) -> Result<AST, String> {
    match (lhs, rhs) {
        (AST::Const(l), AST::Const(r)) => Ok(AST::Const(f(l, r))),
        (_, _) => Err("type error".to_string()),
    }
}

fn unpack2(mut args: Vec<AST>) -> Result<(AST, AST), String> {
    if args.len() == 2 {
        let arg1 = args.pop().ok_or("err".to_string())?;
        let arg0 = args.pop().ok_or("err".to_string())?;
        Ok((arg0, arg1))
    } else {
        Err(format!("wrong arity: expected {}, got {}", 2, args.len()))
    }
}

fn unpack1(mut args: Vec<AST>) -> Result<AST, String> {
    if args.len() == 1 {
        let arg0 = args.pop().ok_or("err".to_string())?;
        Ok(arg0)
    } else {
        Err(format!("wrong arity: expected {}, got {}", 1, args.len()))
    }
}

pub fn print(args: Vec<AST>) -> Result<AST, String> {
    let value = unpack1(args)?;
    println!("{}", crate::repr(value));
    Ok(AST::Nil)
}

pub fn add(args: Vec<AST>) -> Result<AST, String> {
    let (lhs, rhs) = unpack2(args)?;
    numeric_op(lhs, rhs, |x, y| x + y)
}

pub fn sub(args: Vec<AST>) -> Result<AST, String> {
    let (lhs, rhs) = unpack2(args)?;
    numeric_op(lhs, rhs, |x, y| x - y)
}

pub fn mul(args: Vec<AST>) -> Result<AST, String> {
    let (lhs, rhs) = unpack2(args)?;
    numeric_op(lhs, rhs, |x, y| x * y)
}

pub fn eq(args: Vec<AST>) -> Result<AST, String> {
    let (lhs, rhs) = unpack2(args)?;
    if lhs == rhs {
        Ok(TRUE())
    } else {
        Ok(FALSE())
    }
}

pub fn gt(args: Vec<AST>) -> Result<AST, String> {
    let (lhs, rhs) = unpack2(args)?;
    match (lhs, rhs) {
        (AST::Const(l), AST::Const(r)) => {
            if l > r {
                Ok(TRUE())
            } else {
                Ok(FALSE())
            }
        }
        (AST::Lit(l), AST::Lit(r)) => {
            if l > r {
                Ok(TRUE())
            } else {
                Ok(FALSE())
            }
        }
        (_, _) => Err("type error".to_string()),
    }
}

pub fn lt(args: Vec<AST>) -> Result<AST, String> {
    let (lhs, rhs) = unpack2(args)?;
    match (lhs, rhs) {
        (AST::Const(l), AST::Const(r)) => {
            if l < r {
                Ok(TRUE())
            } else {
                Ok(FALSE())
            }
        }
        (AST::Lit(l), AST::Lit(r)) => {
            if l < r {
                Ok(TRUE())
            } else {
                Ok(FALSE())
            }
        }
        (_, _) => Err("type error".to_string()),
    }
}
