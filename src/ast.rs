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

impl AST {
    pub fn lit_true() -> AST {
        AST::Lit("true".to_string())
    }
    pub fn lit_false() -> AST {
        AST::Lit("false".to_string())
    }
}
