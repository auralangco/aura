use crate::{literal::Literal, tyexpr::{FnType, TypeExpr}};

/// IR Node that refeers to declaration statements in Aura
/// 
/// Those statements are present in modules 
/// 
/// # Example
/// ```norun
/// // main.aura
/// val x Int = 10
/// val y Float = 10.0
/// 
/// fn sum(a Int, b Int): Int = a + b
/// 
/// type Person = (name String, age Int)
/// 
/// main {
///     println(sum(x, y))
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ModDecl {
    Val(ValDecl),
    Fn,
    Type(TypeDecl),
    Main,
}

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
    pub ty: TypeExpr
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub symbol: String,
    pub ty: FnType
}