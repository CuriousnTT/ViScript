mod ast;
mod common;
mod lexer;
mod parser;
mod source;
mod token;

use std::env;
use lexer::tokenizer::tokenize;
use source::sourcefile::SourceFile;
//use parser::ast::ASTNode;
use parser::parser::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Get test file from arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <test_file>", args[0]);
    };
    let filename = &args[1];
    println!("Reading {}...", filename);
    let file = SourceFile::new(filename)?;
    println!("Input:\n\n{}\n", file.source);
    
    let lexer_result = tokenize(
        &file.source,
        file.is_vsx()
    );
    if !lexer_result.errors.is_empty() {
        eprintln!("Lexer errors:");
        
        for err in &lexer_result.errors {
            eprint!("{}", err.report(&file));
        }

        //return;
    }
    if !lexer_result.tokens.is_empty() {
        println!("Raw Lexer Tokens:")
    }
    for token in &lexer_result.tokens {
        println!("  {}", token)
    }
    
    let parse_result = parse(&lexer_result.tokens);
    if !parse_result.errors.is_empty() {
        eprintln!("Parser errors:");
    
        for err in &parse_result.errors {
            eprint!("{}", err.report(&file));
        }
    }

    if let Some(ast) = &parse_result.ast {
        if !ast.statements.is_empty() {
            println!("Parsed statements");
            
            for stmt in &ast.statements {
                println!("  {:?}", stmt)
            }
        }
    }

    //let ast: ASTNode = parse(&tokens);
    //println!("Token AST:\n{:?}", ast);
    Ok(())
}