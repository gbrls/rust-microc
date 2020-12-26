use crate::ast::AST;
use crate::parser;
use std::fmt;

#[derive(Debug)]
pub enum Instructions {
    PUSH(i32),
    ADD,
    MUL,
    DIV,
}

impl fmt::Display for Instructions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Instructions::PUSH(x) => write!(f, "mov ax, {}\npush ax", x),
            Instructions::ADD => write!(f, "pop bx\npop ax\nadd ax, bx\npush ax"),
            Instructions::MUL => write!(f, "pop bx\npop ax\nmul bx\npush ax"),
            Instructions::DIV => write!(f, "pop bx\npop ax\ndiv bx\npush ax"),
        }
    }
}

pub fn instr_to_string(is: &Vec<Instructions>) -> String {
    let mut s = String::new();
    red_ln!("{:?}", is);

    for i in is {
        s.push_str(format!("{}\n", i).as_str());
    }

    s
}

pub fn compile(ast: &AST) -> Vec<Instructions> {
    use Instructions::*;
    match ast {
        AST::Number(x) => vec![PUSH(x.to_owned())],
        AST::Add(a, b) => {
            let l = compile(a);
            let r = compile(b);

            let mut v: Vec<Instructions> = Vec::new();

            v.extend(l);
            v.extend(r);
            v.push(ADD);

            v
        }

        AST::Mul(a, b) => {
            let l = compile(a);
            let r = compile(b);

            let mut v: Vec<Instructions> = Vec::new();

            v.extend(l);
            v.extend(r);
            v.push(MUL);

            v
        }

        AST::Div(a, b) => {
            let l = compile(a);
            let r = compile(b);

            let mut v: Vec<Instructions> = Vec::new();

            v.extend(l);
            v.extend(r);
            v.push(DIV);

            v
        }
        _ => todo!("Can't compile"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn print_comp(expr: &str) {
        let ast = parser::parse(expr);
        let is = compile(&ast);

        red_ln!("{:?} => {:?}", ast, is);
        red_ln!("Compiled: ");

        for i in is {
            red_ln!("{}", i);
        }
    }

    #[test]
    fn test_add_mul() {
        print_comp("1 + 2 * 3");
        print_comp("( 1 + 2 ) * 3");
        print_comp("( 4 + 4 ) / 2");
    }
}
