
#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer};
    use crate::parser::{Parser};

    #[test]
    fn test_winzig_01() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_01").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert_eq!(true, true);
    }

}