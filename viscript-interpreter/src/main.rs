mod lexer;
//mod parser;
mod utility;

use std::env;
use std::fs;
use lexer::tokenizer::tokenize;
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
    println!("Input:\n\n{}\n", input);
    
    let tokens = match tokenize(&input) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("Error tokenizing input: {}", err);
            return;
        }
    };
    println!("Input Tokenized:\n{:?}", tokens);
    
    //let input_bytes = &input.as_bytes();
    //let ast: ASTNode = parse(&tokens);
    //println!("Token AST:\n{:?}", ast);
}