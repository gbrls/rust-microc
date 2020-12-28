//TODO: Symbol table!

use crate::ast::AST;
use crate::parser;
use std::{collections::HashSet, fmt};

/// The idea behind this enum is to be our IR.
/// Each instruction pops it's inputs from the stack and then pops them
/// back.
#[derive(Debug)]
pub enum IR {
    PUSH(i32),
    ADD,
    SUB,
    MUL,
    DIV,
    GetGlobal(String),
    SetGlobal(String),
}

impl fmt::Display for IR {
    /// This is how we get from our IR to x86 asm (in this case nasm's)
    //TODO: this function calling stuff should be abstracted.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IR::PUSH(x) => write!(f, "mov ax, {}\npush ax", x),
            IR::ADD => write!(f, "pop bx\npop ax\nadd ax, bx\npush ax"),
            IR::SUB => write!(f, "pop bx\npop ax\nsub ax, bx\npush ax"),
            IR::MUL => write!(f, "pop bx\npop ax\nmul bx\npush ax"),
            IR::DIV => write!(f, "pop bx\npop ax\ndiv bx\npush ax"),
            IR::GetGlobal(v) => write!(f, "mov al, [{}]\npush ax", v),
            IR::SetGlobal(v) => write!(f, "pop ax\nmov [{}], al", v),
            //x => todo!("Cant compile this IR to asm {:?}", x),
        }
    }
}

pub fn ir_to_asm(is: &[IR]) -> String {
    red_ln!("{:?}", is);
    let mut globals = HashSet::new();
    for i in is {
        match i {
            IR::SetGlobal(x) => globals.insert(x),
            IR::GetGlobal(x) => globals.insert(x),
            _ => false,
        };
    }

    cyan_ln!("Globals {:?}", globals);

    let mut s = String::new();

    s.push_str("\nsection .data\n\n");
    for var in globals {
        s.push_str(format!("{} db 0\n", var).as_str());
    }
    s.push_str("\nsection .text\n");
    s.push_str("\n_start:\n\n");

    for i in is {
        s.push_str(format!("{}\n", i).as_str());
    }

    s
}

pub fn compile(ast: &AST) -> Vec<IR> {
    use IR::*;
    match ast {
        AST::Number(x) => vec![PUSH(x.to_owned())],
        AST::Add(a, b) => {
            let l = compile(a);
            let r = compile(b);

            let mut v: Vec<IR> = Vec::new();

            v.extend(l);
            v.extend(r);
            v.push(ADD);

            v
        }
        AST::Sub(a, b) => {
            let l = compile(a);
            let r = compile(b);

            let mut v: Vec<IR> = Vec::new();

            v.extend(l);
            v.extend(r);
            v.push(SUB);

            v
        }
        AST::Mul(a, b) => {
            let l = compile(a);
            let r = compile(b);

            let mut v: Vec<IR> = Vec::new();

            v.extend(l);
            v.extend(r);
            v.push(MUL);

            v
        }
        AST::Div(a, b) => {
            let l = compile(a);
            let r = compile(b);

            let mut v: Vec<IR> = Vec::new();

            v.extend(l);
            v.extend(r);
            v.push(DIV);

            v
        }

        AST::GetGlobal(name) => vec![IR::GetGlobal(name.to_owned())],

        AST::AssignGlobal(name, expr) => {
            let mut c = compile(expr);
            c.push(IR::SetGlobal(name.to_owned()));
            c
        }

        AST::Many(v) => v.iter().map(|t| compile(t)).fold(Vec::new(), |acc, new| {
            let mut v = acc;
            v.extend(new);
            v
        }),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn print_expr(expr: &str) {
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
        print_expr("1 + 2 * 3");
        print_expr("( 1 + 2 ) * 3");
        print_expr("( 4 + 4 ) / 2");
    }
}
