#[derive(Debug, Clone)]
pub enum Template {
    /// The start of a template, until first interpolation
    Head(String),
    /// The part of a template between interpolations
    Middle(String),
    /// The end of a template, after the last interpolation
    Tail(String),
    /// Complete template with no interpolation (i.e. a pointless template)
    Complete(String)
}