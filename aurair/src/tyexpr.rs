#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpr {
    Ident(String),
    Compound(CompoundType),
    Struct(StructType),
    Fn(FnType),
}

/// Aura compound type, a list of other types where the order matter
/// 
/// # Example
/// 
/// ```norun
/// type Coordinate = (Int, Int)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CompoundType(pub Vec<TypeExpr>);


/// The Aura struct type, a list of symbols with their type
///
/// Similar to compound but with named fields
///  
/// # Example
/// 
/// ```norun
/// type Car = (brand String, year Int, model String)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct StructType(pub Vec<(String, TypeExpr)>);

/// IR Node for a Aura fn type
/// 
/// # Example
/// 
/// ```norun
/// type MyFn = (a Int, b Int) -> Bool 
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FnType {
    pub input: StructType,
    pub output: Box<TypeExpr>
}