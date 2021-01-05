//x86-64 nasm reference https://www.csee.umbc.edu/portal/help/nasm/sample_64.shtml
//TODO: use different registers for different operand sizes

use crate::analysis;
use crate::analysis::CompilationError;
use crate::ast::{Type, AST};
use crate::parser;
use std::collections::HashMap;

#[derive(Debug)]
struct Stack {
    // name, scope, type
    data: Vec<(String, u32, Type)>,
    pub scope: u32,
    size: u32,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            data: Vec::new(),
            scope: 0,
            size: 0,
        }
    }

    // This function removes all the variables declared in the current scope and
    // returns how many bytes should be removed from the stack.
    fn pop(&mut self) -> u32 {
        let mut offset = 0;
        while !self.data.is_empty() {
            let (_, s, t) = self.data.last().unwrap();
            if *s < self.scope {
                break;
            } else {
                let sz: u32 = (*t).into();
                offset += sz;
                self.data.pop();
            }
        }

        self.size -= offset;

        offset
    }

    fn push(&mut self, name: String, typ: Type) {
        self.data.push((name, self.scope, typ));

        let of: u32 = typ.into();
        self.size += of;
    }

    fn find(&self, var: &str) -> Option<(u32, Type)> {
        let mut sz = 0;
        for (name, _, t) in self.data.iter().rev() {
            if name == var {
                return Some(((self.size - sz), *t));
            } else {
                let s: u32 = (*t).into();
                sz += s;
            }
        }
        None
    }
}

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

    // offset, size
    GetLocal(u32, u32),
    SetLocal(u32, u32),

    // Stack operations
    Shrink(u32),
    Grow(u32),
}

impl IR {
    fn emit(&self, syms: &HashMap<String, Type>) -> String {
        let reg = |sz| match sz {
            1 => "al",
            2 => "ax",
            4 => "eax",
            8 => "rax",
            _ => panic!("There's no register for {} bits!", sz * 8),
        };

        match self {
            IR::PUSH(x) => format!("mov ax, {}\npush ax", x),

            IR::ADD => "pop bx\npop ax\nadd eax, ebx\npush ax".to_string(),
            IR::SUB => "pop bx\npop ax\nsub eax, ebx\npush ax".to_string(),
            IR::MUL => "pop bx\npop ax\nmul ebx\npush ax".to_string(),
            IR::DIV => "pop bx\npop ax\ndiv ebx\npush ax".to_string(),

            IR::GetGlobal(v) => {
                let sz: u32 = (*syms.get(v).unwrap()).into();
                format!("mov {}, [{}]\npush ax", reg(sz), v)
            }
            IR::SetGlobal(v) => {
                let sz: u32 = (*syms.get(v).unwrap()).into();
                format!("pop ax\nmov [{}], {}", v, reg(sz))
            }

            IR::GetLocal(v, sz) => format!("mov {}, [rbp-{}]\npush ax", reg(*sz), v),
            IR::SetLocal(v, sz) => format!("pop ax\nmov [rbp-{}], {}", v, reg(*sz)),

            IR::Grow(sz) => format!("sub rsp, {}", sz),
            IR::Shrink(sz) => format!("add rsp, {}", sz),
        }
    }
}

struct Compiler {
    symbols: HashMap<String, Type>,
    stack: Stack,
    ir: Vec<IR>,
}

impl Compiler {
    fn new() -> Compiler {
        Compiler {
            symbols: HashMap::new(),
            stack: Stack::new(),
            ir: Vec::new(),
        }
    }

    fn emit(&mut self, op: IR) {
        self.ir.push(op)
    }

    fn find_type(&self, name: &str) -> Option<Type> {
        match self.stack.find(name) {
            Some((_, t)) => Some(t),
            None => match self.symbols.get(name) {
                Some(v) => Some(*v),
                None => None,
            },
        }
    }

    //TODO: do some static analysis here on the local variables, it is missing.

    fn ast_to_ir(&mut self, ast: &AST) -> Result<Option<Type>, CompilationError> {
        use Type::*;
        let mut bin_int = |a, b, op| -> Result<Option<Type>, CompilationError> {
            let a = self.ast_to_ir(a)?;
            let b = self.ast_to_ir(b)?;

            match (a, b) {
                (Some(Type::INT), Some(Type::INT)) => {
                    self.emit(op);
                    Ok(Some(INT))
                }
                _ => Err(CompilationError::IncompatibleTypes),
            }
        };

        use IR::*;
        match ast {
            AST::Bool(v) => {
                let x = if *v { 1 } else { 0 };
                self.emit(PUSH(x));
                Ok(Some(Type::BOOL))
            }

            AST::Number(x) => {
                self.emit(PUSH(x.to_owned()));
                Ok(Some(Type::INT))
            }
            AST::Add(a, b) => bin_int(a, b, ADD),
            AST::Sub(a, b) => bin_int(a, b, SUB),
            AST::Mul(a, b) => bin_int(a, b, MUL),
            AST::Div(a, b) => bin_int(a, b, DIV),

            AST::Many(v) => {
                for e in v {
                    self.ast_to_ir(e)?;
                }
                Ok(None)
            }

            AST::GetVar(name) => {
                match self.stack.find(name) {
                    Some((off, t)) => self.emit(IR::GetLocal(off, t.into())),
                    None => self.emit(IR::GetGlobal(name.to_owned())),
                }
                cyan_ln!("Looking var {} = {:?}", name, self.find_type(name));
                match self.find_type(name) {
                    Some(t) => Ok(Some(t)),
                    None => Err(CompilationError::VariableNotDeclared(name.to_owned())),
                }
            }

            AST::AssignVar(name, expr) => {
                let e_type = self.ast_to_ir(expr)?;
                match self.stack.find(name) {
                    Some((off, t)) => self.emit(IR::SetLocal(off, t.into())),
                    None => self.emit(IR::SetGlobal(name.to_owned())),
                }

                match (self.find_type(name), e_type) {
                    (Some(a), Some(b)) => {
                        if a == b {
                            Ok(Some(a))
                        } else {
                            Err(CompilationError::IncompatibleTypes)
                        }
                    }
                    (None, _) => Err(CompilationError::VariableNotDeclared(name.to_owned())),
                    (Some(_), None) => Err(CompilationError::IncompatibleTypes),
                }
            }

            AST::DeclareVar(name, typ) => {
                if self.stack.scope > 0 {
                    self.stack.push(name.to_owned(), *typ);
                    let sz: u32 = (*typ).into();
                    self.emit(IR::Grow(sz));
                } else {
                    self.symbols.insert(name.to_owned(), *typ);
                }
                Ok(None)
            }

            AST::Block(v) => {
                self.stack.scope += 1;

                self.ast_to_ir(&AST::Many(v.to_owned()))?;

                let sz = self.stack.pop();
                self.emit(IR::Shrink(sz));
                self.stack.scope -= 1;

                Ok(None)
            }
            x => todo!("{:?}", x),
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
    s.push_str("mov rbp, rsp\n");

    for i in is {
        s.push_str(i.emit(symbol_table).as_str());
        s.push_str("\n");
    }

    s
}

pub fn compile(ast: &AST) -> Result<String, analysis::CompilationError> {
    let mut compiler = Compiler::new();
    compiler.ast_to_ir(ast)?;
    let vec = compiler.ir;

    red_ln!("Symbol table: {:?}", compiler.symbols);

    Ok(ir_to_asm(vec.as_ref(), &compiler.symbols))
}

#[cfg(test)]
mod tests {

    use super::*;

    fn print_expr(expr: &str) {
        let ast = parser::parse(expr);
        let mut compiler = Compiler::new();
        compiler.ast_to_ir(&ast);
        let is = compiler.ir;

        red_ln!("{:?} => {:?}", ast, is);
        red_ln!("ast_to_ird: ");

        for i in is {
            red_ln!("{}", i.emit(&compiler.symbols));
        }
    }

    #[test]
    fn test_add_mul() {
        print_expr("1 + 2 * 3");
        print_expr("( 1 + 2 ) * 3");
        print_expr("( 4 + 4 ) / 2");
    }
}
