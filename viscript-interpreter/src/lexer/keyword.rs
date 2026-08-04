use strum_macros::{{EnumString, Display}};

#[derive(Debug, Clone, PartialEq, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Keyword {
    // declarations
    Const,
    Let,
    Rel,

    Class,
    Enum,
    Interface,
    Type,
    Function,

    // Execution structure
    If,
    Else,
    Switch,
    Case,
    Default,

    For,
    While,
    Do,
    With,

    Break,
    Continue,
    Return,

    Try,
    Catch,
    Finally,
    Throw,

    Debugger,

    // Imports
    Import,
    Export,
    From,

    // Type system
    Declare,
    Namespace,

    Infer,
    Keyof,

    As,
    Is,

    // Class system
    Extends,
    Super,
    New,
    This,

    Static,

    Public,
    Private,
    Protected,
    Readonly,

    Get,
    Set,

    Override,
    Implements,

    // Operators-as-keywords
    In,
    Instanceof,
    Typeof,
    Delete,


    // Async
    Async,
    Await,
    Yield,

    // Special
    Of,
    Void,
}