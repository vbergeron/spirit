pub mod functions;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpiritValue {
    Const(i64),
    Lit(String),
    Let(String, Box<SpiritValue>, Box<SpiritValue>),
    Function(String, Box<SpiritValue>),
    Apply(Box<SpiritValue>, Box<SpiritValue>),
    Native2(
        Box<SpiritValue>,
        Box<SpiritValue>,
        fn(SpiritValue, SpiritValue) -> Result<SpiritValue, String>,
    ),
    Cond(Box<SpiritValue>, Box<SpiritValue>, Box<SpiritValue>),
}

fn reduce(argname: String, argvalue: SpiritValue, subject: SpiritValue) -> SpiritValue {
    return match subject {
        SpiritValue::Const(v) => SpiritValue::Const(v),
        SpiritValue::Lit(name) => {
            if name == argname {
                argvalue
            } else {
                SpiritValue::Lit(name)
            }
        }
        SpiritValue::Let(name, head, body) => SpiritValue::Let(
            name,
            Box::new(reduce(argname.clone(), argvalue.clone(), *head)),
            Box::new(reduce(argname.clone(), argvalue.clone(), *body)),
        ),
        SpiritValue::Function(name, code) => SpiritValue::Function(
            name,
            Box::new(reduce(argname.clone(), argvalue.clone(), *code)),
        ),
        SpiritValue::Apply(func, argv) => SpiritValue::Apply(
            Box::new(reduce(argname.clone(), argvalue.clone(), *func)),
            Box::new(reduce(argname.clone(), argvalue.clone(), *argv)),
        ),
        SpiritValue::Native2(lhs, rhs, nativefn) => SpiritValue::Native2(
            Box::new(reduce(argname.clone(), argvalue.clone(), *lhs)),
            Box::new(reduce(argname.clone(), argvalue.clone(), *rhs)),
            nativefn,
        ),
        SpiritValue::Cond(cond, body, fallback) => SpiritValue::Cond(
            Box::new(reduce(argname.clone(), argvalue.clone(), *cond)),
            Box::new(reduce(argname.clone(), argvalue.clone(), *body)),
            Box::new(reduce(argname.clone(), argvalue.clone(), *fallback)),
        ),
    };
}

fn eval(env: &mut HashMap<String, SpiritValue>, value: SpiritValue) -> Result<SpiritValue, String> {
    println!("{:?}", value);
    return match value {
        SpiritValue::Const(v) => Ok(SpiritValue::Const(v)),

        SpiritValue::Lit(name) => Err(format!("symbol {} was not reduced", name)),
        //match env.get(&name) {
        //    Some(expr) => Result::Ok(eval(env, expr.clone())?),
        //    None => Result::Err(format!("undefined variable {}", name).to_string()),
        //},
        SpiritValue::Let(name, head, body) => eval(env, reduce(name, *head, *body)),

        SpiritValue::Function(argname, body) => Ok(SpiritValue::Function(argname, body)),

        SpiritValue::Apply(func, argvalue) => match eval(env, *func)? {
            SpiritValue::Function(argname, funcbody) => {
                let res = reduce(argname.clone(), *argvalue, *funcbody);
                eval(env, res)
            }
            other => {
                Result::Err(format!("calling {:?} which is not a function", other).to_string())
            }
        },

        SpiritValue::Native2(arg0, arg1, nativefn) => {
            nativefn(eval(env, *arg0)?, eval(env, *arg1)?)
        }

        SpiritValue::Cond(cond, body, fallback) => {
            if eval(env, *cond) != Ok(SpiritValue::Const(0)) {
                eval(env, *body)
            } else {
                eval(env, *fallback)
            }
        }
    };
}

fn parse<Tokens>(tokens: &mut Tokens) -> Option<SpiritValue>
where
    Tokens: Iterator<Item = String>,
{
    match tokens.next()?.as_str() {
        "let" => {
            let name = tokens.next()?.to_string();
            tokens.next().filter(|x| x == "=");
            let head = parse(tokens)?;
            tokens.next().filter(|x| x == "in");
            let body = parse(tokens)?;
            Some(SpiritValue::Let(
                name.to_string(),
                Box::new(head),
                Box::new(body),
            ))
        }

        "apply" => {
            let func = parse(tokens)?;
            let arg = parse(tokens)?;
            Some(SpiritValue::Apply(Box::new(func), Box::new(arg)))
        }

        "add" => {
            let lhs = parse(tokens)?;
            let rhs = parse(tokens)?;
            Some(SpiritValue::Native2(
                Box::new(lhs),
                Box::new(rhs),
                functions::add,
            ))
        }

        "def" => {
            let argname = tokens.next()?.to_string();
            tokens.next().filter(|x| x == "->");
            let funcbody = parse(tokens)?;
            Some(SpiritValue::Function(
                argname.to_string(),
                Box::new(funcbody),
            ))
        }

        "if" => {
            let cond = parse(tokens)?;
            tokens.next().filter(|x| x == "then");
            let body = parse(tokens)?;
            tokens.next().filter(|x| x == "else");
            let fallback = parse(tokens)?;
            Some(SpiritValue::Cond(
                Box::new(cond),
                Box::new(body),
                Box::new(fallback),
            ))
        }

        token => match token.parse::<i64>() {
            Ok(value) => Some(SpiritValue::Const(value)),
            Err(_) => Some(SpiritValue::Lit(token.to_string())),
        },
    }
}

fn read() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    return buffer;
}

fn main() {
    let mut env: HashMap<String, SpiritValue> = HashMap::new();

    loop {
        let line = read();
        let mut tokens = line.split_whitespace().map(|x| x.to_string());
        let code = parse(&mut tokens).unwrap();
        let result = eval(&mut env, code);
        println!("{:?}", result);
    }
}
