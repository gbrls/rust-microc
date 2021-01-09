//TODO: Convert from macros to functions, more: http://unhandledexpression.com/general/2019/06/17/nom-5-is-here.html
// Cheatsheet https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md
//FIXME: Nom for some reason is not working with \n, so we map \n to ; and then parse
//TODO: comments
//TODO: Boolean literals

use crate::ast::{Type, AST};
use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
    IResult,
};
use std::str::FromStr;

named!(ignore_ws<&str, &str>, take_while!(|c: char| c.is_whitespace() || c == '\n' || c == '\t'));
named!(number_i32<&str, i32>, map!(digit1, |i| i32::from_str(i).unwrap()));
named!(number<&str, i32>, preceded!(complete!(ignore_ws), number_i32));

fn bool_true(i: &str) -> IResult<&str, bool> {
    map(preceded(complete(ignore_ws), tag("true")), |_| true)(i)
}

fn bool_false(i: &str) -> IResult<&str, bool> {
    map(preceded(complete(ignore_ws), tag("false")), |_| false)(i)
}
fn bool_fn(i: &str) -> IResult<&str, AST> {
    map(alt((bool_true, bool_false)), AST::Bool)(i)
}

fn symbol(i: &str) -> IResult<&str, &str> {
    preceded(complete(ignore_ws), alpha1)(i)
}

fn ptype(i: &str) -> IResult<&str, Type> {
    use strum::IntoEnumIterator;

    map(alt((tag("int"), tag("bool"))), |x| {
        let mut f = None;
        for v in Type::iter() {
            if format!("{}", v).as_str() == x {
                f = Some(v)
            }
        }
        f.expect("Type variant not found")
    })(i)
}

/*
GRAMMAR (may be outdated)

<program> ::= <statement>*

<statement> ::= <block> | <varDecl> | <assignStmt> | <exprStmt>

<block> ::= { <statement>* }
<varDecl> ::= <type> SYMBOL ;
<assignStmt> ::= SYMBOL = <expression> ;
<exprStmt> ::= <expression> ;


<expression> ::= <term>

<term> ::= <factor> (('+' | '-') <factor>)*
<factor> ::= <and_op> (('*' | '/') <and_op>)*
<and_op> ::= <primary> (&& <primary>)*

<primary> ::= NUMBER | SYMBOL | BOOL | ( <term> )

<type> ::= "int" | "bool"

*/

fn program(i: &str) -> IResult<&str, AST> {
    map(many1(statement), AST::Many)(i)
}

fn statement(i: &str) -> IResult<&str, AST> {
    //TODO: maybe remove expression statements?
    alt((while_stmt, if_stmt, block, declare, assign, expr_stmt))(i)
}

fn while_stmt(i: &str) -> IResult<&str, AST> {
    map(
        tuple((preceded(complete(ignore_ws), tag("while")), expr, block)),
        |(_, cond, b)| AST::While(Box::new(cond), Box::new(b)),
    )(i)
}

// this is unsound for C, "if true {...}"" is accepted
// and it's incomplete, "if(x) x = false" is not accepted
fn if_stmt(i: &str) -> IResult<&str, AST> {
    map(
        tuple((
            preceded(complete(ignore_ws), tag("if")),
            expr,
            block,
            opt(else_part),
        )),
        |(_, expr, if_block, else_block)| {
            let eb = match else_block {
                Some(e) => Some(Box::new(e)),
                None => None,
            };
            AST::If(Box::new(expr), Box::new(if_block), eb)
        },
    )(i)
}

fn else_part(i: &str) -> IResult<&str, AST> {
    preceded(preceded(complete(ignore_ws), tag("else")), block)(i)
}

fn block(i: &str) -> IResult<&str, AST> {
    map(
        delimited(
            preceded(complete(ignore_ws), char('{')),
            many0(statement),
            preceded(complete(ignore_ws), char('}')),
        ),
        AST::Block,
    )(i)
}

fn expr_stmt(i: &str) -> IResult<&str, AST> {
    terminated(expr, preceded(ignore_ws, char(';')))(i)
}

fn declare(i: &str) -> IResult<&str, AST> {
    terminated(
        complete(map(
            tuple((preceded(ignore_ws, ptype), symbol)),
            |(t, name)| AST::DeclareVar(name.to_string(), t),
        )),
        preceded(complete(ignore_ws), char(';')),
    )(i)
}

fn assign(i: &str) -> IResult<&str, AST> {
    terminated(
        map(
            tuple((symbol, preceded(complete(ignore_ws), char('=')), expr)),
            |(name, _, e)| AST::AssignVar(name.to_owned(), Box::new(e)),
        ),
        preceded(complete(ignore_ws), char(';')),
    )(i)
}

named!(assign_old<&str, AST>,
    do_parse!(
        name: symbol >>
        preceded!(complete!(ignore_ws), char!('=')) >>
        complete!(ignore_ws) >>
        value: expr >>
        (AST::AssignVar(name.to_owned(), Box::new(value)))
    )
);

named!(expr<&str, AST>, map!(or_op, |x| x));

fn or_op(i: &str) -> IResult<&str, AST> {
    map(
        tuple((
            and_op,
            fold_many0(
                preceded(preceded(complete(ignore_ws), tag("||")), and_op),
                None,
                |acc, n| match acc {
                    Some(a) => Some(AST::BoolOr(Box::new(a), Box::new(n))),
                    None => Some(n),
                },
            ),
        )),
        |(f, r)| match r {
            Some(a) => AST::BoolOr(Box::new(f), Box::new(a)),
            None => f,
        },
    )(i)
}

fn and_op(i: &str) -> IResult<&str, AST> {
    map(
        tuple((
            lesser_op,
            fold_many0(
                preceded(preceded(complete(ignore_ws), tag("&&")), lesser_op),
                None,
                |acc, n| match acc {
                    Some(a) => Some(AST::BoolAnd(Box::new(a), Box::new(n))),
                    None => Some(n),
                },
            ),
        )),
        |(f, r)| match r {
            Some(a) => AST::BoolAnd(Box::new(f), Box::new(a)),
            None => f,
        },
    )(i)
}

fn lesser_op(i: &str) -> IResult<&str, AST> {
    map(
        tuple((
            term,
            fold_many0(
                preceded(preceded(complete(ignore_ws), char('<')), term),
                None,
                |acc, n| match acc {
                    Some(a) => Some(AST::Lesser(Box::new(a), Box::new(n))),
                    None => Some(n),
                },
            ),
        )),
        |(f, r)| match r {
            Some(a) => AST::Lesser(Box::new(f), Box::new(a)),
            None => f,
        },
    )(i)
}

named!(term<&str, AST>,
    do_parse!(
        first: factor >>
        rest: fold_many0! (
            tuple!(
                preceded!(complete!(ignore_ws), alt!(char!('+') | char!('-'))),
                factor
            ),
            first,
            |acc, (op, v) | {
                if op == '+' {
                    AST::Add(Box::new(acc), Box::new(v))
                } else {
                    AST::Sub(Box::new(acc), Box::new(v))
                }
            }
        ) >>
        (rest)
    )
);

named!(factor<&str, AST>,
    do_parse!(
        first: primary >>
        rest: fold_many0! (
            tuple!(
                preceded!(complete!(ignore_ws), alt!(char!('*') | char!('/'))),
                primary
            ),
            first,
            |acc, (op, v) | {
                if op == '*' {
                    AST::Mul(Box::new(acc), Box::new(v))
                } else {
                    AST::Div(Box::new(acc), Box::new(v))
                }
            }
        ) >>
        (rest)
    )
);

named!(primary<&str, AST>,
    preceded!(complete!(ignore_ws),
        alt!(
            bool_fn |
            map!(number, |x| AST::Number(x)) |
            delimited!(
                char!('('),
                expr,
                preceded!(complete!(ignore_ws), char!(')'))
            ) |
            map!(symbol, |x| AST::GetVar(x.to_owned()))
        )
    )
);

pub fn parse(input: &str) -> AST {
    let mut ninput = String::new();

    for l in input.lines() {
        if !l.starts_with("//") && !l.is_empty() {
            ninput.push_str(l);
        }
    }

    red_ln!("new input: {}", ninput);

    let (_rest, ans) = program(ninput.as_str()).unwrap();
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prim() {
        println!("{:?}", primary("   ( 123 )"));
        println!("{:?}", primary("123"));
    }

    #[test]
    fn fact() {
        println!("{:?}", factor("2 * 3 * 4"));
        println!("{:?}", factor("1 * 2 / 3"));
    }

    #[test]
    fn ter() {
        println!("{:?}", term("1 + 2 / 3"));
        println!("{:?}", term("( 1 + 2 ) / 3"));
    }

    #[test]
    fn sym() {
        println!("{:?}", symbol("\n  Hello there"));
        println!("{:?}", symbol("test ."));
        println!("{:?}", symbol("testtt"));
    }

    #[test]
    fn test_assign() {
        println!("{:?}", assign("foo  =    1 + 2 + 3;"));
        println!("{:?}", assign("a=b;"));
    }

    #[test]
    fn test_program() {
        println!("{:?}", program(" a=1  ;   b = a * 2;"));
        println!("{:?}", program("1"));
        println!("{:?}", program("((  1+2  ) * 3+2  )    *4;"));
        println!("{:?}", program("1;4;3;"));

        println!("{:?}", program("bool a; int b; b = 10; b;"));
        println!("{:?}", program("{bool a; {int b; {b = 10;}} b;}"));

        println!(
            "{:?}",
            program("bool t; {  t = true;    bool b = false;   }t;")
        );

        println!("{:?}", block(" { t = true;     bool b; } t;"));
    }

    fn test_newline(i: &str) -> IResult<&str, Vec<AST>> {
        many0(preceded(alt((newline, char(';'))), expr))(i)
    }

    #[test]
    fn test_type() {
        println!("{:?}", ptype("bool a"));
    }

    #[test]
    fn test_decl() {
        println!("{:?}", declare("bool a;"));
        println!("{:?}", declare("int b;"));
    }

    #[test]
    fn test_expr_stmt() {
        println!("{:?}", expr_stmt("1 * 3 + 4 / (4 - 3);"));
    }

    #[test]
    fn test_block() {
        println!("{:?}", block("{int a; a = 10; bool b;}"));
    }

    #[test]
    fn test_bool() {
        println!("{:?}", bool_fn(" true"));
        println!("{:?}", bool_fn("   false;"));
    }

    #[test]
    fn test_if() {
        println!("{:?}", if_stmt("if (2 * 3) {int a; a = 20;}"));
        println!(
            "{:?}",
            if_stmt("if (2 * 3) {int a; a = 20;} else { int b; b = 5;}")
        );
    }

    #[test]
    fn test_else() {
        println!("{:?}", else_part(" else {int a; a = 20;}"));
    }

    #[test]
    fn test_and() {
        println!("{:?}", and_op(" 1 + 2 && 3 * 4 && true"));
        println!("{:?}", and_op(" 1 + 2 / 3 * 4"));
    }

    #[test]
    fn test_or() {
        println!("{:?}", or_op(" 1 + 2 && 3 * 4 && true"));
        println!("{:?}", or_op(" 1 + 2 / 3 * 4"));
        println!("{:?}", or_op(" true || false && true"));
        println!("{:?}", or_op(" true || 1 + 2 && 3 * 4 && true || false"));
        println!("{:?}", parse("if (true && false) { int a; }"));
    }

    #[test]
    fn test_while() {
        println!("{:?}", while_stmt("while (1 / 2) {int a; a = 10;}"))
    }

    #[test]
    fn test_lesser() {
        println!("{:?}", lesser_op("(1 / 2) < a;"))
    }

    #[test]
    fn test_parse() {
        println!("{:?}", parse("1 + 2 * 3").eval());
        println!("{:?}", parse("( 1 + 2 ) * 3").eval());
    }
}
