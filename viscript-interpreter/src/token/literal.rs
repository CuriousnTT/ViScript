use crate::token::regex::RegExp;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    BigInt(String),
    Boolean(bool),
    Null,
    Numeric(String),
    RegExp(RegExp),
    String(String),
    Undefined,
}