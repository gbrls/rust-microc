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

        //TODO: maybe forbid global redeclatation
        DeclareVar(_, _) => Ok(None),

        GetVar(name) => match symbols.get(name) {
            Some(t) => Ok(Some(t.to_owned())),
            None => Err(VariableNotDeclared(name.to_owned())),
        },
        AssignVar(name, e) => {
            let err = VariableNotDeclared(name.to_owned());
            let a = *symbols.get(name).ok_or(err)?;
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
