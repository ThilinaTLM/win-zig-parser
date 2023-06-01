#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer};
    use crate::parser::{Parser};
    use crate::parser::tree::StringTree;

    #[test]
    fn test_winzig_01() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_01").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, false);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_01.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_02() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_02").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, false);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_02.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_03() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_03").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, false);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_03.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_04() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_04").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, false);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_04.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_05() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_05").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, false);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_05.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_06() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_06").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, false);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_06.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_07() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_07").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, false);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_07.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_08() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_08").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, false);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_08.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_09() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_09").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, false);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_09.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_10() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_10").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, false);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_10.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_11() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_11").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, false);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_11.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_12() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_12").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, false);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_12.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_13() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_13").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, false);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_13.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_14() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_14").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, false);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_14.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_15() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_15").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, false);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_15.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }
}
