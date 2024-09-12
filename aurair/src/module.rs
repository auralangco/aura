use crate::decl::{fn_::FnBody, FnDecl, TypeDecl, ValDecl};

#[derive(Debug, Clone, PartialEq)]
pub struct Mod {
    pub imports: Vec<Import>,
    pub decls: Vec<ModDecl>,
    pub main: Option<MainDecl>,
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
    Fn(FnDecl),
    Type(TypeDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MainDecl(pub FnBody);

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub path: String,
    pub alias: Option<String>,
}