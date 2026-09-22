use strum_macros::{EnumString, Display};

#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display, Eq)]
pub enum Directive {
    #[strum(serialize = "log")]
    Log,
}