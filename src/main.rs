use std::io::Write;

pub mod env;
pub mod functions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpiritValue {
    Const(i64),
    Lit(String),
    Let(String, Box<SpiritValue>, Box<SpiritValue>),
    Function(String, Box<SpiritValue>),
    Apply(Box<SpiritValue>, Box<SpiritValue>),
    Native2(String, Box<SpiritValue>, Box<SpiritValue>),
    Cond(Box<SpiritValue>, Box<SpiritValue>, Box<SpiritValue>),
}

fn eval(env: &mut env::Env, value: SpiritValue) -> Result<SpiritValue, String> {
    return match value {
        SpiritValue::Const(v) => Ok(SpiritValue::Const(v)),

        SpiritValue::Lit(name) => match env.get_var(name) {
            Some(expr) => Result::Ok(eval(env, expr.clone())?),
            None => Result::Err("oops undefined variable".to_string()),
        },

        SpiritValue::Let(name, head, body) => {
            let define = eval(env, *head)?;
            env.add_var(name.clone(), define);
            let res = eval(env, *body)?;
            env.del_var(name);
            Ok(res)
        }

        SpiritValue::Function(argname, body) => Ok(SpiritValue::Function(argname, body)),

        SpiritValue::Apply(func, argvalue) => match eval(env, *func)? {
            SpiritValue::Function(argname, funcbody) => {
                env.frame_push();
                env.add_var(argname.clone(), eval(&mut env.clone(), *argvalue)?);
                let res = eval(env, *funcbody)?;
                env.del_var(argname);
                env.frame_pop();
                Ok(res)
            }
            other => {
                Result::Err(format!("calling {:?} which is not a function", other).to_string())
            }
        },

        SpiritValue::Native2(name, arg0, arg1) => {
            let mut nenv = env.clone();
            match env.get_native(name.clone()) {
                Some(f) => {
                    let _arg0 = eval(&mut nenv, *arg0)?;
                    let _arg1 = eval(&mut nenv, *arg1)?;
                    f(vec![_arg0, _arg1])
                }
                None => Err(format!("native function {} not defined", name)),
            }
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

fn parse_iter<Tokens>(tokens: &mut Tokens) -> Option<SpiritValue>
where
    Tokens: Iterator<Item = String>,
{
    match tokens.next()?.as_str() {
        "let" => {
            let name = tokens.next()?.to_string();
            tokens.next().filter(|x| x == "=");
            let head = parse_iter(tokens)?;
            tokens.next().filter(|x| x == "in");
            let body = parse_iter(tokens)?;
            Some(SpiritValue::Let(
                name.to_string(),
                Box::new(head),
                Box::new(body),
            ))
        }

        "apply" => {
            let func = parse_iter(tokens)?;
            let arg = parse_iter(tokens)?;
            Some(SpiritValue::Apply(Box::new(func), Box::new(arg)))
        }

        "native2" => {
            let name = tokens.next()?.to_string();
            let lhs = parse_iter(tokens)?;
            let rhs = parse_iter(tokens)?;
            Some(SpiritValue::Native2(name, Box::new(lhs), Box::new(rhs)))
        }

        "def" => {
            let argname = tokens.next()?.to_string();
            tokens.next().filter(|x| x == "->");
            let funcbody = parse_iter(tokens)?;
            Some(SpiritValue::Function(
                argname.to_string(),
                Box::new(funcbody),
            ))
        }

        "if" => {
            let cond = parse_iter(tokens)?;
            tokens.next().filter(|x| x == "then");
            let body = parse_iter(tokens)?;
            tokens.next().filter(|x| x == "else");
            let fallback = parse_iter(tokens)?;
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

fn parse(code: String) -> SpiritValue {
    let mut tokens = code.split_whitespace().map(|x| x.to_string());
    parse_iter(&mut tokens).unwrap()
}

fn read() -> String {
    print!("spirit> ");
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    return buffer;
}

fn print(res:Result<SpiritValue, String>) -> () {
    print!("> ");
    match res {
        Ok(result) => 
        println!("{:?}", result),
        Err(err) => println!("ERROR : {:?}", err),
    }
}

fn main() {
    let mut env = env::Env::new(false);
    env.add_native("native:add".to_string(), functions::add);
    env.add_native("native:mul".to_string(), functions::mul);
    env.add_native("native:eq".to_string(), functions::eq);

    loop {
        print(eval(&mut env, parse(read())));
    }
}
