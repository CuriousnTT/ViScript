use strum_macros::{EnumString, Display};

#[derive(Debug, Clone, PartialEq, EnumString, Display)]
pub enum Directive {
    #[strum(serialize = "log")]
    Log,
}