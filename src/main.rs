use std::io::Write;

pub mod env;
pub mod functions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AST {
    Nil,
    Const(i64),
    Lit(String),
    Let(String, Box<AST>, Box<AST>),
    Def(String, Box<AST>),
    Function(String, Box<AST>),
    Apply(Box<AST>, Box<AST>),
    Native1(String, Box<AST>),
    Native2(String, Box<AST>, Box<AST>),
    Cond(Box<AST>, Box<AST>, Box<AST>),
}

fn TRUE() -> AST {
    AST::Lit("true".to_string())
}
fn FALSE() -> AST {
    AST::Lit("false".to_string())
}

fn eval(env: &mut env::Env, value: AST) -> Result<AST, String> {
    if env.debug() {
        println!("{:?}", value.clone());
    }
    return match value {
        AST::Nil => Ok(AST::Nil),

        AST::Const(v) => Ok(AST::Const(v)),

        AST::Lit(name) => match env.get_var(name.clone()) {
            Some(expr) => Result::Ok(eval(env, expr.clone())?),
            None => Ok(AST::Lit(name)),
        },

        AST::Let(name, head, body) => {
            let define = eval(env, *head)?;
            env.add_var(name.clone(), define);
            let res = eval(env, *body)?;
            env.del_var(name);
            Ok(res)
        }

        AST::Def(name, defined) => {
            env.add_var(name.clone(), *defined);
            Ok(AST::Nil)
        }

        AST::Function(argname, body) => Ok(AST::Function(argname, body)),

        AST::Apply(func, argvalue) => match eval(env, *func)? {
            AST::Function(argname, funcbody) => {
                let computed_arg = eval(env, *argvalue)?;
                env.frame_push();
                env.add_var(argname.clone(), computed_arg.clone());
                let computed = eval(env, *funcbody)?;
                let res = match computed {
                    AST::Function(arg, body) => AST::Function(
                        arg,
                        Box::new(AST::Let(argname.clone(), Box::new(computed_arg), body)),
                    ),
                    other => other,
                };
                env.del_var(argname.clone());
                env.frame_pop();
                Ok(res)
            }
            other => {
                Result::Err(format!("calling {:?} which is not a function", other).to_string())
            }
        },

        AST::Native1(name, arg0) => {
            let mut nenv = env.clone();
            match env.get_native(name.clone()) {
                Some(f) => {
                    let _arg0 = eval(&mut nenv, *arg0)?;
                    f(vec![_arg0])
                }
                None => Err(format!("native function {} not defined", name)),
            }
        }

        AST::Native2(name, arg0, arg1) => {
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

        AST::Cond(cond, body, fallback) => {
            if eval(env, *cond) == Ok(AST::Lit("true".to_string())) {
                eval(env, *body)
            } else {
                eval(env, *fallback)
            }
        }
    };
}

fn parse_iter<Tokens>(tokens: &mut Tokens) -> Option<AST>
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
            Some(AST::Let(name.to_string(), Box::new(head), Box::new(body)))
        }

        "@" => {
            let func = parse_iter(tokens)?;
            let arg = parse_iter(tokens)?;

            Some(AST::Apply(Box::new(func), Box::new(arg)))
        }

        "native1" => {
            let name = tokens.next()?.to_string();
            let arg0 = parse_iter(tokens)?;
            Some(AST::Native1(name, Box::new(arg0)))
        }

        "native2" => {
            let name = tokens.next()?.to_string();
            let arg0 = parse_iter(tokens)?;
            let arg1 = parse_iter(tokens)?;
            Some(AST::Native2(name, Box::new(arg0), Box::new(arg1)))
        }

        "fn" => {
            let argname = tokens.next()?.to_string();
            tokens.next().filter(|x| x == "->");
            let funcbody = parse_iter(tokens)?;
            Some(AST::Function(argname.to_string(), Box::new(funcbody)))
        }

        "def" => {
            let name = tokens.next()?.to_string();
            tokens.next().filter(|x| x == "=");
            let defined = parse_iter(tokens)?;
            Some(AST::Def(name.to_string(), Box::new(defined)))
        }

        "if" => {
            let cond = parse_iter(tokens)?;
            tokens.next().filter(|x| x == "then");
            let body = parse_iter(tokens)?;
            tokens.next().filter(|x| x == "else");
            let fallback = parse_iter(tokens)?;
            Some(AST::Cond(
                Box::new(cond),
                Box::new(body),
                Box::new(fallback),
            ))
        }

        token => match token.parse::<i64>() {
            Ok(value) => Some(AST::Const(value)),
            Err(_) => Some(AST::Lit(token.to_string())),
        },
    }
}

fn parse(code: String) -> AST {
    let mut tokens = code.split_whitespace().map(|x| x.to_string());
    parse_iter(&mut tokens).unwrap()
}

fn repr(value: AST) -> String {
    match value {
        AST::Nil => "Nil".to_string(),
        AST::Const(v) => format!("{}", v),
        AST::Lit(s) => s,
        AST::Let(name, head, body) => format!("let {} = {} in {}", name, repr(*head), repr(*body)),
        AST::Def(name, _) => format!("def {} = <code>", name),
        AST::Function(argname, body) => format!("fn {} -> {}", argname, repr(*body)),
        AST::Apply(f, arg) => format!("@ {} {}", repr(*f), repr(*arg)),
        AST::Native1(name, arg0) => format!("native {} {}", name, repr(*arg0)),
        AST::Native2(name, arg0, arg1) => {
            format!("native {} {} {}", name, repr(*arg0), repr(*arg1))
        }
        AST::Cond(cond, body, fallback) => format!(
            "if {}  then {} else {}",
            repr(*cond),
            repr(*body),
            repr(*fallback)
        ),
    }
}

fn read() -> String {
    print!("spirit> ");
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    return buffer;
}

fn print(res: Result<AST, String>) -> () {
    print!("> ");
    match res {
        Ok(result) => println!("{:?}", result),
        Err(err) => println!("ERROR : {:?}", err),
    }
}

fn builtin(env: &mut env::Env, name: &str, code: &str) -> () {
    env.add_var(name.to_string(), parse(code.to_string()));
}

fn main() {
    let mut env = env::Env::new(false);
    env.add_native("native:print".to_string(), functions::print);

    builtin(&mut env, "print", "fn x -> native1 native:print x");

    env.add_native("native:add".to_string(), functions::add);
    env.add_native("native:sub".to_string(), functions::sub);
    env.add_native("native:mul".to_string(), functions::mul);
    env.add_native("native:eq".to_string(), functions::eq);
    env.add_native("native:lt".to_string(), functions::lt);
    env.add_native("native:gt".to_string(), functions::gt);

    builtin(&mut env, "add", "fn x -> fn y -> native2 native:add x y");
    builtin(&mut env, "sub", "fn x -> fn y -> native2 native:sub x y");
    builtin(&mut env, "mul", "fn x -> fn y -> native2 native:mul x y");
    builtin(&mut env, "eq", "fn x -> fn y -> native2 native:eq x y");
    builtin(&mut env, "gt", "fn x -> fn y -> native2 native:gt x y");
    builtin(&mut env, "lt", "fn x -> fn y -> native2 native:lt x y");

    loop {
        print(eval(&mut env, parse(read())));
    }
}
