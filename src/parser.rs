//TODO: Convert from macros to functions, more: http://unhandledexpression.com/general/2019/06/17/nom-5-is-here.html
// Cheatsheet https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md

use crate::ast::AST;
use nom::{character::complete::*, combinator::*, sequence::*, IResult};
use std::str::FromStr;

named!(ignore_ws<&str, &str>, take_while!(|c: char| c.is_whitespace() || c == '\n'));

// This is more "idiomatic nom", does the same as the previous and then converts the input to i32
named!(number_i32<&str, i32>, map!(digit1, |i| i32::from_str(i).unwrap()));
named!(number<&str, i32>, preceded!(complete!(ignore_ws), number_i32));

//named!(symbol<&str, &str>,
//    preceded!(
//        complete!(ignore_ws),
//        take_while1!(|c: char| c.is_alphabetic())
//    )
//);
fn symbol(i: &str) -> IResult<&str, &str> {
    preceded(complete(ignore_ws), alpha0)(i)
}

// <statement> ::= assignStmt | (todo)
// <assign_stmt> ::= SYMBOL '=' <expression> ';'
// <expression> ::= <term>
// <term> ::= <factor> (('+' | '-') <factor>)*
// <factor> ::= <primary> (('*' | '/') <primary>)*
// <primary> ::= NUMBER | ( <term> )

fn assign(i: &str) -> IResult<&str, AST> {
    map(
        tuple((symbol, preceded(complete(ignore_ws), char('=')), expr)),
        |(name, _, e)| AST::AssignGlobal(name.to_owned(), Box::new(e)),
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
            map!(symbol, |x| AST::GetGlobal(x.to_owned())) |
            delimited!(
                char!('('),
                term,
                preceded!(complete!(ignore_ws), char!(')'))
            )
        )
    )
);

pub fn parse(input: &str) -> AST {
    let (_, ans) = expr(input).unwrap();
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
        println!("{:?}", assign("foo  =    1 + 2 + 3"));
        println!("{:?}", assign("a=b"));
    }

    #[test]
    fn test_parse() {
        println!("{:?}", parse("1 + 2 * 3").eval());
        println!("{:?}", parse("( 1 + 2 ) * 3").eval());
    }
}
