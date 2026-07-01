mod lexer;
//mod parser;
mod utility;

use std::env;
use std::fs;
use lexer::tokenizer::tokenize;
use utility::span::SourceFile;
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
    let file = SourceFile::new(input.clone());
    let input = input.replace("\r\n", "\n");
    println!("Input:\n\n{}\n", input);
    
    let lexer_result = tokenize(&input);
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