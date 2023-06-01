#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer};
    use crate::parser::{Parser};
    use crate::parser::tree::StringTree;

    #[test]
    fn test_winzig_01() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_01").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_01.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_02() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_02").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_02.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_03() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_03").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_03.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_04() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_04").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_04.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }

    #[test]
    fn test_winzig_05() {
        let mut lexer = Lexer::new(include_str!("./../../winzig_test/winzig_05").to_string());
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens);
        let program = parser.parse();

        let tree_str = program.get_string_tree(0).join("\n").trim().to_string();
        let expected = include_str!("./../../winzig_test/winzig_05.tree").trim().to_string();
        assert_eq!(tree_str, expected);
    }
}
