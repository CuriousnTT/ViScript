use crate::ast::{
    format::{
        expression::print_expression,
        format::Formatter,
    },
    template::Template,
    template_part::TemplatePart,
};
use crate::source::span::Spanned;

pub fn print_template(
    formatter: &mut Formatter,
    template: &Spanned<Template>,
) {
    formatter.line(format!(
        "Template {}",
        template.span.format(),
    ));

    formatter.indented(|formatter| {
        for part in &template.value.parts {
            match part {
                TemplatePart::Text(text ) => {
                    formatter.line(format!(
                        "Text({:?}) {}",
                        text.value,
                        text.span.format(),
                    ));
                }

                TemplatePart::Expression(expression) => {
                    formatter.line("expression:");

                    formatter.indented(|formatter| {
                        print_expression(formatter, expression);
                    });
                }
            }
        }
    });
}