use crate::ast::{
    intrinsic_type::IntrinsicType,
    type_expression::TypeExpression,
};
use crate::source::span::{Span, Spanned};

///Tuple Type Positions can be ambiguous
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TupleTypePosition {
    /// Omitted type resolves to 'Any'.
    Labeled {
        type_: Spanned<TypeExpression>,
        label: Spanned<String>,
    },
    /// type_ is obligatory and can not be inferred.
    /// Repetition is allowed
    Unlabeled {
        type_: Spanned<TypeExpression>,
        repetition: Option<TupleRepetition>,
    },
    /// TypeExpression found a single identifier
    /// Context required to determine if it is a type_ or label
    Unclear {
        identifier: Spanned<String>
    }  
}

impl TupleTypePosition {
    pub fn is_labeled(&self) -> bool {
        matches!(
            self,
            Self::Labeled { .. }
        )
    }

    pub fn is_unclear(&self) -> bool {
        matches!(
            self,
            Self::Unclear { .. }
        )
    }

    pub fn is_unlabeled(&self) -> bool {
        matches!(
            self,
            Self::Unlabeled { .. }
        )
    }

    pub fn is_resolved(&self) -> bool {
        !self.is_unclear()
    }

    pub fn resolve_as_labeled(self) -> Self {
        match self {
            Self::Unclear { identifier } => {
                let any_pos = identifier.span.end;
                Self::Labeled {
                    type_: Spanned {
                        value: TypeExpression::Intrinsic(IntrinsicType::Any),
                        span: Span::empty(any_pos)
                    },
                    label: identifier
                }
            },
            other => other,
        }
    }

    pub fn resolve_as_unlabeled(self) -> Self {
        match self {
            Self::Unclear { identifier } => Self::Unlabeled {
                type_: identifier.map(TypeExpression::Identifier),
                repetition: None,
            },
            other => other,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleRepetition {
    pub value: Spanned<String>,
}
impl TupleRepetition {

    /// implementation not final
    pub fn as_u32(&self) -> Option<u32> {
        let value = self.value.value.as_str();

        if !self.is_integer() {
            return None;
        }

        value.parse().ok()
    }

    /// implementation not final
    pub fn is_integer(&self) -> bool {
        self.value.value.chars().all(|c| c.is_ascii_digit())
    }
}