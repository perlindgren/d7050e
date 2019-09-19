extern crate nom;

use std::iter::Peekable;
use std::slice::Iter;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{digit1, multispace0},
    combinator::{cut, map},
    error::ParseError,
    multi::many1,
    sequence::{delimited, preceded},
    IResult,
};

use crate::ast::{Expr, Op, Span, SpanExpr};

pub fn parse_i32(i: Span) -> IResult<Span, (Span, i32)> {
    map(digit1, |digit_str: Span| {
        (digit_str, digit_str.fragment.parse::<i32>().unwrap())
    })(i)
}

fn parse_op(i: Span) -> IResult<Span, (Span, Op)> {
    alt((
        map(tag("=="), |s| (s, Op::Eq)),
        map(tag("!="), |s| (s, Op::Neq)),
        map(tag("**"), |s| (s, Op::Pow)),
        map(tag("&&"), |s| (s, Op::And)),
        map(tag("||"), |s| (s, Op::Or)),
        map(tag("+"), |s| (s, Op::Add)),
        map(tag("-"), |s| (s, Op::Sub)),
        map(tag("*"), |s| (s, Op::Mul)),
        map(tag("/"), |s| (s, Op::Div)),
        map(tag("!"), |s| (s, Op::Not)),
    ))(i)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    Num(i32),
    Par(Vec<SpanToken<'a>>),
    Op(Op),
}

type SpanToken<'a> = (Span<'a>, Token<'a>);

fn parse_terminal(i: Span) -> IResult<Span, SpanToken> {
    alt((
        map(parse_i32, |(s, v)| (s, Token::Num(v))),
        map(parse_par(parse_tokens), |(s, tokens)| {
            (s, Token::Par(tokens))
        }),
    ))(i)
}

fn parse_token(i: Span) -> IResult<Span, SpanToken> {
    preceded(
        multispace0,
        alt((map(parse_op, |(s, op)| (s, Token::Op(op))), parse_terminal)),
    )(i)
}

// I think the outer span is wrong
fn parse_tokens(i: Span) -> IResult<Span, (Span, Vec<SpanToken>)> {
    map(many1(parse_token), |tokens| (i, tokens))(i)
}

fn compute_atom<'a>(t: &mut Peekable<Iter<SpanToken<'a>>>) -> SpanExpr<'a> {
    match t.next() {
        Some((s, Token::Num(i))) => (*s, Expr::Num(*i)),
        Some((_, Token::Par(v))) => climb(&mut v.iter().peekable(), 0),
        Some((s, Token::Op(op))) => (*s, Expr::UnaryOp(*op, Box::new(climb(t, 4)))), // assume highest precedence
        _ => panic!("error in compute atom"),
    }
}

fn climb<'a>(
    t: &mut Peekable<Iter<SpanToken<'a>>>,
    min_prec: u8,
) -> SpanExpr<'a> {
    let mut result: SpanExpr = compute_atom(t);

    loop {
        match t.peek() {
            Some((s, Token::Op(op))) => {
                let (prec, ass) = get_prec(op);
                if prec < min_prec {
                    break;
                };
                let next_prec = prec
                    + match ass {
                        Ass::Left => 1,
                        _ => 0,
                    };
                t.next();
                let rhs = climb(t, next_prec);
                result = (*s, Expr::BinOp(*op, Box::new(result), Box::new(rhs)))
            }
            _ => {
                break;
            }
        }
    }
    result
}

pub fn test(s: &str, v: i32) {
    match parse_tokens(Span::new(s)) {
        Ok((Span { fragment: "", .. }, (_, t))) => {
            let mut t = t.iter().peekable();
            println!("{:?}", &t);
            let e = climb(&mut t, 0);
            println!("{:?}", &e);
            println!("eval {} {}", math_eval(&e), v);
            assert_eq!(math_eval(&e), v);
        }
        Ok((s, t)) => println!(
            "parse incomplete, \n parsed tokens \t{:?}, \n remaining \t{:?}",
            t, s
        ),
        Err(err) => println!("{:?}", err),
    }
}

// helpers
fn parse_par<'a, O, F, E>(
    inner: F,
) -> impl Fn(Span<'a>) -> IResult<Span<'a>, O, E>
where
    F: Fn(Span<'a>) -> IResult<Span<'a>, O, E>,
    E: ParseError<Span<'a>>,
{
    // delimited allows us to split up the input
    // cut allwos us to consume the input (and prevent backtracking)
    delimited(char('('), preceded(multispace0, inner), cut(char(')')))
}

fn math_eval(e: &SpanExpr) -> i32 {
    match e.clone().1 {
        Expr::Num(i) => i,
        Expr::BinOp(op, l, r) => {
            let lv = math_eval(&l);
            let rv = math_eval(&r);
            match op {
                Op::Add => lv + rv,
                Op::Sub => lv - rv,
                Op::Mul => lv * rv,
                Op::Div => lv / rv,
                Op::Pow => lv.pow(rv as u32),
                _ => unimplemented!(),
            }
        }
        Expr::UnaryOp(op, e) => {
            let e = math_eval(&e);
            match op {
                Op::Add => e,
                Op::Sub => -e,
                _ => unimplemented!(),
            }
        }
        _ => unimplemented!(),
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Ass {
    Left,
    Right,
}

fn get_prec(op: &Op) -> (u8, Ass) {
    match op {
        Op::Add => (1, Ass::Left),
        Op::Sub => (1, Ass::Left),
        Op::Mul => (2, Ass::Left),
        Op::Div => (2, Ass::Left),
        Op::Pow => (3, Ass::Right),
        _ => unimplemented!(),
    }
}
