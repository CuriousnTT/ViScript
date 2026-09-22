use crate::ast::{
    format::{
        expression::print_expression,
        format::Formatter,
        format_statement::print_statement,
    },
    tuple::Tuple,
    tuple_position::TuplePosition,
};
use crate::source::span::Spanned;

pub fn print_tuple(
    formatter: &mut Formatter,
    tuple: &Spanned<Tuple>,
) {
    formatter.line(format!(
        "Tuple {}",
        tuple.span.format(),
    ));

    formatter.indented(|formatter| {
        for position in &tuple.value.positions {
            print_tuple_position(formatter, position);
        }
    });
}

fn print_tuple_position(
    formatter: &mut Formatter,
    position: &Spanned<TuplePosition>,
) {
    formatter.line(format!(
        "position {}", 
        position.span.format(),
    ));

    formatter.indented(|formatter| {
        if !position.value.statements.is_empty() {
            formatter.line("statements:");
            
            formatter.indented(|formatter| {
                for statement in &position.value.statements {
                    print_statement(formatter, statement);
                }
            });
        }

        if let Some(label) = &position.value.label {
            formatter.line(format!(
                "label: {:?} {}",
                label.value,
                label.span.format(),
            ));
        }

        formatter.line("value:");

        formatter.indented(|formatter| {
            print_expression(formatter, &position.value.value);
        });

    });
}