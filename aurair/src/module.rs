use crate::decl::{fn_::FnBody, FnDecl, TypeDecl, ValDecl};

///  Root node for Aura modules
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

/// Node that represents the main entrypoint in Aura
/// 
/// Use only for the main module in a binary application
/// 
/// # Example
/// ```norun
/// main {
///    println("Hello, World!")
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MainDecl(pub FnBody);

/// Node that represents an import statement in Aura
/// 
/// # Example
/// 
/// ```norun
/// import aura/io // Imports the io module
/// import m = aura/math // Imports the math module with alias m
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub path: String,
    pub alias: Option<String>,
}