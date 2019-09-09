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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i32),
    BinOp(Box<Expr>, Op, Box<Expr>),
}

// // deep cloning (is it necessary)
// impl Clone for Expr {
//     fn clone(&self) -> Expr {
//         match self {
//             Expr::BinOp(l, op, r) => Expr::BinOp (
//                 l.clone(), *op, r.clone()
//             ),
//             Expr::Num(i) => Expr::Num(*i)
//         }
//     }
// }

pub fn parse_i32(i: &str) -> IResult<&str, Expr> {
    map(digit1, |digit_str: &str| {
        Expr::Num(digit_str.parse::<i32>().unwrap())
    })(i)
}

fn parse_op(i: &str) -> IResult<&str, Op> {
    alt((
        map(tag("+"), |_| Op::Add),
        map(tag("-"), |_| Op::Sub),
        map(tag("*"), |_| Op::Mul),
        map(tag("/"), |_| Op::Div),
        map(tag("^"), |_| Op::Pow),
    ))(i)
}

fn parse_expr(i: &str) -> IResult<&str, Expr> {
    preceded(
        multispace0,
        alt((
            map(
                tuple((parse_i32, preceded(multispace0, parse_op), parse_expr)),
                |(l, op, r)| Expr::BinOp(Box::new(l), op, Box::new(r)),
            ),
            parse_i32,
        )),
    )(i)
}

fn math_expr(e: &Expr) -> String {
    match e {
        Expr::Num(i) => format!("{}", i),
        Expr::BinOp(l, op, r) => {
            format!("({:?}, {}, {})", op, math_expr(l), math_expr(r))
        }
    }
}

fn math_eval(e: &Expr) -> i32 {
    match e {
        Expr::Num(i) => *i,
        Expr::BinOp(l, op, r) => {
            let lv = math_eval(l);
            let rv = math_eval(r);
            match op {
                Op::Add => lv + rv,
                Op::Sub => lv - rv,
                Op::Mul => lv * rv,
                Op::Div => lv / rv,
                Op::Pow => lv.pow(rv as u32),
            }
        }
    }
}

enum Ass {
    Left,
    Right,
}

fn climb_op(op: &Op) -> (u8, Ass) {
    match op {
        Op::Add => (1, Ass::Left),
        Op::Sub => (1, Ass::Left),
        Op::Mul => (2, Ass::Left),
        Op::Div => (2, Ass::Left),
        Op::Pow => (3, Ass::Right),
    }
}

// fn build_expr(l:&Expr, op:&Op, r:&Expr) -> Expr {
//     Expr::BinOp(Box::new(*l), *op, Box::new(*r))
// }

fn climb(e: Expr, min_prec: u8) -> Expr {
    println!("in climb {}, {}", math_expr(&e), min_prec);
    match e.clone() {
        Expr::Num(_) => e,
        Expr::BinOp(l, op, r) => {
            let (prec, ass) = climb_op(&op);

            let mut res = e.clone();
            let mut cur = e.clone();
            while let Expr::BinOp(l, op, r) = cur {
                cur = *r.clone();
                let rhs = climb(
                    cur.clone(),
                    prec + match ass {
                        Ass::Left => 1,
                        Ass::Right => 0,
                    },
                );
                println!("rhs {}", math_expr(&rhs));
                res = Expr::BinOp(Box::new(res), op, Box::new(rhs))
            }

            res
        }
    }
}

// fn test_eq(s: &str, v: i32) {
//     assert_eq!(math_eval(&climb(parse_expr(s).unwrap().1), 0), v);
// }

// #[test]
// fn climb1() {
//     test_eq("1-2+3", 1 - 2 + 3);
// }

// #[test]
// fn climb2() {
//     test_eq("1*2+3", 1 * 2 + 3);
// }

// #[test]
// fn climb3() {
//     test_eq("1*2+3*4-5", 1 * 2 + 3 * 4 - 5);
// }

// #[test]
// fn climb4() {
//     test_eq("2^5", 2i32.pow(5));
// }

// #[test]
// fn climb5() {
//     test_eq("2*3+4+5", 2 * 3 + 4 + 5);
// }

// #[test]
// fn climb6() {
//     test_eq("2*3-4*5-2", 2 * 3 - 4 * 5 - 2);
// }

// #[test]
// fn climb_err() {
//     test_eq("2 + 2 ^ 5 -3", 2 + 2i32.pow(5 - 3));
// }

fn climb_test(s: &str, v: i32) {
    let p = parse_expr(s).unwrap().1;
    println!("{:?}", &p);
    println!("math {}", math_expr(&p));
    let r = climb(p, 0);
    println!("r {:?}", &r);
    println!("math r {}", math_expr(&r));
    println!("eval r {} = {} ", math_eval(&r), v);
}

fn main() {
    // climb_test("2*5+10+10", 2*5+10+10);
    // climb_test("2*5+10*11-1", 2*5+10*11-1);
    // climb_test("2*5+10*11-2+12", 2*5+10*11-1+12);
    // climb_test("1+2*3-4+5", 1 + 2 * 3 - 4 + 5);
    climb_test("1", 1);
    climb_test("1+2", 1 + 2);
}
