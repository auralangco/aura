use literal::Literal;

pub mod literal;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Ident(String),
    Call(Box<Expr>, StructExpr),
    CompoundExpr(CompoundExpr),
    StructExpr(StructExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompoundExpr(pub Vec<Expr>);

#[derive(Debug, Clone, PartialEq)]

pub struct StructExpr {
    pub positional: Vec<Expr>,
    pub named: Vec<(String, Expr)>,
}