
#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer};
    use crate::parser::{Parser};

    #[test]
    fn test_winzig_01() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_01").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(true, true);
    }

    #[test]
    fn test_winzig_02() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_02").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(true, true);
    }

    #[test]
    fn test_winzig_03() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_03").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(true, true);
    }

    #[test]
    fn test_winzig_04() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_04").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(true, true);
    }

    #[test]
    fn test_winzig_05() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_05").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(true, true);
    }

    #[test]
    fn test_winzig_06() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_06").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(true, true);
    }

    #[test]
    fn test_winzig_07() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_07").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(true, true);
    }

    #[test]
    fn test_winzig_08() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_08").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(true, true);
    }

    #[test]
    fn test_winzig_09() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_09").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(true, true);
    }

    #[test]
    fn test_winzig_10() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_10").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(true, true);
    }

    #[test]
    fn test_winzig_11() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_11").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(true, true);
    }

    #[test]
    fn test_winzig_12() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_12").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(true, true);
    }

    #[test]
    fn test_winzig_13() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_13").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(true, true);
    }

    #[test]
    fn test_winzig_14() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_14").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(true, true);
    }

    #[test]
    fn test_winzig_15() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_15").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(true, true);
    }

}