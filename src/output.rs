use std::io::Write;
use std::process::Command;

pub fn build_and_run_with_template(program: &str, template: &str) -> i32 {
    let f = "internal";

    let prog = use_template(program, template);

    write_to_file(prog.as_ref(), format!("asm/{}.asm", f).as_ref()).unwrap();
    assemble(format!("asm/{}.asm", f).as_ref());
    link(format!("asm/{}.o", f).as_ref());
    execute("asm/a.out")
}

pub fn build_and_run(program: &str) -> i32 {
    let f = "internal";

    write_to_file(program, format!("asm/{}.asm", f).as_ref()).unwrap();
    assemble(format!("asm/{}.asm", f).as_ref());
    link(format!("asm/{}.o", f).as_ref());
    execute("asm/a.out")
}

fn use_template(program: &str, template: &str) -> String {
    cyan_ln!("Tying to read {}", template);
    let mut t = std::fs::read_to_string(template).unwrap();
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

fn execute(fname: &str) -> i32 {
    cyan_ln!("Trying to execute {}", fname);

    let output = Command::new(fname)
        .status()
        .unwrap_or_else(|e| panic!("Falied to execute {}", e))
        .code()
        .unwrap();

    blue_ln!("Executed {} with output {:?}", fname, output);

    //delete(fname);

    output
}

fn delete(fname: &str) {
    cyan_ln!("Trying to delete {}", fname);

    let output = Command::new("rm")
        .arg(fname)
        .status()
        .unwrap_or_else(|e| panic!("Falied to execute {}", e))
        .code()
        .unwrap();
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
        delete("asm/a.out");
    }

    #[test]
    //fn test_exec() {
    //    execute("asm/a.out");
    //    delete("asm/a.out");
    //}
    #[test]
    fn test_pipeline() {
        let f = std::fs::read_to_string("asm/ret.asm").unwrap();
        build_and_run(f.as_ref());
    }

    #[test]
    fn test_pipeline_template() {
        ls();
        build_and_run_with_template("push 42\nEXIT", "./asm/template");
    }

    fn ls() {
        let output = Command::new("ls")
            .arg("./asm")
            .output()
            .unwrap_or_else(|e| panic!("Falied to execute {}", e))
            .stdout
            .to_ascii_lowercase();
        red_ln!("{:?}", String::from_utf8(output));
    }
}
