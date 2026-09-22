use crate::ast::{
    format::{
        format::Formatter,
        template::print_template,
        tuple::print_tuple,
    },
    expression::Expression,
};
use crate::source::span::Spanned;

pub fn print_expression(
    formatter: &mut Formatter,
    expression: &Spanned<Expression>,
) {
    match &expression.value {
        Expression::Identifier(value) => {
            formatter.line(format!(
                "Identifier({value:?}) {}",
                expression.span.format(),
            ));
        }

        Expression::NullLiteral => {
            formatter.line(format!(
                "NullLiteral {}",
                expression.span.format(),
            ));
        }

        Expression::NumericLiteral(value) => {
            formatter.line(format!(
                "NumericLiteral({value:?}) {}",
                expression.span.format(),
            ));
        }

        Expression::StringLiteral(value) => {
            formatter.line(format!(
                "StringLiteral({value:?}) {}",
                expression.span.format(),
            ));
        }

        Expression::UndefinedLiteral => {
            formatter.line(format!(
                "UndefinedLiteral {}",
                expression.span.format(),
            ));
        }

        Expression::Binary {
            left,
            operator,
            right,
        } => {
            formatter.line(format!(
                "Binary {} {}",
                format!("{:?}", operator.value),
                expression.span.format(),
            ));

            formatter.indented(|formatter| {
                formatter.line("left:");

                formatter.indented(|formatter| {
                    print_expression(formatter, left);
                });

                formatter.line(format!(
                    "operator: {:?} {}",
                    operator.value,
                    operator.span.format(),
                ));

                formatter.line("right:");

                formatter.indented(|formatter| {
                    print_expression(formatter, right);
                });
            });
        }

        Expression::Assignment {
            target,
            operator,
            value,
        } => {
            formatter.line(format!(
                "Assignment {} {}",
                format!("{:?}", operator.value),
                expression.span.format(),
            ));

            formatter.indented(|formatter| {
                formatter.line("target:");

                formatter.indented(|formatter| {
                    print_expression(formatter, target);
                });

                formatter.line(format!(
                    "operator: {:?} {}",
                    operator.value,
                    operator.span.format(),
                ));

                formatter.line("value:");

                formatter.indented(|formatter| {
                    print_expression(formatter, value);
                });
            });
        }

        Expression::MemberAccess { object, member } => {
            formatter.line(format!(
                "MemberAccess {}",
                expression.span.format(),
            ));

            formatter.indented(|formatter| {
                formatter.line("object:");

                formatter.indented(|formatter| {
                    print_expression(formatter, object);
                });

                formatter.line(format!(
                    "member: {:?} {}",
                    member.value,
                    member.span.format(),
                ));
            });
        }

        Expression::Call {
            callable,
            arguments,
        } => {
            formatter.line(format!(
                "Call {}",
                expression.span.format(),
            ));

            formatter.indented(|formatter| {
                formatter.line("callable:");

                formatter.indented(|formatter| {
                    print_expression(formatter, callable);
                });

                formatter.line("arguments:");

                print_tuple(formatter, arguments);
            });
        }

        Expression::Tuple(tuple) => {
            formatter.line(format!(
                "TupleExpression {}",
                expression.span.format(),
            ));

            formatter.indented(|formatter| {
                print_tuple(formatter, tuple);
            });
        }

        Expression::Template(template) => {
            formatter.line(format!(
                "TemplateExpression {}",
                expression.span.format(),
            ));

            formatter.indented(|formatter| {
                print_template(formatter, template);
            });
        }
    }
}