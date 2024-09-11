#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Char(char),
    Array(Vec<Literal>),
    Compound(Vec<Literal>),
    Struct(Vec<(String, Literal)>),
    Unit
}