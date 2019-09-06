extern crate nom;

use nom::combinator::map_res;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map,
    error::{context, ErrorKind, VerboseError, VerboseErrorKind},
    sequence::{preceded, tuple},
    Err, IResult,
};

use nom_locate::{position, LocatedSpan};

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
}

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Num(Span<'a>, i32),
    BinOp(Span<'a>, Box<Expr<'a>>, Op, Box<Expr<'a>>),
}

// type IResult<I, O, E = u32> = Result<(I, O), Err<I, E>>;

pub fn parse_i32(i: Span) -> IResult<Span, Expr> {
    map(digit1, |digit_str: Span| {
        Expr::Num(digit_str, digit_str.fragment.parse::<i32>().unwrap())
    })(i)
    // Err(Err::Error((i, ErrorKind::Alpha)))
}

fn parse_expr(i: Span) -> IResult<Span, Expr> {
    preceded(
        multispace0,
        alt((
            map(
                tuple((parse_i32, preceded(multispace0, tag("+")), parse_expr)),
                |(l, _, r)| Expr::BinOp(i, Box::new(l), Op::Add, Box::new(r)),
            ),
            parse_i32,
        )),
    )(i)
}

// cargo test
#[test]
fn test_parse_i32_1() {
    let (rest, expr) = parse_expr(Span::new("1")).unwrap();

    // check that we are at the end of the input
    assert_eq!(
        rest,
        Span {
            offset: 1,
            line: 1,
            fragment: "",
            extra: (),
        },
    );

    // check that the expression is parsed correctly
    assert_eq!(
        expr,
        Expr::Num(
            Span {
                offset: 0,
                line: 1,
                fragment: "1",
                extra: (),
            },
            1
        )
    );
}


fn main() {
    let (a, b) = parse_expr(Span::new("1")).unwrap();
    println!("{:?}", parse_expr(Span::new("1")));
    println!("{:?}", parse_expr(Span::new("1+2 + 3")));
    println!("{:?}", parse_expr(Span::new("   1+ 1a")));
    println!("{:?}", parse_expr(Span::new("11111111111111111111111111")));
}
