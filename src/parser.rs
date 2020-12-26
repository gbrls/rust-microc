use crate::ast::AST;
use nom::character::complete::digit1;
use std::str::FromStr;

named!(ignore_ws<&str, &str>, take_while!(|c: char| c.is_whitespace()));

// This is more "idiomatic nom", does the same as the previous and then converts the input to i32
named!(number_i32<&str, i32>, map!(digit1, |i| i32::from_str(i).unwrap()));

named!(number<&str, i32>, preceded!(complete!(ignore_ws), number_i32));

//  The grammar for this parser is the following:
// <term> ::= <factor> (('+' | '-') <factor>)*
// <factor> ::= <primary> (('*' | '/') <primary>)*
// <primary> ::= NUMBER | ( <term> )

named!(primary<&str, AST>,
    preceded!(complete!(ignore_ws),
        alt!(
            map!(number, |x| AST::Number(x)) |
            delimited!(
                char!('('),
                term,
                preceded!(complete!(ignore_ws), char!(')'))
            )
        )
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

pub fn parse(input: &str) -> AST {
    let (_, ans) = term(input).unwrap();
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
    fn test_parse() {
        println!("{:?}", parse("1 + 2 * 3").eval());
        println!("{:?}", parse("( 1 + 2 ) * 3").eval());
    }
}
