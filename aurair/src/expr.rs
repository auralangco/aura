use literal::Literal;

use crate::tyexpr::TypeExpr;

pub mod literal;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Ident(String),
    Call(Box<Expr>, StructExpr),
    Compound(CompoundExpr),
    Struct(StructExpr),
    Operation(Operation),
    Access(Access)
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompoundExpr(pub Vec<Expr>);

#[derive(Debug, Clone, PartialEq)]
pub struct StructExpr {
    pub positional: Vec<Expr>,
    pub named: Vec<(String, Expr)>,
}

/// Node to describe all operations in Aura that produces a value 
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    /// Expr + Expr
    Add(Box<Expr>, Box<Expr>),
    /// Expr - Expr
    Sub(Box<Expr>, Box<Expr>),
    /// -Expr
    Neg(Box<Expr>),
    /// Expr * Expr
    Mul(Box<Expr>, Box<Expr>),
    /// Expr / Expr
    Div(Box<Expr>, Box<Expr>),
    /// Expr ** Expr
    Pow(Box<Expr>, Box<Expr>),
    /// Expr % Expr
    Mod(Box<Expr>, Box<Expr>),
    /// Expr && Expr
    And(Box<Expr>, Box<Expr>),
    /// Expr || Expr
    Or(Box<Expr>, Box<Expr>),
    /// !Expr
    Not(Box<Expr>),
    /// Expr == Expr
    Eq(Box<Expr>, Box<Expr>),
    /// Expr != Expr
    Neq(Box<Expr>, Box<Expr>),
    /// Expr > Expr
    Gt(Box<Expr>, Box<Expr>),
    /// Expr < Expr
    Lt(Box<Expr>, Box<Expr>),
    /// Expr >= Expr
    Geq(Box<Expr>, Box<Expr>),
    /// Expr <= Expr
    Leq(Box<Expr>, Box<Expr>),
    /// Expr ++ Expr
    Concat(Box<Expr>, Box<Expr>),
    /// Expr :: Expr
    Join(Box<Expr>, Box<Expr>),
    /// Expr \\ UIntLit
    Split(Box<Expr>, Literal),
    /// Expr |> Expr
    Pipe(Box<Expr>, Box<Expr>),
    /// Expr ?> Expr
    SafePipe(Box<Expr>, Box<Expr>),
    /// Expr?!
    UnwrapPanic(Box<Expr>),
    /// Expr ?= Expr
    UnwrapOr(Box<Expr>, Box<Expr>),
    /// Expr .. Expr
    Range(Box<Expr>, Box<Expr>),
    /// Expr ..= Expr
    RangeInclusive(Box<Expr>, Box<Expr>),
}

/// Describe the use of `.` and `:` accessers in Aura code
/// 
/// `.` accesses a property
/// `:` accesses a associated member
#[derive(Debug, Clone, PartialEq)]
pub enum Access {
    /// Expr . Ident
    ValProp(Box<Expr>, String),
    /// Type . Ident
    TyProp(TypeExpr, String),
    /// Expr : Ident
    ValAssoc(Box<Expr>, String),
    /// Type : Ident
    TyAssoc(TypeExpr, String)
}