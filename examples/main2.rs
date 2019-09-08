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
        Expr::BinOp(l, op, r) => format!("({} {:?} {})", math_expr(l), op, math_expr(r)),
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
                Op::Pow => lv ^ rv,
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

fn test_clone(e: Expr) -> Expr {
    Expr::BinOp(Box::new(e.clone()), Op::Add, Box::new(e))
}

// fn climb(e: Expr) -> Expr {
//     println!("climb {:?}", &e);
//     match e.clone() {
//         Expr::Num(_) => e,
//         Expr::BinOp(l, op, r) => {
//             let (prec, ass) = climb_op(&op);

//             // lookahead
//             let r_new = climb(*r);
//             match r_new.clone() {
//                 Expr::Num(_) => Expr::BinOp(Box::new(*l), op, Box::new(r_new)),
//                 Expr::BinOp(r_l, r_op, r_r) => {
//                     let (r_prec, r_ass) = climb_op(&r_op);
//                     println!(
//                         "-- l: {:?}, r: {:?}, r_prec {}, prec {}
//                     ",
//                         r_l, r_r, prec, r_prec
//                     );
//                     if r_prec
//                         < prec
//                             + match r_ass {
//                                 Ass::Left => 1,
//                                 Ass::Right => 0,
//                             }
//                     {
//                         // swap
//                         println!("swap");
//                         let new_l = Expr::BinOp(Box::new(*l), op, Box::new(*r_l));
//                         let new_top = Expr::BinOp(Box::new(new_l), r_op, Box::new(*r_r));

//                         new_top
//                     } else {
//                         Expr::BinOp(Box::new(*l), op, Box::new(r_new))
//                     }
//                 }
//             }
//         }
//     }
// }

fn climb(e: Expr) -> Expr {
    println!("climb {:?}", &e);
    match e.clone() {
        Expr::Num(_) => e,
        Expr::BinOp(l, op, r) => {
            let (prec, ass) = climb_op(&op);

            // lookahead
            let e = match *r.clone() {
                Expr::Num(_) => e,
                Expr::BinOp(r_l, r_op, r_r) => {
                    let (r_prec, r_ass) = climb_op(&r_op);
                    println!(
                        "-- l: {:?}, r: {:?}, r_prec {}, prec {}
                    ",
                        r_l, r_r, prec, r_prec
                    );
                    if r_prec
                        < prec
                            + match r_ass {
                                Ass::Left => 1,
                                Ass::Right => 0,
                            }
                    {
                        // swap
                        println!("swap");
                        let new_l = Expr::BinOp(Box::new(*l), op, Box::new(*r_l));
                        let new_top = Expr::BinOp(Box::new(new_l), r_op, Box::new(*r_r));

                        climb(new_top)
                    } else {
                        e
                    }
                }
            };
            
            match e {
                Expr::Num(_) => e,
                Expr::BinOp(l, op, r) => Expr::BinOp(l, op, Box::new(climb(*r)))
            }
        }
    }
}

fn test_eq(s:&str, v:i32) {
    assert_eq!(math_eval(&climb(parse_expr(s).unwrap().1)), v);    
}

#[test]
fn climb1() {
    test_eq("1-2+3", 1-2+3);
}

#[test]
fn climb2() {
    test_eq("1*2+3", 1*2+3);
}

#[test]
fn climb3() {
    test_eq("1*2+3*4-5", 1*2+3*4-5);
}

fn main() {
    let p = parse_expr("3*2+5").unwrap().1;
    println!("{:?}", &p);
    println!("math {}", math_expr(&p));
    let r = climb(p);
    println!("r {:?}", &r);
    println!("math r {}", math_expr(&r));
    println!("eval r {} = {} ", math_eval(&r), 3 * 2 + 5);

    println!();
    let p = parse_expr("3+2*5").unwrap().1;
    println!("{:?}", &p);
    println!("math {}", math_expr(&p));
    let r = climb(p);
    println!("r {:?}", &r);
    println!("math r {}", math_expr(&r));
    println!("eval r {} = {} ", math_eval(&r), 3 + 2 * 5);

    println!();
    let p = parse_expr("3+2*5+27").unwrap().1;
    println!("{:?}", &p);
    println!("math {}", math_expr(&p));
    let r = climb(p);
    println!("r {:?}", &r);
    println!("math r {}", math_expr(&r));
    println!("eval r {} = {} ", math_eval(&r), 3 + 2 * 5 + 27);

    println!();
    let p = parse_expr("2*5+11*27+13").unwrap().1;
    println!("{:?}", &p);
    println!("math {}", math_expr(&p));
    let r = climb(p);
    println!("r {:?}", &r);
    println!("math r {}", math_expr(&r));
    println!("eval r {} = {} ", math_eval(&r), 2 * 5 + 11 * 27 + 13);

    println!();
    let p = parse_expr("1-2-3").unwrap().1;
    println!("{:?}", &p);
    println!("math {}", math_expr(&p));
    let r = climb(p);
    println!("r {:?}", &r);
    println!("math r {}", math_expr(&r));
    println!("eval r {} = {} ", math_eval(&r), 1 - 2 - 3);

    let i = "1-2-3-4";
    println!("\n{}", i);
    let p = parse_expr(i).unwrap().1;
    println!("{:?}", &p);
    println!("math {}", math_expr(&p));
    println!("eval r {} = {} ", math_eval(&p), 1 - 2 - 3 - 4);
    let r = climb(p);
    println!("r {:?}", &r);
    println!("math r {}", math_expr(&r));
    println!("eval r {} = {} ", math_eval(&r), ((1 - 2) - 3) - 4);
}
