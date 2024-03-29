extern crate fox_lib;

use fox_lib::evaluator::object::*;
use fox_lib::evaluator::*;
use fox_lib::lexer::token::*;
use fox_lib::lexer::*;
use fox_lib::parser::*;
use std::fs::File;
use std::io::prelude::*;

fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[test]
fn test_example_hash() {
    let code_string = read_file("examples/map-reduce.mk".to_owned()).ok().unwrap();
    let mut evaluator = Evaluator::new();
    let (_, lex_tokens) = Lexer::lex_tokens(&code_string).unwrap();
    let tokens = Tokens::new(&lex_tokens);
    let (_, program) = Parser::parse_tokens(tokens).unwrap();
    let eval = evaluator.eval_program(program);
    assert_eq!(eval, Object::Null);
}

#[test]
fn test_reduce() {
    let code_string = read_file("examples/hash.mk".to_owned()).ok().unwrap();
    let mut evaluator = Evaluator::new();
    let (_, lex_tokens) = Lexer::lex_tokens(&code_string).unwrap();
    let tokens = Tokens::new(&lex_tokens);
    let (_, program) = Parser::parse_tokens(tokens).unwrap();
    let eval = evaluator.eval_program(program);
    assert_eq!(eval, Object::Null);
}
