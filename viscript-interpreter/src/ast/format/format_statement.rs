use crate::ast::{
    format::{
        expression::print_expression,
        format::Formatter,
        tuple::print_tuple,
    },
    statement::Statement
};
use crate::source::span::Spanned;

pub fn format_statement(statement: &Spanned<Statement>) -> String {
    let mut formatter = Formatter::new();
    print_statement(&mut formatter, statement);
    formatter.finish()
}

pub fn print_statement(
    formatter: &mut Formatter,
    statement: &Spanned<Statement>,
) {
    match &statement.value {
        Statement::Directed {
            statement,
            directives
        } => {
            formatter.line(format!(
                "DirectedStatement {}",
                statement.span.format(),
            ));

            formatter.indented(|formatter| {
                formatter.line(
                    "Directives:",
                );
                formatter.indented(|formatter| {
                    for directive in directives {
                        formatter.line(format!(
                            "Directive {:?} {}",
                            directive.value,
                            directive.span.format(),
                        ));
                    };
                });
                print_statement(formatter, statement);
            })
        }
        Statement::ExportDeclaration {
            keyword,
            default,
            declaration
        } => {
            formatter.line(format!(
                "ExportStatement {}",
                statement.span.format(),
            ));

            formatter.indented(|formatter| {
                formatter.line(format!(
                    "Export {}",
                    keyword.span.format(),
                ));
                if let Some(default) = default {
                    formatter.line(format!(
                        "Default {}",
                        default.span.format(),
                    ));
                }
                print_statement(formatter, declaration);
            });
        }

        Statement::Expression(expression) => {
            formatter.line(format!(
                "ExpressionStatement {}",
                statement.span.format(),
            ));

            formatter.indented(|formatter| {
                print_expression(formatter, expression);
            });
        }

        Statement::FunctionDeclaration {
            async_,
            keyword,
            identifier,
            parameters,
            body,
        } => {
            formatter.line(format!(
                "FunctionDeclaration {}",
                statement.span.format(),
            ));

            formatter.indented(|formatter| {

                if let Some(async_) = async_ {
                    formatter.line(format!(
                        "Async {}",
                        async_.span.format(),
                    ));
                }

                formatter.line(format!(
                    "keyword: {:?} {}",
                    keyword.value,
                    keyword.span.format(),
                ));

                formatter.line(format!(
                    "identifier: {:?} {}",
                    identifier.value,
                    identifier.span.format(),
                ));

                formatter.line(format!(
                    "parameters: {:?} {}",
                    parameters.value,
                    parameters.span.format(),
                ));

                formatter.line(format!(
                    "body: StatementList {}",
                    body.span.format(),
                ));

                formatter.indented(|formatter| {
                    for statement in &body.statements {
                        print_statement(formatter, statement);
                    }
                });
            });
        }

        Statement::If {
            condition,
            then,
            else_
        } => {
            formatter.line(format!(
                "IfStatement {}",
                statement.span.format(),
            ));

            formatter.indented(|formatter| {
                formatter.line("condition:");

                formatter.indented(|formatter| {
                    print_tuple(formatter, condition);
                });

                formatter.line(format!(
                    "then: StatementList {}",
                    then.span.format(),
                ));

                formatter.indented(|formatter| {
                    for statement in &then.statements {
                        print_statement(formatter, statement);
                    }
                });

                if let Some(else_) = else_ {
                    formatter.line("else:");

                    formatter.indented(|formatter| {
                        print_statement(formatter, else_)
                    });
                }
            })
        }

        Statement::Return {
            keyword,
            value
        } => {
            formatter.line(format!(
                "ReturnStatement {}",
                statement.span.format(),
            ));

            formatter.indented(|formatter| {
                formatter.line(format!(
                    "keyword: {:?} {}",
                    keyword.value,
                    keyword.span.format(),
                ));
            });

            if let Some(value) = value {
                formatter.indented(|formatter| {
                    print_expression(formatter, value);
                });
            }
        }

        Statement::VariableDeclaration {
            keyword,
            identifier,
            value,
        } => {
            formatter.line(format!(
                "VariableDeclaration {}",
                statement.span.format(),
            ));

            formatter.indented(|formatter| {
                formatter.line(format!(
                    "keyword: {:?} {}",
                    keyword.value,
                    keyword.span.format(),
                ));

                formatter.line(format!(
                    "identifier: {:?} {}",
                    identifier.value,
                    identifier.span.format(),
                ));

                if let Some(value) = value {
                    formatter.line("value:");

                    formatter.indented(|formatter| {
                        print_expression(formatter, value);
                    });
                }
            });
        }

    }
}