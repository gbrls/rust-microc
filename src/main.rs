use ast::Type;

//TODO: Add support for the ternary operator
//TODO: some tests fail because the OS can't open/execute the .o or a.out files
//TODO: recoverable errors

#[macro_use]
extern crate nom;

#[macro_use]
extern crate colour;

extern crate strum;

mod analysis;
mod ast;
mod compiler;
mod output;
mod parser;

fn exec_file(path: &str) -> Result<(), analysis::CompilationError> {
    let contents = std::fs::read_to_string(path).expect("File not found");
    red_ln!("contents: {:?}", contents);
    let res = compile_and_run(contents.as_str())?;
    cyan_ln!("{} -> {}", path, res);

    Ok(())
}

fn compile_and_run(input: &str) -> Result<i32, analysis::CompilationError> {
    let ast = parser::parse(input);
    red_ln!("ast: {:?}", ast);

    let mut ins = compiler::compile(&ast)?;
    ins.push_str("\nEXIT\n");

    Ok(output::build_and_run_with_template(
        ins.as_str(),
        "asm/template",
    ))
}

fn main() -> Result<(), analysis::CompilationError> {
    //exec_file("examples/1.mc");
    exec_file("examples/2.mc")?;
    exec_file("examples/3.mc")?;
    exec_file("examples/4.mc")?;
    exec_file("examples/5.mc")?;
    exec_file("examples/6.mc")?;
    exec_file("examples/7.mc")?;
    exec_file("examples/8.mc")?;
    exec_file("examples/9.mc")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmp_eval_compile(input: &str) -> bool {
        let parsed = parser::parse(input);

        cyan_ln!("Cmp: parsed {:?} = {:?}", input, parsed);

        let eval = parsed.eval();
        let exec = compile_and_run(input).unwrap();

        eval == exec
    }

    // Here we test if the evaluated AST has the same output as the compiled program.
    #[test]
    fn test_eval_and_compile() {
        assert!(cmp_eval_compile("1 + 2;"));
        assert!(cmp_eval_compile("2 * 2 + 3;"));
        assert!(cmp_eval_compile("2/2+3;"));
        assert!(cmp_eval_compile("((  1+2  ) * 3+4  )    *4;"));
        assert!(cmp_eval_compile("(6-1)/2;"));
    }

    fn assert_compile(v: i32, i: &str) {
        let ans = compile_and_run(i).unwrap();
        assert_eq!(ans, v);
    }

    #[test]
    fn test_globals() {
        assert_compile(4, "int a;int y;a = 10; a = a * a; y = a / 20 - 1; y;");

        assert_compile(
            10,
            "int a; { a = 10; if (false) { a = 1; if (false) { a = 2; } a = 5; } } a;",
        );

        assert_compile(
            3, 
            "int a; { a = 0; if (false) { a = 1; } else { a = 2; if (true) { a = 3; } else { a = 4; } } } a;"
        );

        assert_compile(
            12,
            "int a; { a = 10; if (true && true && false) { a = 11; } else { if (true && (false || true)) { a = 12; } else { if (true || false) { a = 13; } } } } a;",
        );
    }
}
