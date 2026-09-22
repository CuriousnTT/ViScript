use crate::ast::template_part::TemplatePart;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    pub parts: Vec<TemplatePart>,
}