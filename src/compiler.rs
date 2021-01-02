//x86-64 nasm reference https://www.csee.umbc.edu/portal/help/nasm/sample_64.shtml

use crate::analysis;
use crate::ast::{Type, AST};
use crate::parser;
use std::collections::HashMap;

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

impl IR {
    fn emit(&self, _symbol_table: &HashMap<String, Type>) -> String {
        match self {
            IR::PUSH(x) => format!("mov ax, {}\npush ax", x),
            IR::ADD => "pop bx\npop ax\nadd ax, bx\npush ax".to_string(),
            IR::SUB => "pop bx\npop ax\nsub ax, bx\npush ax".to_string(),
            IR::MUL => "pop bx\npop ax\nmul bx\npush ax".to_string(),
            IR::DIV => "pop bx\npop ax\ndiv bx\npush ax".to_string(),
            IR::GetGlobal(v) => format!("mov al, [{}]\npush ax", v),
            IR::SetGlobal(v) => format!("pop ax\nmov [{}], al", v),
        }
    }
}

pub fn ir_to_asm(is: &[IR], symbol_table: &HashMap<String, Type>) -> String {
    red_ln!("{:?}", is);

    let mut s = String::new();

    let sz = |t| match t {
        Type::INT => "dd",
        Type::BOOL => "db",
    };

    s.push_str("\nsection .data\n\n");
    for (name, t) in symbol_table {
        s.push_str(format!("{} {} 0\n", name, sz(*t)).as_str());
    }
    s.push_str("\nsection .text\n");
    s.push_str("\n_start:\n\n");

    for i in is {
        s.push_str(i.emit(symbol_table).as_str());
        s.push_str("\n");
    }

    s
}

pub fn compile(ast: &AST) -> Result<String, analysis::CompilationError> {
    let mut sym_tabl = HashMap::new();

    let vec = ast_to_ir(ast, &mut sym_tabl);

    let _ = analysis::vars(ast, &sym_tabl)?;
    let _ = analysis::types(ast, &sym_tabl)?;

    red_ln!("Symbol table: {:?}", sym_tabl);

    Ok(ir_to_asm(vec.as_ref(), &sym_tabl))
}

//TODO: Some nodes on the tree don't need a specific instruction,
// e.g. DeclareGlobal
fn ast_to_ir(ast: &AST, symbol_table: &mut HashMap<String, Type>) -> Vec<IR> {
    use IR::*;
    match ast {
        AST::Number(x) => vec![PUSH(x.to_owned())],
        AST::Add(a, b) => {
            let l = ast_to_ir(a, symbol_table);
            let r = ast_to_ir(b, symbol_table);

            let mut v: Vec<IR> = Vec::new();

            v.extend(l);
            v.extend(r);
            v.push(ADD);

            v
        }
        AST::Sub(a, b) => {
            let l = ast_to_ir(a, symbol_table);
            let r = ast_to_ir(b, symbol_table);

            let mut v: Vec<IR> = Vec::new();

            v.extend(l);
            v.extend(r);
            v.push(SUB);

            v
        }
        AST::Mul(a, b) => {
            let l = ast_to_ir(a, symbol_table);
            let r = ast_to_ir(b, symbol_table);

            let mut v: Vec<IR> = Vec::new();

            v.extend(l);
            v.extend(r);
            v.push(MUL);

            v
        }
        AST::Div(a, b) => {
            let l = ast_to_ir(a, symbol_table);
            let r = ast_to_ir(b, symbol_table);

            let mut v: Vec<IR> = Vec::new();

            v.extend(l);
            v.extend(r);
            v.push(DIV);

            v
        }

        AST::GetGlobal(name) => vec![IR::GetGlobal(name.to_owned())],

        AST::AssignGlobal(name, expr) => {
            let mut c = ast_to_ir(expr, symbol_table);
            c.push(IR::SetGlobal(name.to_owned()));
            c
        }

        AST::Many(v) => {
            v.iter()
                .map(|t| ast_to_ir(t, symbol_table))
                .fold(Vec::new(), |acc, new| {
                    let mut v = acc;
                    v.extend(new);
                    v
                })
        }

        AST::DeclareGlobal(name, t) => {
            symbol_table.insert(name.to_owned(), t.to_owned());
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn print_expr(expr: &str) {
        let ast = parser::parse(expr);
        let mut symbol_table = HashMap::new();
        let is = ast_to_ir(&ast, &mut symbol_table);

        red_ln!("{:?} => {:?}", ast, is);
        red_ln!("ast_to_ird: ");

        for i in is {
            red_ln!("{}", i.emit(&symbol_table));
        }
    }

    #[test]
    fn test_add_mul() {
        print_expr("1 + 2 * 3");
        print_expr("( 1 + 2 ) * 3");
        print_expr("( 4 + 4 ) / 2");
    }
}
