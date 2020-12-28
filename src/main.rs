//TODO: Add support for the ternary operator
//TODO: some tests fail because the OS can't open/execute the .o or a.out files

#[macro_use]
extern crate nom;

#[macro_use]
extern crate colour;

mod ast;
mod compiler;
mod output;
mod parser;

fn compile_and_run(input: &str) -> i32 {
    let ast = parser::parse(input);
    let mut ins = compiler::ir_to_asm(&compiler::compile(&ast));
    ins.push_str("EXIT\n");
    output::build_and_run_with_template(ins.as_str(), "asm/template")
}

fn main() {
    compile_and_run("(6-1)/2");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmp_eval_compile(input: &str) -> bool {
        let parsed = parser::parse(input);

        cyan_ln!("Cmp: parsed {:?} = {:?}", input, parsed);

        let eval = parsed.eval();
        let exec = compile_and_run(input);

        eval == exec
    }

    // Here we test if the evaluated AST has the same output as the compiled program.
    #[test]
    fn test_eval_and_compile() {
        assert!(cmp_eval_compile("1 + 2"));
        assert!(cmp_eval_compile("2 * 2 + 3"));
        assert!(cmp_eval_compile("2/2+3"));
        assert!(cmp_eval_compile("((  1+2  ) * 3+4  )    *4"));
    }

    #[test]
    fn test_globals() {
        println!(
            "{:?}",
            compile_and_run("a = 10; a = a * a; y = a / 20 - 1; y")
        )
    }
}
