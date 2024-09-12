use crate::{expr::Expr, tyexpr::TypeExpr};

#[derive(Debug, Clone, PartialEq)]
pub enum FnBody {
    Expr(Expr),
    Block(Vec<Statement>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Bind(BindStmt),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BindStmt {
    pub symbol: String,
    pub ty: TypeExpr,
    pub value: Expr,
}