use fn_::FnBody;

use crate::{expr::literal::Literal, tyexpr::{FnType, TypeExpr}};

pub mod fn_;

/// IR Node that represents a value declaration in Aura
/// 
/// This is the generic value declaration in Aura, expects a symbol, the type and a literal to be assigned to it
/// 
/// # Example
/// ```norun
/// val name String = "Aura"
/// val version Int = 0
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ValDecl {
    pub symbol: String,
    pub ty: TypeExpr,
    pub value: Literal
}

/// IR Node that represents a type declaration in Aura
/// 
/// This is a WIP node, given that associated members aren't implemented yet
/// 
/// # Example
/// 
/// ```norun    
/// type Dog = (breed String, color String, height Float)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDecl {
    pub symbol: String,
    pub ty: TypeExpr,
    pub assoc: Vec<TyAssoc>,
}

/// Node for function declaration in Aura
/// 
/// # Example
/// 
/// ```norun
/// fn sum(a Int, b Int) -> Int = a + b
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub symbol: String,
    pub ty: FnType,
    pub body: FnBody
}

/// Node for associated members in a type declaration
/// 
/// Those members may be `val`, `fn` or `type`
/// 
/// # Example
/// 
/// ```norun
/// type Coordinate = (x Int, y Int) {
///     // Associated type
///     type T = Int
/// 
///     // Associated value
///     val origin Coordinate = Coordinate (x = 0, y = 0)
///     
///     // Associated function
///     fn swap(self) -> Coordinate = Coordinate (x = self.y, y = self.x )
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum TyAssoc {
    Val(ValDecl),
    Fn(FnDecl),
    Type(TypeDecl),
}