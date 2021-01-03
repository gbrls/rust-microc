use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum CompilationError {
    VariableNotDeclared(String),
    IncompatibleTypes,
}
impl std::fmt::Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid program, can't compile: {:?}", self)
    }
}

pub fn vars(ast: &AST, symbols: &HashMap<String, Type>) -> Result<(), CompilationError> {
    use AST::*;

    let eval_branch = |a, b| -> Result<(), CompilationError> {
        let _ = vars(a, symbols)?;
        let _ = vars(b, symbols)?;

        Ok(())
    };

    match ast {
        Number(_) => Ok(()),
        Add(a, b) => eval_branch(a, b),
        Sub(a, b) => eval_branch(a, b),
        Mul(a, b) => eval_branch(a, b),
        Div(a, b) => eval_branch(a, b),
        Many(v) => {
            for e in v {
                let _ = vars(e, symbols)?;
            }
            Ok(())
        }
        DeclareGlobal(_, _) => Ok(()),
        AssignGlobal(name, e) => {
            if symbols.contains_key(name) {
                vars(e, symbols)
            } else {
                Err(CompilationError::VariableNotDeclared(name.to_owned()))
            }
        }
        GetGlobal(name) => {
            if symbols.contains_key(name) {
                Ok(())
            } else {
                Err(CompilationError::VariableNotDeclared(name.to_owned()))
            }
        }

        Block(v) => vars(&Many(v.to_owned()), symbols),
    }
}

pub fn types(ast: &AST, symbols: &HashMap<String, Type>) -> Result<Option<Type>, CompilationError> {
    use CompilationError::*;
    use Type::*;
    use AST::*;

    let eval_branch = |a, b| -> Result<(Option<Type>, Option<Type>), CompilationError> {
        let ta = types(a, symbols)?;
        let tb = types(b, symbols)?;

        Ok((ta, tb))
    };

    match ast {
        Number(_) => Ok(Some(INT)),
        Add(a, b) => match eval_branch(a, b) {
            Ok((Some(INT), Some(INT))) => Ok(Some(INT)),
            _ => Err(IncompatibleTypes),
        },
        Sub(a, b) => match eval_branch(a, b) {
            Ok((Some(INT), Some(INT))) => Ok(Some(INT)),
            _ => Err(IncompatibleTypes),
        },
        Mul(a, b) => match eval_branch(a, b) {
            Ok((Some(INT), Some(INT))) => Ok(Some(INT)),
            _ => Err(IncompatibleTypes),
        },
        Div(a, b) => match eval_branch(a, b) {
            Ok((Some(INT), Some(INT))) => Ok(Some(INT)),
            _ => Err(IncompatibleTypes),
        },

        Many(v) => {
            for e in v {
                let _ = types(e, symbols)?;
            }

            Ok(None)
        }

        Block(v) => types(&Many(v.to_owned()), symbols),

        DeclareGlobal(_, _) => Ok(None),
        GetGlobal(name) => Ok(Some(*symbols.get(name).unwrap())), // this unwrap will never fail if we check it in a previous pass, and we did so.
        AssignGlobal(name, e) => {
            let a = *symbols.get(name).unwrap();
            let b = types(e, symbols)?;

            match b {
                None => Err(IncompatibleTypes),
                Some(x) => {
                    if x == a {
                        Ok(Some(x))
                    } else {
                        Err(IncompatibleTypes)
                    }
                }
            }
        }
    }
}
