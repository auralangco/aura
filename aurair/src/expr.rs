use literal::Literal;

use crate::tyexpr::TypeExpr;

pub mod literal;

/// Node to describe an expression in Aura
/// 
/// Expression are the building blocks of Aura code, they can be literals, identifiers, calls, operations, etc. 
/// They produce a value and can be used in many places in the code.
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

/// Node to describe a compound expression in Aura
/// 
/// This is a list of expressions where the order matter
/// 
/// # Example
/// ```norun
/// main {
///     origin (Int, Int) = (0, 0)
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CompoundExpr(pub Vec<Expr>);

/// Node to describe a struct expression in Aura
/// 
/// This is a struct expression where the type is inferred from the context
/// 
/// # Example
/// ```norun
/// type Person = (name String, age Int)
/// 
/// main {
///     p Person = (name = "Aura", age = 0)
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct StructExpr {
    pub positional: Vec<Expr>,
    pub named: Vec<(String, Expr)>,
}

/// Node to describe a specific struct expression in Aura
/// 
/// This is a struct expression where the type is explicitly declared
/// 
/// # Example
/// 
/// ```norun
/// type Person = (name String, age Int)
/// 
/// main {
///     p Person = Person(name = "Aura", age = 0)
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SpecificStructExpr {
    pub ty: TypeExpr,
    pub positional: Vec<Expr>,
    pub named: Vec<(String, Expr)>,
}

/// Node to describe all operations in Aura that produces a value by using 
/// operators (symbols)
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