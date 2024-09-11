use crate::decl::{TypeDecl, ValDecl};

pub struct Mod {
    pub decls: Vec<ModDecl>
}

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