#[macro_use]
extern crate nom;

#[macro_use]
extern crate colour;

mod ast;
mod compiler;
mod output;
mod parser;

fn main() {
    let ast = parser::parse("(2 + 3) * 4");
    let mut ins = compiler::instr_to_string(&compiler::compile(&ast));
    ins.push_str("EXIT\n");
    output::build_and_run_with_template(ins.as_str(), "ast/template");
}
