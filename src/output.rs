use std::io::Write;
use std::process::Command;

pub fn build_and_run_with_template(program: &str, template: &str) {
    let f = "internal";

    let prog = use_template(program, template);

    write_to_file(prog.as_ref(), format!("asm/{}.asm", f).as_ref()).unwrap();
    assemble(format!("asm/{}.asm", f).as_ref());
    link(format!("asm/{}.o", f).as_ref());
    execute("asm/a.out");
}

pub fn build_and_run(program: &str) {
    let f = "internal";

    write_to_file(program, format!("asm/{}.asm", f).as_ref()).unwrap();
    assemble(format!("asm/{}.asm", f).as_ref());
    link(format!("asm/{}.o", f).as_ref());
    execute("asm/a.out");
}

fn use_template(program: &str, template: &str) -> String {
    let mut t = std::fs::read_to_string("asm/template").unwrap();
    t.push('\n');
    t.push_str(program);

    t
}

fn write_to_file(program: &str, fname: &str) -> std::io::Result<()> {
    let mut f = std::fs::File::create(fname)?;
    f.write_all(program.as_bytes())?;

    Ok(())
}

fn assemble(fname: &str) {
    let output = Command::new("nasm")
        .arg("-felf64")
        .arg(fname)
        .output()
        .unwrap_or_else(|e| panic!("Falied to execute {}", e));

    println!("Assembled with output: {:?}", output);
}

fn link(fname: &str) {
    let output = Command::new("ld")
        .arg(fname)
        .arg("-o")
        .arg("asm/a.out")
        .output()
        .unwrap_or_else(|e| panic!("Falied to execute {}", e));

    println!("Linked with output: {:?}", output);
}

fn execute(fname: &str) {
    let output = Command::new(fname)
        .status()
        .unwrap_or_else(|e| panic!("Falied to execute {}", e))
        .code()
        .unwrap();

    blue_ln!("Executed {} with output {:?}", fname, output);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_assemble() {
        assemble("asm/ret.asm");
    }

    #[test]
    fn test_link() {
        link("asm/ret.o");
    }

    #[test]
    fn test_exec() {
        execute("asm/a.out");
    }

    #[test]
    fn test_pipeline() {
        let f = std::fs::read_to_string("asm/ret.asm").unwrap();
        build_and_run(f.as_ref());
    }

    #[test]
    fn test_pipeline_template() {
        build_and_run_with_template("push 42\nEXIT", "asm/template");
    }
}
