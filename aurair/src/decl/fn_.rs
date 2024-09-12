use crate::{expr::Expr, tyexpr::TypeExpr};

// TODO: Other bodies
/// Node for function body in Aura
/// 
/// A body is where state can be produced by binding values to names for later use
#[derive(Debug, Clone, PartialEq)]
pub enum FnBody {
    Expr(Expr),
    Block(Vec<Statement>),
}

// TODO: Operation-Bind statements
/// The building block of a body, where state can be produced
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Bind(BindStmt),
    Expr(Expr),
}

/// Bind statements for bodies
/// 
/// # Example
/// 
/// ```norun
/// main {
///     name String = "Aura"
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BindStmt {
    pub symbol: String,
    pub ty: TypeExpr,
    pub value: Expr,
}