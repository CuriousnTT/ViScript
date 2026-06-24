#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    BigInt(String),
    Boolean(bool),
    Null,
    Numeric(String),
    RegExp(String),
    String(String),
    Undefined,
}