use crate::lexer::Lexer;
use crate::parser::Parser;

mod lexer;
mod parser;

fn main() {
    let mut lexer = Lexer::new(include_str!("./../winzig_test/winzig_03").to_string());
    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens);
    parser.parse();
}
