#[derive(Debug, Clone)]
pub enum AST {
    Number(i32),
    Bool(bool),

    Add(Box<AST>, Box<AST>),
    Sub(Box<AST>, Box<AST>),

    Mul(Box<AST>, Box<AST>),
    Div(Box<AST>, Box<AST>),

    AssignVar(String, Box<AST>),
    GetVar(String),
    DeclareVar(String, Type),

    // Executes multiple expressions and returns the value of the last.
    //
    Many(Vec<AST>),
    Block(Vec<AST>),
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

// We use this trait to get the size of each type in bytes
impl Into<u32> for Type {
    fn into(self) -> u32 {
        match &self {
            Type::INT => 4,
            Type::BOOL => 1,
        }
    }
}
