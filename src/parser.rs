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

named!(ignore_ws<&str, &str>, take_while!(|c: char| c.is_whitespace() || c == '\n'));
named!(number_i32<&str, i32>, map!(digit1, |i| i32::from_str(i).unwrap()));
named!(number<&str, i32>, preceded!(complete!(ignore_ws), number_i32));

fn symbol(i: &str) -> IResult<&str, &str> {
    preceded(complete(ignore_ws), alpha1)(i)
}

//FIXME: maybe this is a problem
fn delim(i: &str) -> IResult<&str, Vec<char>> {
    many1(alt((newline, char(';'))))(i)
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
<statement> ::= <assignStmt> | <exprStmt> | <varDecl>
<assignStmt> ::= SYMBOL = <expression> ;
<varDecl> ::= <type> SYMBOL ;

<type> ::= "int" | "bool"

<expression> ::= <term>
<term> ::= <factor> (('+' | '-') <factor>)*
<factor> ::= <primary> (('*' | '/') <primary>)*
<primary> ::= NUMBER | SYMBOL | ( <term> )

*/

fn program(i: &str) -> IResult<&str, AST> {
    map(many1(statement), AST::Many)(i)
}

fn statement(i: &str) -> IResult<&str, AST> {
    //TODO: maybe remove expression statements?
    alt((declare, assign, expr_stmt))(i)
}

fn expr_stmt(i: &str) -> IResult<&str, AST> {
    terminated(expr, preceded(ignore_ws, char(';')))(i)
}

fn declare(i: &str) -> IResult<&str, AST> {
    terminated(
        complete(map(
            tuple((preceded(ignore_ws, ptype), symbol)),
            |(t, name)| AST::DeclareGlobal(name.to_string(), t),
        )),
        preceded(complete(ignore_ws), char(';')),
    )(i)
}

fn assign(i: &str) -> IResult<&str, AST> {
    terminated(
        map(
            tuple((symbol, preceded(complete(ignore_ws), char('=')), expr)),
            |(name, _, e)| AST::AssignGlobal(name.to_owned(), Box::new(e)),
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
        (AST::AssignGlobal(name.to_owned(), Box::new(value)))
    )
);

named!(expr<&str, AST>, map!(term, |x| x));

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
            map!(number, |x| AST::Number(x)) |
            delimited!(
                char!('('),
                term,
                preceded!(complete!(ignore_ws), char!(')'))
            ) |
            map!(symbol, |x| AST::GetGlobal(x.to_owned()))
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

    let (_, ans) = program(ninput.as_str()).unwrap();
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
    fn test_parse() {
        println!("{:?}", parse("1 + 2 * 3").eval());
        println!("{:?}", parse("( 1 + 2 ) * 3").eval());
    }
}
