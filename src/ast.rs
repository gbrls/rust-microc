#[derive(Debug, Clone)]
pub enum AST {
    Number(i32),
    Add(Box<AST>, Box<AST>),
    Sub(Box<AST>, Box<AST>),

    Mul(Box<AST>, Box<AST>),
    Div(Box<AST>, Box<AST>),

    AssignGlobal(String, Box<AST>),
    GetGlobal(String),
    DeclareGlobal(String, Type),

    // Executes multiple expressions and returns the value of the last.
    //
    Many(Vec<AST>),
}

impl AST {
    /// This only exists for debug porposes;
    pub fn eval(&self) -> i32 {
        match &self {
            AST::Number(x) => x.to_owned(),
            AST::Add(a, b) => a.eval() + b.eval(),
            AST::Sub(a, b) => a.eval() - b.eval(),
            AST::Mul(a, b) => a.eval() * b.eval(),
            AST::Div(a, b) => a.eval() / b.eval(),
            AST::Many(v) => v.last().expect("Program can't be empy").eval(),
            x => todo!("Implement evaluation for {:?}", x),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, strum_macros::EnumIter)]
pub enum Type {
    INT,
    BOOL,
}

use std::fmt;

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::INT => write!(f, "int"),
            Type::BOOL => write!(f, "bool"),
        }
    }
}
