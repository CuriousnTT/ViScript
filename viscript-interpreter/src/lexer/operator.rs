use strum_macros::{EnumString, Display};

#[derive(Debug, Clone, PartialEq, EnumString, Display)]
pub enum Operator {
    // Arithmetic
    #[strum(serialize = "+")]
    Addition,
    #[strum(serialize = "-")]
    Subtraction,
    #[strum(serialize = "*")]
    Multiplication,
    #[strum(serialize = "/")]
    Division,
    #[strum(serialize = "%")]
    Remainder,
    #[strum(serialize = "**")]
    Exponentiation,

    #[strum(serialize = "++")]
    Increment,
    #[strum(serialize = "--")]
    Decrement,

    // Assignment
    #[strum(serialize = "=")]
    Assignment,
    #[strum(serialize = "+=")]
    AdditionAssignment,
    #[strum(serialize = "-=")]
    SubtractionAssignment,
    #[strum(serialize = "*=")]
    MultiplicationAssignment,
    #[strum(serialize = "/=")]
    DivisionAssignment,
    #[strum(serialize = "%=")]
    RemainderAssignment,
    #[strum(serialize = "**=")]
    ExponentiationAssignment,

    // Bitwise assignment
    #[strum(serialize = "<<=")]
    LeftShiftAssignment,
    #[strum(serialize = ">>=")]
    RightShiftAssignment,
    #[strum(serialize = ">>>=")]
    UnsignedRightShiftAssignment,
    #[strum(serialize = "&=")]
    BitwiseAndAssignment,
    #[strum(serialize = "|=")]
    BitwiseOrAssignment,
    #[strum(serialize = "^=")]
    BitwiseXorAssignment,

    // Logical assignment
    #[strum(serialize = "&&=")]
    LogicalAndAssignment,
    #[strum(serialize = "||=")]
    LogicalOrAssignment,
    #[strum(serialize = "??=")]
    NullishCoalescingAssignment,

    // Comparison
    #[strum(serialize = "==")]
    Equality,
    #[strum(serialize = "!=")]
    Inequality,
    #[strum(serialize = "===")]
    StrictEquality,
    #[strum(serialize = "!==")]
    StrictInequality,
    #[strum(serialize = "<")]
    LessThan,
    #[strum(serialize = "<=")]
    LessThanOrEqual,
    #[strum(serialize =">")]
    GreaterThan,
    #[strum(serialize = ">=")]
    GreaterThanOrEqual,

    // Bitwise
    #[strum(serialize = "&")]
    BitwiseAnd,
    #[strum(serialize = "|")]
    BitwiseOr,
    #[strum(serialize = "^")]
    BitwiseXor,
    #[strum(serialize = "~")]
    BitwiseNot,
    #[strum(serialize = "<<")]
    LeftShift,
    #[strum(serialize = ">>")]
    RightShift,
    #[strum(serialize = ">>>")]
    UnsignedRightShift,
    #[strum(serialize = "?")]
    Conditional,

    // Logical
    #[strum(serialize = "&&")]
    LogicalAnd,
    #[strum(serialize ="||")]
    LogicalOr,
    #[strum(serialize = "!")]
    LogicalNot,

    #[strum(serialize = "??")]
    NullishCoalescing,

    // Other
    #[strum(serialize = "?.")]
    OptionalChaining,
    #[strum(serialize = "=>")]
    Arrow,
    #[strum(serialize = "...")]
    Spread,
}

impl Operator {
    /// returns true for Operators that are legal at the top level of a VSX
    /// opening tag's attribute list (outside of expressions)
    /// 
    /// NOTE: Rules apply differently immediately after VSX tag *names*
    /// Ex: A VSX element can be defined as
    /// <NameSpace-Element<Type, Argument>
    ///   /*Attributes follow */
    /// />
    /// LessThan and GreaterThan mark rule boundaries.
    /// They are only legal in exactly that position.
    pub fn legal_in_vsx(self) -> bool {
        matches!(
            self,

            //Attribute assignment
            Self::Assignment    // =

            // Omit LessThan & GreaterThan?

        )
    }
}