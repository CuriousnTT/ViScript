mod common;
mod lexer;
//mod parser;
mod source;

use std::env;
use std::fs;
use lexer::tokenizer::tokenize;
use source::sourcefile::SourceFile;
//use parser::ast::ASTNode;
//use parser::parser::parse;

fn main() {
    //Get test file from arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <test_file>", args[0]);
        return;
    };
    let filename = &args[1];
    println!("Reading {}...", filename);

    //Read and adapt test file to avoid errors
    let input = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    };
    let input = input.replace("\r\n", "\n");
    let file = SourceFile::new(input);
    println!("Input:\n\n{}\n", file.source);
    
    let lexer_result = tokenize(&file.source);
    if !lexer_result.errors.is_empty() {
        eprintln!("Lexer errors:");
        
        for err in &lexer_result.errors {
            eprint!("{}", err.report(&file));
        }

        //return;
    }
    if !lexer_result.tokens.is_empty() {
        eprintln!("Tokens:")
    }
    for token in &lexer_result.tokens {
        println!("  {}", token)
    }
    
    //let input_bytes = &input.as_bytes();
    //let ast: ASTNode = parse(&tokens);
    //println!("Token AST:\n{:?}", ast);
}