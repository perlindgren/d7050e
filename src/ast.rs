// AST

use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Eq,
    Neq,
    And,
    Or,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Not,
}

type SpanOp<'a> = (Span<'a>, Op);

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i32),
    Par(Box<Expr>),
    // Identifier
    // Function application
    BinOp(Op, Box<Expr>, Box<Expr>),
    UnaryOp(Op, Box<Expr>),
}

type SpanExpr<'a> = (Span<'a>, Expr);
