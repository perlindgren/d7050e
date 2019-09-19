// AST

use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;

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
pub enum Expr<'a> {
    Num(i32),
    Par(Box<SpanExpr<'a>>),
    // Identifier
    // Function application
    BinOp(Op, Box<SpanExpr<'a>>, Box<SpanExpr<'a>>),
    UnaryOp(Op, Box<SpanExpr<'a>>),
}

pub type SpanExpr<'a> = (Span<'a>, Expr<'a>);
