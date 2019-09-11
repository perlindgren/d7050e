extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{digit1, multispace0},
    combinator::{cut, map, opt},
    error::ParseError,
    multi::{fold_many0, many0, separated_list},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Eq,
    Neq,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UOp {
    Minus,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i32),
    BinOp(Op, Box<Expr>, Box<Expr>),
    Unary(UOp, Box<Expr>),
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

fn parse_uop(i: &str) -> IResult<&str, UOp> {
    preceded(
        multispace0,
        alt((map(tag("-"), |_| UOp::Minus), map(tag("!"), |_| UOp::Not))),
    )(i)
}

fn parse_mulop(i: &str) -> IResult<&str, Op> {
    preceded(
        multispace0,
        alt((map(tag("*"), |_| Op::Mul), map(tag("/"), |_| Op::Div))),
    )(i)
}

fn parse_addop(i: &str) -> IResult<&str, Op> {
    preceded(
        multispace0,
        alt((map(tag("+"), |_| Op::Add), map(tag("-"), |_| Op::Sub))),
    )(i)
}

//   expr       ::= eq-expr
//   eq-expr    ::= add-expr ( ( '==' | '!=' ) add-expr ) *
//   add-expr   ::= mul-expr ( ( '+' | '-' ) mul-expression ) *
//   mul-expr   ::= primary ( ( '*' | '/' ) terminal ) *
//   terminal   ::= '(' expr ')' | NUMBER | VARIABLE | '-' primary

fn parse_expr(i: &str) -> IResult<&str, Expr> {
    parse_additative(i)
}

fn parse_additative(i: &str) -> IResult<&str, Expr> {
    //map(tuple((parse_terminal, opt(parse_rhs)), |((_,t), _)| t))(i)
    map(
        tuple((
            parse_multiplicative,
            many0(tuple((parse_addop, parse_multiplicative))),
        )),
        |(t, m)| {
            println!("add: t {:?}, m {:?}", t, m);
            let r = m.iter().fold(t, |l, (op, r)| {
                println!("l {:?}, r {:?}", l, r);
                Expr::BinOp(*op, Box::new(l), Box::new(r.clone()))
            });
            r
        },
    )(i)
}

fn parse_multiplicative(i: &str) -> IResult<&str, Expr> {
    //map(tuple((parse_terminal, opt(parse_rhs)), |((_,t), _)| t))(i)
    map(
        tuple((
            parse_terminal,
            many0(tuple((parse_mulop, parse_multiplicative))),
        )),
        |(t, m)| {
            println!("mul: t {:?}, m {:?}", t, m);
            let r = m.iter().fold(t, |l, (op, r)| {
                println!("l {:?}, r {:?}", l, r);
                Expr::BinOp(*op, Box::new(l), Box::new(r.clone()))
            });
            r
        },
    )(i)
}

fn parse_terminal(i: &str) -> IResult<&str, Expr> {
    preceded(
        multispace0,
        alt((
            parse_i32,
            map(tuple((parse_uop, parse_terminal)), |(uop, e)| {
                Expr::Unary(uop, Box::new(e))
            }),
            parse_parenthesis(parse_expr),
        )),
    )(i)
}

// helpers
fn parse_parenthesis<'a, O, F, E>(
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

fn main() {
    let p = parse_additative("(1+2)*(5-1-1)").unwrap().1;
    println!("{:?} {} {}", p, math_eval(&p), (1 + 2) * (5 - 1 - 1));

    let p = parse_additative("5*(20+2)/4").unwrap().1;
    println!("{:?} {} {}", p, math_eval(&p), 5 * (20 + 2) / 4);
}

fn math_expr(e: &Expr) -> String {
    match e {
        Expr::Num(i) => format!("{}", i),
        Expr::BinOp(op, l, r) => {
            format!("({:?}, {}, {})", op, math_expr(l), math_expr(r))
        }
        Expr::Unary(op, e) => format!("({:?}, {})", op, math_expr(e)),
    }
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
        _ => unimplemented!(),
    }
}

// #[derive(Debug, Copy, Clone, PartialEq)]
// enum Ass {
//     Left,
//     Right,
// }

// fn climb_op(op: &Op) -> (u8, Ass) {
//     match op {
//         Op::Add => (1, Ass::Left),
//         Op::Sub => (1, Ass::Left),
//         Op::Mul => (2, Ass::Left),
//         Op::Div => (2, Ass::Left),
//         Op::Pow => (3, Ass::Right),
//     }
// }

// operator precedence parser
// https://en.wikipedia.org/wiki/Operator-precedence_parser
// parse_expression ()
//     return parse_expression_1 (parse_primary (), 0)

// parse_expression_1 (lhs, min_precedence)
//     lookahead := peek next token
//     while lookahead is a binary operator whose precedence is >= min_precedence
//         op := lookahead
//         advance to next token
//         rhs := parse_primary ()
//         lookahead := peek next token
//         while lookahead is a binary operator whose precedence is greater
//                  than op's, or a right-associative operator
//                  whose precedence is equal to op's
//             rhs := parse_expression_1 (rhs, lookahead's precedence)
//             lookahead := peek next token
//         lhs := the result of applying op with operands lhs and rhs
//     return lhs

// fn token_to_expr(t: Token) -> Expr {
//     match t {
//         Token::Num(i) => Expr::Num(i),
//         _ => panic!(),
//     }
// }

// fn climb(mut v: Vec<Token>, min_prec: u8) -> (Expr, Vec<Token>) {
//     println!("in climb {:?}, {}", v, min_prec);
//     let t = v.last().unwrap();
//     let mut result = token_to_expr(*t);
//     // loop {
//     //     match v.pop() {
//     //         Some(Token::Num(_)) => {
//     //             println!("break num");
//     //             break;
//     //         }
//     //         Some(Token::Op(op)) => {
//     //             println!("result {:?}, op {:?}, v:{:?}", result, op, v);
//     //             let (prec, assoc) = climb_op(&op);
//     //             if prec < min_prec {
//     //                 println!("break prec");
//     //                 break;
//     //             } else {
//     //                 println!("push");
//     //                 let next_min_prec =
//     //                     if assoc == Ass::Left { 1 + prec } else { prec };
//     //                 let (rhs, v_rest) = climb(v.clone(), next_min_prec);
//     //                 v = v_rest;
//     //                 println!("return from call, rhs {:?}, v {:?}", rhs, v);
//     //                 println!("current result {:?}", result);
//     //                 result = Expr::BinOp(Box::new(result), op, Box::new(rhs));
//     //                 println!("new result {:?}", result);
//     //             }
//     //         }

//     //         _ => {
//     //             println!("reaced end");
//     //             break;
//     //         } // reached end
//     //     }
//     // }
//     (result, v)
// }

// fn test_eq(s: &str, v: i32) {
//     let mut p = parse_expr(s).unwrap().1;
//     println!("{:?}", p);
//     p.reverse();
//     let e = climb(p, 0);
//     println!("{:?}", e);

//     println!("e = {}, v = {}", math_eval(&e.0), v);
// }

// fn main() {
//     test_eq("1 + 2", 1 + 2);
//     // test_eq("1 + 2 * 3", 1 + 2 * 3);
//     // test_eq("3 * 4 + 5", 3 * 4 + 5);

//     //     // climb_test("2*5+10+10", 2*5+10+10);
//     //     // climb_test("2*5+10*11-1", 2*5+10*11-1);
//     //     // climb_test("2*5+10*11-2+12", 2*5+10*11-1+12);
//     //     // climb_test("1+2*3-4+5", 1 + 2 * 3 - 4 + 5);
//     //     climb_test("1", 1);
//     //     climb_test("1+2", 1 + 2);
// }

// // // #[test]
// // // fn climb1() {
// // //     test_eq("1-2+3", 1 - 2 + 3);
// // // }

// // // #[test]
// // // fn climb2() {
// // //     test_eq("1*2+3", 1 * 2 + 3);
// // // }

// // // #[test]
// // // fn climb3() {
// // //     test_eq("1*2+3*4-5", 1 * 2 + 3 * 4 - 5);
// // // }

// // // #[test]
// // // fn climb4() {
// // //     test_eq("2^5", 2i32.pow(5));
// // // }

// // // #[test]
// // // fn climb5() {
// // //     test_eq("2*3+4+5", 2 * 3 + 4 + 5);
// // // }

// // // #[test]
// // // fn climb6() {
// // //     test_eq("2*3-4*5-2", 2 * 3 - 4 * 5 - 2);
// // // }

// // // #[test]
// // // fn climb_err() {
// // //     test_eq("2 + 2 ^ 5 -3", 2 + 2i32.pow(5 - 3));
// // // }

// // fn climb_test(s: &str, v: i32) {
// //     let p = parse_expr(s).unwrap().1;
// //     println!("{:?}", &p);
// //     println!("math {}\n", math_expr(&p));
// //     let r = climb(p, 0);
// //     println!("r {:?}", &r);
// //     println!("math r {}", math_expr(&r));
// //     println!("eval r {} = {} ", math_eval(&r), v);
// // }
