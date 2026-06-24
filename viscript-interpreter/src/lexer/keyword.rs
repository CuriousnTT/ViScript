#[derive(Debug, Clone, PartialEq)]
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
    Target,
    Meta,
    Of,
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "async" => Some(Self::Async),
            "as" => Some(Self::As),
            "await" => Some(Self::Await),

            "break" => Some(Self::Break),

            "case" => Some(Self::Case),
            "catch" => Some(Self::Catch),
            "class" => Some(Self::Class),
            "const" => Some(Self::Const),
            "continue" => Some(Self::Continue),

            "debugger" => Some(Self::Debugger),
            "declare" => Some(Self::Declare),
            "default" => Some(Self::Default),
            "delete" => Some(Self::Delete),
            "do" => Some(Self::Do),

            "else" => Some(Self::Else),
            "enum" => Some(Self::Enum),
            "export" => Some(Self::Export),
            "extends" => Some(Self::Extends),

            "finally" => Some(Self::Finally),
            "for" => Some(Self::For),
            "from" => Some(Self::From),
            "function" => Some(Self::Function),

            "get" => Some(Self::Get),

            "if" => Some(Self::If),
            "import" => Some(Self::Import),
            "implements" => Some(Self::Implements),
            "infer" => Some(Self::Infer),
            "in" => Some(Self::In),
            "instanceof" => Some(Self::Instanceof),
            "interface" => Some(Self::Interface),
            "is" => Some(Self::Is),
            
            "keyof" => Some(Self::Keyof),
            
            "let" => Some(Self::Let),

            "meta" => Some(Self::Meta),
            
            "namespace" => Some(Self::Namespace),
            "new" => Some(Self::New),

            "of" => Some(Self::Of),
            "override" => Some(Self::Override),

            "private" => Some(Self::Private),
            "protected" => Some(Self::Protected),
            "public" => Some(Self::Public),

            "readonly" => Some(Self::Readonly),
            "rel" => Some(Self::Rel),
            "return" => Some(Self::Return),

            "set" => Some(Self::Set),
            "static" => Some(Self::Static),
            "super" => Some(Self::Super),
            "switch" => Some(Self::Switch),

            "target" => Some(Self::Target),
            "this" => Some(Self::This),
            "throw" => Some(Self::Throw),
            "try" => Some(Self::Try),
            "typeof" => Some(Self::Typeof),
            "type" => Some(Self::Type),
            
            //"using" => Some(Self::Using),

            "while" => Some(Self::While),
            //"with" => Some(Self::With),

            "yield" => Some(Self::Yield),

            _ => None,
        }
    }
}