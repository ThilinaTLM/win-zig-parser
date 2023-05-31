#![allow(dead_code)]

use std::str::FromStr;

mod test;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,

    LessThan,
    LessEqual,
    NotEqual,
    GreaterThan,
    GreaterEqual,
    Equal,

    Assignment,
    Swap,
}

impl Operator {
    fn is_operator_starting_char(c: char) -> bool {
        match c {
            '+' | '-' | '*' | '/' | '<' | '>' | '=' | ':' => true,
            _ => false,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Program,
    Var,
    Const,
    Type,
    Function,
    Return,
    Begin,
    End,
    Output,
    If,
    Then,
    Else,
    While,
    Do,
    Case,
    Of,
    Otherwise,
    Repeat,
    For,
    Until,
    Loop,
    Pool,
    Exit,

    Mod,
    And,
    Or,
    Not,

    Read,
    Succ,
    Pred,
    Chr,
    Ord,
    Eof,
}

impl Keyword {
    fn parse_keyword(s: String) -> Option<Keyword> {
        match s.as_str() {
            "program" => Some(Keyword::Program),
            "var" => Some(Keyword::Var),
            "const" => Some(Keyword::Const),
            "type" => Some(Keyword::Type),
            "function" => Some(Keyword::Function),
            "return" => Some(Keyword::Return),
            "begin" => Some(Keyword::Begin),
            "end" => Some(Keyword::End),
            "output" => Some(Keyword::Output),
            "if" => Some(Keyword::If),
            "then" => Some(Keyword::Then),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "do" => Some(Keyword::Do),
            "case" => Some(Keyword::Case),
            "of" => Some(Keyword::Of),
            "otherwise" => Some(Keyword::Otherwise),
            "repeat" => Some(Keyword::Repeat),
            "for" => Some(Keyword::For),
            "until" => Some(Keyword::Until),
            "loop" => Some(Keyword::Loop),
            "pool" => Some(Keyword::Pool),
            "exit" => Some(Keyword::Exit),
            "mod" => Some(Keyword::Mod),
            "and" => Some(Keyword::And),
            "or" => Some(Keyword::Or),
            "not" => Some(Keyword::Not),
            "read" => Some(Keyword::Read),
            "succ" => Some(Keyword::Succ),
            "pred" => Some(Keyword::Pred),
            "chr" => Some(Keyword::Chr),
            "ord" => Some(Keyword::Ord),
            "eof" => Some(Keyword::Eof),
            _ => None,
        }
    }
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Keyword::parse_keyword(s.to_string()) {
            Some(keyword) => Ok(keyword),
            None => Err(()),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Operator(Operator),
    Integer(i64),
    Char(char),
    String(String),
    Comment,
    Whitespace,

    Newline,
    Dot,
    Dots,
    Colon,
    Semicolon,
    Comma,
    LeftParen,
    RightParen,
}

impl PartialEq<Token> for &Token {
    fn eq(&self, other: &Token) -> bool {
        *self == other
    }
}

pub struct Lexer {
    source_text: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source_text: String) -> Self {
        Self {
            source_text,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.commit();
            let token = self.scan_token();
            tokens.push(token);
        }

        tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source_text.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source_text.chars().nth(self.current - 1).unwrap()
    }

    fn commit(&mut self) {
        self.start = self.current;
    }

    fn rollback(&mut self) {
        self.current = self.start;
    }

    fn selection(&mut self) -> String {
        self.source_text[self.start..self.current].to_string()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source_text.chars().nth(self.current).unwrap()
        }
    }

    fn scan_token(&mut self) -> Token {
        if let Some(token) = self.identifier_or_keyword() {
            return token;
        }
        if let Some(token) = self.operator() {
            return token;
        }
        if let Some(token) = self.integer() {
            return token;
        }
        if let Some(token) = self.char() {
            return token;
        }
        if let Some(token) = self.string() {
            return token;
        }
        if let Some(token) = self.comment() {
            return token;
        }
        if let Some(token) = self.whitespace() {
            return token;
        }
        if let Some(token) = self.special_token() {
            return token;
        }
        panic!("Unexpected character: {}", self.peek())
    }

    fn identifier_or_keyword(&mut self) -> Option<Token> {
        // early return if not a valid identifier start
        if !self.peek().is_alphabetic() && self.peek() != '_' {
            return None
        }

        // consume the rest of the identifier
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text = self.selection();

        if let Ok(keyword) = Keyword::from_str(&text) {
            return Some(Token::Keyword(keyword));
        }
        Some(Token::Identifier(text))
    }

    fn operator(&mut self) -> Option<Token> {
        // early return if not a valid operator start
        if !Operator::is_operator_starting_char(self.peek()) {
            return None
        }

        // consume the rest of the operator
        let c = self.advance();
        match c {
            '+' => Some(Token::Operator(Operator::Plus)),
            '-' => Some(Token::Operator(Operator::Minus)),
            '*' => Some(Token::Operator(Operator::Multiply)),
            '/' => Some(Token::Operator(Operator::Divide)),
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    Some(Token::Operator(Operator::LessEqual))
                } else if self.peek() == '>' {
                    self.advance();
                    Some(Token::Operator(Operator::NotEqual))
                } else {
                    Some(Token::Operator(Operator::LessThan))
                }
            },
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    Some(Token::Operator(Operator::GreaterEqual))
                } else {
                    Some(Token::Operator(Operator::GreaterThan))
                }
            },
            '=' => Some(Token::Operator(Operator::Equal)),
            ':' => {
                if self.peek() == '=' {
                    self.advance();
                    if self.peek() == ':' {
                        self.advance();
                        Some(Token::Operator(Operator::Swap))
                    } else {
                        Some(Token::Operator(Operator::Assignment))
                    }
                } else {
                    self.rollback();
                    None
                }
            },
            _ => None,
        }
    }

    fn integer(&mut self) -> Option<Token> {
        if !self.peek().is_digit(10) {
            return None
        }

        while self.peek().is_digit(10) {
            self.advance();
        }
        let text = self.selection();
        Some(Token::Integer(text.parse().unwrap()))
    }

    fn char(&mut self) -> Option<Token> {
        if self.peek() != '\'' {
            return None
        }
        self.advance();

        let c = self.advance();

        if self.peek() != '\'' {
            panic!("Expected closing ' for char literal");
        }
        self.advance();

        Some(Token::Char(c))
    }

    fn string(&mut self) -> Option<Token> {
        if self.peek() != '"' {
            return None
        }
        self.advance();

        while self.peek() != '"' {
            if self.advance() == '\n' {
                panic!("Unexpected newline in string literal");
            }
        }
        self.advance();

        let text = self.selection();
        Some(Token::String(text))
    }

    fn comment(&mut self) -> Option<Token> {
        match self.peek() {
            '#' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
                Some(Token::Comment)
            },
            '{' => {
                while self.peek() != '}' && !self.is_at_end() {
                    if self.advance() == '\n' {
                        self.line += 1;
                    }
                }
                if self.is_at_end() {
                    panic!("Unexpected end of file in block comment");
                }
                self.advance();
                Some(Token::Comment)
            },
            _ => None,
        }
    }

    fn whitespace(&mut self) -> Option<Token> {
        if self.peek() != ' ' && self.peek() != '\t' && self.peek() != '\x0C' && self.peek() != '\x0B' {
            return None
        }

        while self.peek() == ' ' || self.peek() == '\t' || self.peek() == '\x0C' || self.peek() == '\x0B' {
            self.advance();
        }

        Some(Token::Whitespace)
    }

    fn special_token(&mut self) -> Option<Token> {
        match self.peek() {
            '\n' => {
                self.advance();
                self.line += 1;
                Some(Token::Newline)
            },
            ';' => {
                self.advance();
                Some(Token::Semicolon)
            },
            ',' => {
                self.advance();
                Some(Token::Comma)
            },
            '(' => {
                self.advance();
                Some(Token::LeftParen)
            },
            ')' => {
                self.advance();
                Some(Token::RightParen)
            },
            '.' => {
                self.advance();
                if self.peek() == '.' {
                    self.advance();
                    Some(Token::Dots)
                } else {
                    Some(Token::Dot)
                }
            }
            ':' => {
                self.advance();
                Some(Token::Colon)
            },
            _ => None,
        }
    }
}



