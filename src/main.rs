use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::parser::tree::StringTree;

mod lexer;
mod parser;

fn main() {
    let mut lexer = Lexer::new(include_str!("./../winzig_test/winzig_01").to_string());
    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens);
    let program = parser.parse();
    let tree = program.get_string_tree(0);
    for line in tree {
        println!("{}", line);
    }
}
