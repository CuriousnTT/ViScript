use crate::lexer::{
    keyword::Keyword,
    literal:: Literal,
    token::Token,
};

use super::ast::ASTNode;
use std::iter::Peekable;
use std::slice::Iter;

pub fn parse(tokens: &[Token]) -> ASTNode {
    let mut tokens = tokens.iter().peekable();
    parse_program(&mut tokens)
}

fn parse_program(tokens: &mut Peekable<Iter<Token>>) -> ASTNode {
    let mut nodes = Vec::new();
    while let Some(_) = tokens.peek() {
        nodes.push(parse_statement(tokens));
    }
    ASTNode::Program(nodes)
}

fn parse_statement(tokens: &mut Peekable<Iter<Token>>) -> ASTNode {
    match tokens.peek() {
        Some(Token::Keyword(kw)) if *kw == Keyword::Let || *kw == Keyword::Const || *kw == Keyword::Rel => {
            parse_variable_declaration(tokens)
        }
        Some(Token::Keyword(kw)) if *kw == Keyword::Function => parse_function_declaration(tokens),
        Some(Token::Identifier(_)) => parse_expression_statement(tokens),
        Some(Token::Keyword(kw)) if *kw == Keyword::If => parse_if_statement(tokens),
        _ => panic!("Unexpected token: {:?}", tokens.peek()),
    }
}

fn parse_variable_declaration(tokens: &mut Peekable<Iter<Token>>) -> ASTNode {
    //TODO: Implement reactivity for 'let' and 'rel'
    let keyword = tokens.next().expect("Expected a keyword").clone();

    let identifier = match tokens.next() {
        Some(Token::Identifier(id)) => id.clone(),
        Some(token) => panic!("Expected an identifier, found {:?}", token),
        None => panic!("Expected an identifier, found end of tokens"),
    };

    match tokens.next() {
        Some(Token::Operator(op)) if op == "=" => (),
        Some(token) => panic!("Expected '=', found {:?}", token),
        None => panic!("Expected '=', found end of tokens"),
    }

    let value = parse_expression(tokens);

    let directive = match tokens.peek() {
        Some(Token::Directive(_)) => {
            let dir_token = tokens.next().unwrap().clone();
            if let Token::Directive(name) = dir_token {
                Some(name)
            } else {
                None
            }
        },
        _ => None,
    };

    match tokens.next() {
        Some(Token::Punctuation(p)) if p == ";" => (),
        Some(token) => panic!("Expected ';', found {:?}", token),
        None => panic!("Expected ';', found end of tokens"),
    }

    ASTNode::VariableDeclaration {
        keyword,
        identifier,
        value: Box::new(value),
        directive,
    }
}

fn parse_expression(tokens: &mut Peekable<Iter<Token>>) -> ASTNode {
    let mut left = parse_primary(tokens);

    while let Some(token) = tokens.peek() {
        match token {
            Token::Operator(op) => {
                let op = op.clone();
                tokens.next();
                let right = parse_expression(tokens);
                left = ASTNode::Expression {
                    left: Box::new(left),
                    operator: op,
                    right: Box::new(right),
                };
            }
            Token::Punctuation(p) if p == "." => {
                tokens.next(); // consume the dot
                let identifier = match tokens.next() {
                    Some(Token::Identifier(id)) => id.clone(),
                    Some(token) => panic!("Expected an identifier after '.', found {:?}", token),
                    None => panic!("Expected an identifier after '.', found end of tokens"),
                };
                left = ASTNode::MemberAccess {
                    object: Box::new(left),
                    property: identifier,
                };
            }
            _ => break,
        }
    }
    
    left
}

fn parse_primary(tokens: &mut Peekable<Iter<Token>>) -> ASTNode {
    match tokens.next() {
        Some(Token::Identifier(id)) => ASTNode::Identifier(id.clone()),

        Some(Token::Literal(lit)) => match lit {
            Literal::Numeric(n) => ASTNode::NumberLiteral(n.clone()),
            Literal::String(s) => ASTNode::StringLiteral(s.clone()),
            Literal::Boolean(b) => ASTNode::BooleanLiteral(*b),
            Literal::Null => ASTNode::UndefinedLiteral,
            Literal::Undefined => ASTNode::UndefinedLiteral,
            Literal::BigInt(s) => ASTNode::BigIntLiteral(s.clone()),
            Literal::RegExp(s) => ASTNode::RegExpLiteral(s.clone()),
        },

        Some(Token::Directive(d)) => ASTNode::Directive(d.clone()),

        Some(token) => panic!("Unexpected token: {:?}", token),
        None => panic!("Unexpected end of tokens"),
    }
}

fn parse_if_statement(tokens: &mut Peekable<Iter<Token>>) -> ASTNode {
    tokens.next(); // consume 'if'
    match tokens.next() {
        Some(Token::Punctuation(p)) if p == "(" => (),
        Some(token) => panic!("Expected '(', found {:?}", token),
        None => panic!("Expected '(', found end of tokens"),
    }

    let condition = parse_expression(tokens);

    match tokens.next() {
        Some(Token::Punctuation(p)) if p == ")" => (),
        Some(token) => panic!("Expected ')', found {:?}", token),
        None => panic!("Expected ')', found end of tokens"),
    }

    let consequence = parse_block(tokens);

    ASTNode::IfStatement {
        condition: Box::new(condition),
        consequence: Box::new(consequence),
    }
}

fn parse_block(tokens: &mut Peekable<Iter<Token>>) -> ASTNode {
    match tokens.next() {
        Some(Token::Punctuation(p)) if p == "{" => (),
        Some(token) => panic!("Expected '{{', found {:?}", token),
        None => panic!("Expected '{{', found end of tokens"),
    }

    let mut statements = Vec::new();
    while let Some(token) = tokens.peek() {
        if let Token::Punctuation(p) = token {
            if p == "}}" {
                break;
            }
        }
        statements.push(parse_statement(tokens));
    }

    match tokens.next() {
        Some(Token::Punctuation(p)) if p == "}" => (),
        Some(token) => panic!("Expected '}}', found {:?}", token),
        None => panic!("Expected '}}', found end of tokens"),
    }

    ASTNode::Block(statements)
}

fn parse_expression_statement(tokens: &mut Peekable<Iter<Token>>) -> ASTNode {
    let expr = parse_expression(tokens);
    match tokens.next() {
        Some(Token::Punctuation(p)) if p == ";" => (),
        Some(token) => panic!("Expected ';', found {:?}", token),
        None => panic!("Expected ';', found end of tokens"),
    }
    expr
}

fn parse_function_declaration(tokens: &mut Peekable<Iter<Token>>) -> ASTNode {
    tokens.next(); // consume 'function'

    let identifier = match tokens.next() {
        Some(Token::Identifier(id)) => id.clone(),
        Some(token) => panic!("Expected an identifier, found {:?}", token),
        None => panic!("Expected an identifier, found end of tokens"),
    };

    match tokens.next() {
        Some(Token::Punctuation(p)) if p == "(" => (),
        Some(token) => panic!("Expected '(', found {:?}", token),
        None => panic!("Expected '(', found end of tokens"),
    }

    //TODO: Parse parameters

    match tokens.next() {
        Some(Token::Punctuation(p)) if p == ")" => (),
        Some(token) => panic!("Expected ')', found {:?}", token),
        None => panic!("Expected ')', found end of tokens"),
    }

    let body = parse_block(tokens);

    ASTNode::FunctionDeclaration {
        name: identifier,
        body: Box::new(body),
    }
}