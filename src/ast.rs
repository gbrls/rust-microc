#[derive(Debug, Clone)]
pub enum AST {
    Number(i32),
    Add(Box<AST>, Box<AST>),
    Sub(Box<AST>, Box<AST>),

    Mul(Box<AST>, Box<AST>),
    Div(Box<AST>, Box<AST>),
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
        }
    }
}
