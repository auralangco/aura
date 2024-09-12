/// Literal values in the language
/// 
/// Those values are mainly used in `val` declarations since they are 
/// known at compile time
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    String(String),
    Char(char),
    Array(Vec<Literal>),
    Compound(Vec<Literal>),
    Struct(Vec<(String, Literal)>),
    Unit
    // TODO: fn, branch, variant
}