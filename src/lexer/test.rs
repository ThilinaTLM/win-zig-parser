
#[cfg(test)]
mod tests {
    use crate::lexer::{Keyword, Lexer, Token};

    #[test]
    fn test_lex_identifier() {
        let mut lexer = Lexer::new("hello".to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![Token::Identifier("hello".to_string())]);
    }

    #[test]
    fn test_lex_keyword() {
        let mut lexer = Lexer::new("program".to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![Token::Keyword(Keyword::Program)]);
    }

    #[test]
    fn test_lex_mixed() {
        let mut lexer = Lexer::new("hello program".to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![Token::Identifier("hello".to_string()), Token::Keyword(Keyword::Program)]);
    }

    #[test]
    #[should_panic(expected = "Unexpected character: !")]
    fn test_lex_unexpected_character() {
        let mut lexer = Lexer::new("hello!".to_string());
        lexer.lex();
    }
}
