use crate::lexer::Lexer;
use crate::parser::tree::StringTree;
use clap::{Parser};


mod lexer;
mod parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct WinZigArgs {
    source: String,

    /// verbose mode
    #[arg(short, long, default_value = "false")]
    verbose: bool,

    /// print tokens
    #[arg(short, long, conflicts_with = "ast", default_value = "false")]
    tokens: bool,

    /// print ast
    #[arg(short, long, conflicts_with = "tokens", default_value = "true")]
    ast: bool,
}

fn main() {
    let args = WinZigArgs::parse();

    // read source file
    let source_text = std::fs::read_to_string(&args.source).unwrap();

    // lexical analysis
    let mut lexer = Lexer::new(source_text);
    let tokens = lexer.lex();
    if args.tokens {
        for token in &tokens {
            println!("{:?}", token);
        }
        return;
    }

    // ast construction
    let mut parser = parser::Parser::new(tokens, args.verbose);
    let program = parser.parse();
    if args.ast {
        let tree = program.get_string_tree(0);
        for line in tree {
            println!("{}", line);
        }
        return;
    }
}
