#[derive(Debug, Clone)]
pub enum ParenContext {
    Expression,
    If,
    While,
    For,
    With,
    Catch,
}

#[derive(Debug, Clone)]
pub enum BraceContext {
    Block,
    Object,
}