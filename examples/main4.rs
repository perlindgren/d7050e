extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Num(i32),
    Op(Op),
}

pub fn parse_i32(i: &str) -> IResult<&str, Token> {
    map(digit1, |digit_str: &str| {
        Token::Num(digit_str.parse::<i32>().unwrap())
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

fn parse_expr(i: &str) -> IResult<&str, Vec<Token>> {
    preceded(
        multispace0,
        alt((
            map(
                tuple((parse_i32, preceded(multispace0, parse_op), parse_expr)),
                |(l, op, r)| {
                    let mut v = Vec::new();
                    v.push(l);
                    v.push(Token::Op(op));
                    v.extend(r);
                    v
                },
            ),
            map(parse_i32, |i| {
                let mut v = Vec::new();
                v.push(i);
                v
            }),
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

#[derive(Debug, Copy, Clone, PartialEq)]
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

// boilerplate
// compute_expr(min_prec):
//   result = compute_atom()

//   while cur token is a binary operator with precedence >= min_prec:
//     prec, assoc = precedence and associativity of current token
//     if assoc is left:
//       next_min_prec = prec + 1
//     else:
//       next_min_prec = prec
//     rhs = compute_expr(next_min_prec)
//     result = compute operator(result, rhs)

//   return result

fn token_to_expr(t: Token) -> Expr {
    match t {
        Token::Num(i) => Expr::Num(i),
        _ => panic!(),
    }
}

fn climb(mut v: Vec<Token>, min_prec: u8) -> (Expr, Vec<Token>) {
    println!("in climb {:?}, {}", v, min_prec);
    let t = v.pop().unwrap();
    let mut result = token_to_expr(t);
    loop {
        match v.pop() {
            Some(Token::Num(_)) => {
                println!("break num");
                break;
            }
            Some(Token::Op(op)) => {
                println!("result {:?}, op {:?}, v:{:?}", result, op, v);
                let (prec, assoc) = climb_op(&op);
                if prec < min_prec {
                    println!("break prec");
                    break;
                } else {
                    println!("push");
                    let next_min_prec =
                        if assoc == Ass::Left { 1 + prec } else { prec };
                    let (rhs, v_rest) = climb(v.clone(), next_min_prec);
                    v = v_rest;
                    println!("return from call, rhs {:?}, v {:?}", rhs, v);
                    println!("current result {:?}", result);
                    result = Expr::BinOp(Box::new(result), op, Box::new(rhs));
                    println!("new result {:?}", result);
                }
            }

            _ => {
                println!("reaced end");
                break;
            } // reached end
        }
    }
    (result, v)
}

fn test_eq(s: &str, v: i32) {
    let mut p = parse_expr(s).unwrap().1;
    println!("{:?}", p);
    p.reverse();
    let e = climb(p, 0);
    println!("{:?}", e);

    println!("e = {}, v = {}", math_eval(&e.0), v);
}

fn main() {
    test_eq("1 + 2", 1 + 2);
    test_eq("1 + 2 * 3", 1 + 2 * 3);
    test_eq("3 * 4 + 5", 3 * 4 + 5);

    //     // climb_test("2*5+10+10", 2*5+10+10);
    //     // climb_test("2*5+10*11-1", 2*5+10*11-1);
    //     // climb_test("2*5+10*11-2+12", 2*5+10*11-1+12);
    //     // climb_test("1+2*3-4+5", 1 + 2 * 3 - 4 + 5);
    //     climb_test("1", 1);
    //     climb_test("1+2", 1 + 2);
}

// // #[test]
// // fn climb1() {
// //     test_eq("1-2+3", 1 - 2 + 3);
// // }

// // #[test]
// // fn climb2() {
// //     test_eq("1*2+3", 1 * 2 + 3);
// // }

// // #[test]
// // fn climb3() {
// //     test_eq("1*2+3*4-5", 1 * 2 + 3 * 4 - 5);
// // }

// // #[test]
// // fn climb4() {
// //     test_eq("2^5", 2i32.pow(5));
// // }

// // #[test]
// // fn climb5() {
// //     test_eq("2*3+4+5", 2 * 3 + 4 + 5);
// // }

// // #[test]
// // fn climb6() {
// //     test_eq("2*3-4*5-2", 2 * 3 - 4 * 5 - 2);
// // }

// // #[test]
// // fn climb_err() {
// //     test_eq("2 + 2 ^ 5 -3", 2 + 2i32.pow(5 - 3));
// // }

// fn climb_test(s: &str, v: i32) {
//     let p = parse_expr(s).unwrap().1;
//     println!("{:?}", &p);
//     println!("math {}\n", math_expr(&p));
//     let r = climb(p, 0);
//     println!("r {:?}", &r);
//     println!("math r {}", math_expr(&r));
//     println!("eval r {} = {} ", math_eval(&r), v);
// }
