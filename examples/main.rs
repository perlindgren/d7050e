extern crate nom;

use nom::combinator::map_res;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map,
    error::{context, VerboseError, VerboseErrorKind},
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(i32),
    BinOp(Box<Expr>, Op, Box<Expr>),
}

pub fn parse_i32(i: &str) -> IResult<&str, Expr> {
    map(digit1, |digit_str: &str| {
        Expr::Num(digit_str.parse::<i32>().unwrap())
    })(i)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    preceded(
        multispace0,
        alt((
            map(
                tuple((parse_i32, preceded(multispace0, tag("+")), parse_expr)),
                |(l, _, r)| Expr::BinOp(Box::new(l), Op::Add, Box::new(r)),
            ),
            parse_i32,
        )),
    )(input)
}

// cargo test
#[test]
fn test_parse_i32_1() {
    let res = parse_expr("2");
    assert!(res == Ok(("", Expr::Num(1))))
}

#[test]
fn test_parse_i32_2() {
    let _ = parse_expr("1a").is_ok();
}

fn main() {
    println!("{:?}", parse_expr("1"));
    println!("{:?}", parse_expr("1+2 + 3"));
    println!("{:?}", parse_expr("   1+ 1a"));
    println!("{:?}", parse_expr("11111111111111111111111111"));
}
