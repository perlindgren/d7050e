extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map,
    error,
    sequence::{preceded, tuple},
    Err,
};

use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
pub struct Error<'a>(Span<'a>, Option<Span<'a>>, ErrorKind);
type IResult<'a, I, O, E = Error<'a>> = Result<(I, O), Err<E>>;

impl<'a> error::ParseError<Span<'a>> for Error<'a> {
    fn from_error_kind(input: Span<'a>, kind: error::ErrorKind) -> Self {
        Error(input, None, ErrorKind::Nom(kind))
    }

    fn append(_: Span<'a>, _: error::ErrorKind, other: Self) -> Self {
        other
    }
}

#[derive(Debug)]
enum ErrorKind {
    ParseIntError(std::num::ParseIntError),
    Nom(error::ErrorKind),
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
}

type SpanOp<'a> = (Span<'a>, Op);

fn parse_op(i: Span) -> IResult<Span, SpanOp> {
    alt((
        map(tag("+"), |s| (s, Op::Add)),
        map(tag("-"), |s| (s, Op::Sub)),
    ))(i)
}

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Num(i32),
    BinOp(Box<SpanExpr<'a>>, SpanOp<'a>, Box<SpanExpr<'a>>),
}

type SpanExpr<'a> = (Span<'a>, Expr<'a>);

pub fn parse_i32<'a>(i: Span<'a>) -> IResult<Span<'a>, SpanExpr> {
    let (i, digits) = digit1(i)?;
    match digits.fragment.parse() {
        Ok(int) => Ok((i, (digits, Expr::Num(int)))),
        Err(e) => Err(Err::Failure(Error(i, Some(digits), ErrorKind::ParseIntError(e)))),
    }
}

fn parse_expr(i: Span) -> IResult<Span, SpanExpr> {
    alt((
        map(
            tuple((parse_i32, preceded(multispace0, parse_op), parse_expr_ms)),
            |(l, op, r)| (i, Expr::BinOp(Box::new(l), op, Box::new(r))),
        ),
        parse_i32,
    ))(i)
}

fn parse_expr_ms(i: Span) -> IResult<Span, SpanExpr> {
    preceded(multispace0, parse_expr)(i)
}

// dumps a Span into a String
fn dump_span(s: &Span) -> String {
    format!(
        "[line :{:?}, col:{:?}, {:?}]",
        s.line,
        s.get_column(),
        s.fragment
    )
}

// dumps a SpanExpr into a String
fn dump_expr(se: &SpanExpr) -> String {
    let (s, e) = se;
    match e {
        Expr::Num(_) => dump_span(s),
        Expr::BinOp(l, (sop, _), r) => {
            format!("<{} {} {}>", dump_expr(l), dump_span(sop), dump_expr(r))
        }
    }
}

fn main() {
    let i = "\n    1+2+10000- \n3";
    // uncomment below for an error example
    let i = "\n    1+200000000000000000+a10000- \n3";
    let pe = parse_expr_ms(Span::new(i));
    println!("pe: {:?}\n", pe);
    match pe {
        Ok((_, (s, e))) => {
            println!(
                "ok, span for expression: {:?}, \n\tline: {:?}, \n\tcolumn: {:?}\n",
                s,
                s.line,
                s.get_column()
            );
            println!("raw e: {:?}\n", &e);
            println!("pretty e: {}\n", dump_expr(&(s, e)));
        }
        Err(Err::Failure(Error(_, Some(s), err))) => {
            println!(
                "{:?} error at:\n\tline: {:?}\n\tcolumn: {:?}\n\tValue: {:?}\n",
                err,
                s.line,
                s.get_column(),
                s.fragment,
            );
            println!("raw s: {:?}", &s);
        }
        Err(err) => Err(err).unwrap(),
    }
}

// In this example, we have a `parse_expr_ms` is the "top" level parser.
// It consumes white spaces, allowing the location information to reflect the exact
// positions in the input file.
//
// The dump_expr will create a pretty printing of the expression with spans for
// each terminal. This will be useful for later for precise type error reporting.
//
// The extra field is not used, it can be used for metadata, such as filename.
