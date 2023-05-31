
#[cfg(test)]
mod tests {
    use crate::lexer::{Keyword, Lexer, Operator, Token};

    #[test]
    fn test_lex_keyword() {
        let mut lexer = Lexer::new("program".to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![Token::Keyword(Keyword::Program)]);
    }

    #[test]
    fn test_lex_operator() {
        let mut lexer = Lexer::new("+".to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![Token::Operator(Operator::Plus)]);
    }

    #[test]
    fn test_lex_integer() {
        let mut lexer = Lexer::new("12345".to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![Token::Integer(12345)]);
    }

    #[test]
    fn test_lex_identifier() {
        let mut lexer = Lexer::new("myVariable".to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![Token::Identifier("myVariable".to_string())]);
    }

    #[test]
    fn test_lex_string() {
        let mut lexer = Lexer::new("\"hello world\"".to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![Token::String("\"hello world\"".to_string())]);
    }

    #[test]
    fn test_lex_comment() {
        let mut lexer = Lexer::new("# this is a comment".to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![Token::Comment]);
    }

    #[test]
    fn test_lex_whitespace() {
        let mut lexer = Lexer::new("   ".to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![Token::Whitespace]);
    }

    #[test]
    fn test_lex_special_token() {
        let mut lexer = Lexer::new(";\n,".to_string());
        let tokens = lexer.lex();
        assert_eq!(tokens, vec![Token::Semicolon, Token::Newline, Token::Comma]);
    }

    #[test]
    fn test_lex_complex() {
        let mut lexer = Lexer::new("program test; var a := 10; # comment".to_string());
        let tokens = lexer.lex();
        assert_eq!(
            tokens,
            vec![
                Token::Keyword(Keyword::Program),
                Token::Whitespace,
                Token::Identifier("test".to_string()),
                Token::Semicolon,
                Token::Whitespace,
                Token::Keyword(Keyword::Var),
                Token::Whitespace,
                Token::Identifier("a".to_string()),
                Token::Whitespace,
                Token::Operator(Operator::Assignment),
                Token::Whitespace,
                Token::Integer(10),
                Token::Semicolon,
                Token::Whitespace,
                Token::Comment
            ]
        );
    }
}
