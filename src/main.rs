extern crate nom;

use nom::combinator::map_res;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map,
    error::{VerboseError, VerboseErrorKind},
    map_res,
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

pub fn parse_i32(i: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    map_res(digit1, |digit_str: &str| match digit_str.parse::<i32>() {
        Err(e) => Err(VerboseError {
            errors: vec![(digit_str, VerboseErrorKind::Context("not a 32-bit integer"))],
        }),
        Ok(x) => Ok(Expr::Num(x)),
    })(i)
}

fn parse_expr(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
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

fn main() {
    println!("{:?}", parse_expr("1"));
    println!("{:?}", parse_expr("1+1a"));
    println!("{:?}", parse_expr("11111111111111111111111111"));
}
