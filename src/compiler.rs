//TODO: Symbol table!

use crate::ast::AST;
use crate::parser;
use std::fmt;

/// The idea behind this enum is to be our IR.
/// Each instruction pops it's inputs from the stack and then pops them
/// back.
#[derive(Debug)]
pub enum Instructions {
    PUSH(i32),
    ADD,
    SUB,
    MUL,
    DIV,
}

impl fmt::Display for Instructions {
    /// This is how we get from our IR to x86 asm (in this case nasm's)
    //TODO: this function calling stuff should be abstracted.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Instructions::PUSH(x) => write!(f, "mov ax, {}\npush ax", x),
            Instructions::ADD => write!(f, "pop bx\npop ax\nadd ax, bx\npush ax"),
            Instructions::SUB => write!(f, "pop bx\npop ax\nsub ax, bx\npush ax"),
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
        AST::Sub(a, b) => {
            let l = compile(a);
            let r = compile(b);

            let mut v: Vec<Instructions> = Vec::new();

            v.extend(l);
            v.extend(r);
            v.push(SUB);

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

        x => todo!("Compiler can't compile this IR code {:?}", x),
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
