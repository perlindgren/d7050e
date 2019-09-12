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
    Minus,
    Not,
    Par
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Atom(Atom),
    BinOp(Op, Box<Expr>, Box<Expr>),
    Unary(Op, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Num(i32),
    // Identifier
    // Function application
}


pub fn parse_i32(i: &str) -> IResult<&str, Expr> {
    map(digit1, |digit_str: &str| {
        Expr::Atom(Atom::Num(digit_str.parse::<i32>().unwrap()))
    })(i)
}

fn parse_uop(i: &str) -> IResult<&str, Op> {
    preceded(
        multispace0,
        alt((map(tag("-"), |_| Op::Minus), map(tag("!"), |_| Op::Not))),
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
            m.iter()
                .fold(t, |l, (op, r)| climb(l, op.clone(), r.clone()))
        },
    )(i)
}

fn binop(op: Op, l: Expr, r: Expr) -> Expr {
    Expr::BinOp(op, Box::new(l), Box::new(r))
}

fn climb(l: Expr, op: Op, r: Expr) -> Expr {
    match r.clone() {
        Expr::BinOp(r_op, r_l, r_r) => {
            let (prec, ass) = climb_op(&op);
            let (r_prec, _) = climb_op(&r_op);
            if r_prec
                < prec
                    + match ass {
                        Ass::Left => 1,
                        _ => 0,
                    }
            {
                binop(r_op, binop(op, l, *r_l), *r_r)
            } else {
                binop(op, l, r)
            }
        }
        _ => binop(op, l, r),
    }
}

fn parse_multiplicative(i: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            parse_terminal,
            many0(tuple((parse_mulop, parse_multiplicative))),
        )),
        |(t, m)| {
            m.iter()
                .fold(t, |l, (op, r)| climb(l, op.clone(), r.clone()))
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
            map(parse_parenthesis(parse_expr), |e| Expr::Unary(Op::Par, Box::new(e)))
        )),
    )(i)
}

// helpers
fn parse_parenthesis<'a, O, F, E>(inner: F) -> impl Fn(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    // delimited allows us to split up the input
    // cut allwos us to consume the input (and prevent backtracking)
    delimited(char('('), preceded(multispace0, inner), cut(char(')')))
}

fn main() {
    let p = parse_additative("1-(1+1)-1)").unwrap().1;
    println!("{:?} {} {}", p, math_eval(&p), 1-(1+1)-1);

    // let p = parse_additative("5*(20+2)/4").unwrap().1;
    // println!("{:?} {} {}", p, math_eval(&p), 5 * (20 + 2) / 4);
}

fn math_expr(e: &Expr) -> String {
    match e {
        Expr::Atom(Atom::Num(i)) => format!("{}", i),
        Expr::BinOp(op, l, r) => format!("({:?}, {}, {})", op, math_expr(l), math_expr(r)),
        Expr::Unary(op, e) => format!("({:?}, {})", op, math_expr(e)),
    }
}

fn math_eval(e: &Expr) -> i32 {
    match e {
        Expr::Atom(Atom::Num(i)) => *i,
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
        },
        Expr::Unary(op, e) => {
            let e = math_eval(e);
            match op {
                Op::Par => e,
                Op::Mul => -e,
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

fn climb_op(op: &Op) -> (u8, Ass) {
    match op {
        Op::Add => (1, Ass::Left),
        Op::Sub => (1, Ass::Left),
        Op::Mul => (2, Ass::Left),
        Op::Div => (2, Ass::Left),
        Op::Pow => (3, Ass::Right),
        _ => unimplemented!(),
    }
}

