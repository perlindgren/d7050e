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

use crust::ast::{Op, Expr};


pub fn parse_i32(i: &str) -> IResult<&str, i32> {
    map(digit1, |digit_str: &str| digit_str.parse::<i32>().unwrap())(i)
}

fn parse_op(i: &str) -> IResult<&str, Op> {
    alt((
        map(tag("=="), |_| Op::Eq),
        map(tag("!="), |_| Op::Neq),
        map(tag("**"), |_| Op::Pow),
        map(tag("&&"), |_| Op::And),
        map(tag("||"), |_| Op::Or),
        map(tag("+"), |_| Op::Add),
        map(tag("-"), |_| Op::Sub),
        map(tag("*"), |_| Op::Mul),
        map(tag("/"), |_| Op::Div),
        map(tag("!"), |_| Op::Not),
    ))(i)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(i32),
    Par(Vec<Token>),
    Op(Op),
}

fn parse_terminal(i: &str) -> IResult<&str, Token> {
    alt((
        map(parse_i32, |v| Token::Num(v)),
        map(parse_par(parse_tokens), |tokens| Token::Par(tokens)),
    ))(i)
}

fn parse_token(i: &str) -> IResult<&str, Token> {
    preceded(
        multispace0,
        alt((map(parse_op, |op| Token::Op(op)), parse_terminal)),
    )(i)
}

fn parse_tokens(i: &str) -> IResult<&str, Vec<Token>> {
    many1(parse_token)(i)
}

fn compute_atom(t: &mut Peekable<Iter<Token>>) -> Expr {
    match t.next() {
        Some(Token::Num(i)) => Expr::Num(*i),
        Some(Token::Par(v)) => climb(&mut v.iter().peekable(), 0),
        Some(Token::Op(op)) => Expr::UnaryOp(*op, Box::new(climb(t, 4))), // assume highest precedence
        _ => panic!("error in compute atom"),
    }
}

fn climb(t: &mut Peekable<Iter<Token>>, min_prec: u8) -> Expr {
    let mut result = compute_atom(t);

    loop {
        match t.peek() {
            Some(Token::Op(op)) => {
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
                result = Expr::BinOp(*op, Box::new(result), Box::new(rhs))
            }
            _ => {
                break;
            }
        }
    }
    result
}

fn test(s: &str, v: i32) {
    match parse_tokens(s) {
        Ok(("", t)) => {
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

fn main() {
    test("- -1 + + 1", - -1 + 1);  // rust does not allow + as a unary op (I do ;)
    test("(-1-1)+(-1+3)", (-1 - 1) + (-1) + 3);
    // just to check that right associative works (you don't need to implement pow)
    test("2+3**2**3*5+1", 2 + 3i32.pow(2u32.pow(3)) * 5 + 1);
    test("(12*2)/3-4", (12 * 2) / 3 - 4);
    test("1*2+3", 1 * 2 + 3);
    // just to check that we get a parse error
    test("1*2+3+3*21-a12+2", 1 * 2 + 3 + 3 * 21 - 12 + 2);
}

// helpers
fn parse_par<'a, O, F, E>(
    inner: F,
) -> impl Fn(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    // delimited allows us to split up the input
    // cut allwos us to consume the input (and prevent backtracking)
    delimited(char('('), preceded(multispace0, inner), cut(char(')')))
}

fn math_eval(e: &Expr) -> i32 {
    match e {
        Expr::Num(i) => *i,
        Expr::BinOp(op, l, r) => {
            let lv = math_eval(l);
            let rv = math_eval(r);
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
            let e = math_eval(e);
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
