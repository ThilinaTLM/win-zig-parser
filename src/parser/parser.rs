#![allow(dead_code)]

use std::cell::Cell;
use crate::lexer::{Operator, Token};
use crate::lexer::Keyword;
use crate::parser::*;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {

        // Filter unnecessary tokens
        let tokens = tokens.into_iter().filter(|t| {
            match t {
                Token::Comment => false,
                Token::Whitespace => false,
                Token::Newline => false,
                _ => true,
            }
        }).collect::<Vec<Token>>();

        Parser {
            tokens,
            current: Cell::new(0),
            verbose: false,
        }
    }

    pub fn parse(&mut self) -> Program {
        self.winzig()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current.get()]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current.get() - 1]
    }

    fn is_at_end(&self) -> bool {
        self.current.get() >= self.tokens.len()
    }

    fn advance(&self) -> &Token {
        if !self.is_at_end() {
            self.current.set(self.current.get() + 1);
        }
        if self.verbose {
            println!("{}: {:?} -> {:?}", self.current.get() + 1, self.previous(), self.peek());
        }
        self.previous()
    }

    // Winzig -> 'program' Name ':' Consts Types Dclns SubProgs Body Name '.' => "program";
    fn winzig(&mut self) -> Program {
        let t = self.peek();
        if t == Token::Keyword(Keyword::Program) {
            self.advance();
            let name = self.name().expect("Expected program name");
            if self.peek() == &Token::Colon {
                self.advance();
            } else {
                panic!("Expected ':'");
            }

            let consts = self.consts();
            let types = self.types();
            let dclns = self.dclns();
            let sub_progs = self.sub_progs();
            let body = self.body().expect("Expected program body");

            let end_name = self.name().expect("Expected program end name");
            if self.peek() == Token::Dot {
                self.advance();
            } else {
                panic!("Expected '.'");
            }

            Program {
                name,
                consts,
                types,
                dclns,
                sub_progs,
                body,
                end_name,
            }
        } else {
            panic!("Expected program");
        }
    }

    // Name -> '<identifier>';
    fn name(&self) -> Option<Identifier> {
        let t = self.peek();
        if let Token::Identifier(name) = t {
            self.advance();
            Some(Identifier { name: name.clone() })
        } else {
            None
        }
    }

    // Consts -> 'const' Const list ',' ';' => "consts";
    // Consts -> => "consts";
    fn consts(&mut self) -> Consts {
        let t = self.peek();
        if t == Token::Keyword(Keyword::Const) {
            self.advance();
            let mut consts = Vec::new();
            loop {
                let c = self.const_expr();
                let t = self.peek();
                if t == Token::Comma {
                    self.advance();
                    consts.push(c);
                } else if t == Token::Comma {
                    self.advance();
                    consts.push(c);
                    break;
                } else {
                    panic!("Expected ',' or ';'");
                }
            }
            Consts {
                consts
            }
        } else {
            Consts {
                consts: Vec::new()
            }
        }
    }

    // Const -> Name '=' ConstValue => "const";
    fn const_expr(&mut self) -> Const {
        let name = self.name().expect("Expected const name");
        let t = self.peek();
        if t == Token::Operator(Operator::Equal) {
            self.advance();
            let value = self.const_value();
            Const {
                name: name.name,
                value,
            }
        } else {
            panic!("Expected '='");
        }
    }

    // ConstValue -> '<integer>';
    // ConstValue -> '<char>';
    // ConstValue -> Name;
    fn const_value(&mut self) -> ConstValue {
        let t = self.peek();
        if let Token::Integer(i) = t {
            self.advance();
            ConstValue::Integer(i.clone())
        } else if let Token::Char(c) = t {
            self.advance();
            ConstValue::Char(c.clone())
        } else if let Token::Identifier(name) = t {
            let name = self.name().expect("Expected const name");
            ConstValue::Name(name)
        } else {
            panic!("Expected const value");
        }
    }

    // Types -> 'type' (Type ';')+ => "types";
    // Types -> => "types";
    fn types(&mut self) -> Types {
        let t = self.peek();
        if t == Token::Keyword(Keyword::Type) {
            self.advance();
            let mut types = Vec::new();
            loop {
                match self.type_expr() {
                    Some(t) => {
                        types.push(t);
                        let t = self.peek();
                        if t == Token::Semicolon {
                            self.advance();
                        } else {
                            panic!("Expected ';'");
                        }
                    }
                    None => break,
                };
            }
            Types {
                types
            }
        } else {
            Types {
                types: Vec::new()
            }
        }
    }

    // Type -> Name '=' LitList => "type";
    fn type_expr(&mut self) -> Option<Type> {
        let name = match self.name() {
            Some(name) => name,
            None => return None,
        };
        let t = self.peek();
        if t == Token::Operator(Operator::Equal) {
            self.advance();
            let lit_list = self.lit_list();
            Some(Type {
                name,
                lit_list,
            })
        } else {
            panic!("Expected '='");
        }
    }

    // LitList -> '(' Name list ',' ')' => "lit";
    fn lit_list(&mut self) -> LitList {
        let t = self.peek();
        if t == Token::LeftParen {
            self.advance();
            let mut names = Vec::new();
            loop {
                let name = self.name().expect("Expected name");
                names.push(name);
                let t = self.peek();
                if t == Token::Comma {
                    self.advance();
                } else if t == Token::RightParen {
                    self.advance();
                    break;
                } else {
                    panic!("Expected ',' or ')'");
                }
            }
            LitList { names }
        } else {
            panic!("Expected '('");
        }
    }

    // Dclns -> 'var' (Dcln ';')+ => "dclns";
    // Dclns -> => "dclns";
    fn dclns(&mut self) -> Dclns {
        let t = self.peek();
        if t == Token::Keyword(Keyword::Var) {
            self.advance();
            let mut dclns = Vec::new();
            loop {
                let d = match self.dcln() {
                    Some(d) => d,
                    None => break,
                };
                dclns.push(d);
                let t = self.peek();
                if t == Token::Semicolon {
                    self.advance();
                } else {
                    panic!("Expected ';'");
                }
            }
            Dclns {
                vars: dclns,
            }
        } else {
            Dclns {
                vars: Vec::new()
            }
        }
    }

    // Dcln -> Name list ',' ':' Name => "var";
    fn dcln(&mut self) -> Option<Var> {
        // check if it's a identifier
        match self.peek() {
            Token::Identifier(_) => {},
            _ => return None,
        }

        // parse name list
        let mut names = vec![];
        loop {
            let name = self.name().expect("Expected name");
            names.push(name);
            let t = self.peek();
            if t == Token::Comma {
                self.advance();
            } else if t == Token::Colon {
                self.advance();
                break;
            } else {
                panic!("Expected ',' or ':'");
            }
        }

        // parse type name
        let name = self.name().expect("Expected name");
        Some(Var {
            names,
            typename: name,
        })
    }

    // Expression -> Term;
    // Expression -> Term '<=' Term => "<=";
    // Expression -> Term '<' Term => "<";
    // Expression -> Term '>=' Term => ">=";
    // Expression -> Term '>' Term => ">";
    // Expression -> Term '=' Term => "=";
    // Expression -> Term '<>' Term => "<>";
    fn expression(&self) -> Expression {
        let left = self.term();
        match self.peek() {
            Token::Operator(Operator::LessEqual) => {
                self.advance();
                let right = self.term();
                Expression::Le {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            Token::Operator(Operator::LessThan) => {
                self.advance();
                let right = self.term();
                Expression::Lt {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            Token::Operator(Operator::GreaterEqual) => {
                self.advance();
                let right = self.term();
                Expression::Ge {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            Token::Operator(Operator::GreaterThan) => {
                self.advance();
                let right = self.term();
                Expression::Gt {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            Token::Operator(Operator::Equal) => {
                self.advance();
                let right = self.term();
                Expression::Eq {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            Token::Operator(Operator::NotEqual) => {
                self.advance();
                let right = self.term();
                Expression::Ne {
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            _ => Expression::Term(Box::new(left)),
        }
    }


    // Term -> Factor Term_
    fn term(&self) -> Term {
        let f = self.factor();
        self.term_(Term::Factor(f))
    }

    // Term_ -> '+' Factor Term_ => "+"
    // Term_ -> '-' Factor Term_ => "-"
    // Term_ -> 'or' Factor Term_ => "or"
    // Term_ -> ε;
    fn term_(&self, t: Term) -> Term {
        match self.peek() {
            Token::Operator(Operator::Plus) => {
                self.advance();
                let f = self.factor();
                self.term_(Term::Add {
                    left: Box::new(t),
                    right: Box::new(f),
                })
            }
            Token::Operator(Operator::Minus) => {
                self.advance();
                let f = self.factor();
                self.term_(Term::Subtract {
                    left: Box::new(t),
                    right: Box::new(f),
                })
            }
            Token::Keyword(Keyword::Or) => {
                self.advance();
                let f = self.factor();
                self.term_(Term::Or {
                    left: Box::new(t),
                    right: Box::new(f),
                })
            }
            _ => t,
        }
    }

    // Factor -> Primary Factor_
    fn factor(&self) -> Factor {
        let p = self.primary();
        self.factor_(Factor::Primary(p))
    }

    // Factor_ -> '*' Primary Factor_ => "*"
    // Factor_ -> '/' Primary Factor_ => "/"
    // Factor_ -> 'and' Primary Factor_ => "and"
    // Factor_ -> 'mod' Primary Factor_ => "mod"
    // Factor_ -> ε;
    fn factor_(&self, f: Factor) -> Factor {
        match self.peek() {
            Token::Operator(Operator::Multiply) => {
                self.advance();
                let p = self.primary();
                self.factor_(Factor::Multiply {
                    left: Box::new(f),
                    right: Box::new(p),
                })
            }
            Token::Operator(Operator::Divide) => {
                self.advance();
                let p = self.primary();
                self.factor_(Factor::Divide {
                    left: Box::new(f),
                    right: Box::new(p),
                })
            }
            Token::Keyword(Keyword::And) => {
                self.advance();
                let p = self.primary();
                self.factor_(Factor::And {
                    left: Box::new(f),
                    right: Box::new(p),
                })
            }
            Token::Keyword(Keyword::Mod) => {
                self.advance();
                let p = self.primary();
                self.factor_(Factor::Mod {
                    left: Box::new(f),
                    right: Box::new(p),
                })
            }
            _ => f,
        }
    }

    // Primary -> '-' Primary => "-";
    // Primary -> '+' Primary;
    // Primary -> 'not' Primary => "not";
    // Primary -> 'eof' => "eof";
    // Primary -> Name;
    // Primary -> '<integer>';
    // Primary -> '<char>';
    // Primary -> Name '(' Expression list ',' ')' => "call";
    // Primary -> '(' Expression ')';
    // Primary -> 'succ' '(' Expression ')' => "succ";
    // Primary -> 'pred' '(' Expression ')' => "pred";
    // Primary -> 'chr' '(' Expression ')' => "chr";
    // Primary -> 'ord' '(' Expression ')' => "ord";
    fn primary(&self) -> Primary {
        let t = self.peek();
        match t {
            Token::Operator(Operator::Minus) => {
                self.advance();
                let p = self.primary();
                Primary::Negate {
                    primary: Box::new(p),
                }
            }
            Token::Operator(Operator::Plus) => {
                self.advance();
                self.primary()
            }
            Token::Keyword(Keyword::Not) => {
                self.advance();
                let p = self.primary();
                Primary::Not {
                    primary: Box::new(p),
                }
            }
            Token::Keyword(Keyword::Eof) => {
                self.advance();
                Primary::Eof
            }
            Token::Identifier(_) => {
                let name = self.name().expect("Expected identifier");
                if self.peek() == Token::LeftParen {
                    self.advance();
                    let mut exprs = Vec::new();
                    loop {
                        let expr = self.expression();
                        exprs.push(expr);
                        if self.peek() == Token::Comma {
                            self.advance();
                        } else if self.peek() == Token::RightParen {
                            self.advance();
                            break;
                        } else {
                            panic!("Expected ',' or ')'");
                        }
                    }
                    Primary::Call {
                        name,
                        exps: exprs,
                    }
                } else {
                    Primary::Name(name)
                }
            }
            Token::Integer(i) => {
                self.advance();
                Primary::Integer(i.clone())
            }
            Token::Char(c) => {
                self.advance();
                Primary::Char(c.clone())
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.expression();
                if self.peek() == Token::RightParen {
                    self.advance();
                    Primary::Expression(expr)
                } else {
                    panic!("Expected ')'");
                }
            }
            Token::Keyword(Keyword::Succ) => {
                self.advance();
                if self.peek() == Token::LeftParen {
                    self.advance();
                    let expr = self.expression();
                    if self.peek() == Token::RightParen {
                        self.advance();
                        Primary::Succ {
                            exp: Box::new(expr),
                        }
                    } else {
                        panic!("Expected ')'");
                    }
                } else {
                    panic!("Expected '('");
                }
            }
            Token::Keyword(Keyword::Pred) => {
                self.advance();
                if self.peek() == Token::LeftParen {
                    self.advance();
                    let expr = self.expression();
                    if self.peek() == Token::RightParen {
                        self.advance();
                        Primary::Pred {
                            exp: Box::new(expr),
                        }
                    } else {
                        panic!("Expected ')'");
                    }
                } else {
                    panic!("Expected '('");
                }
            }
            Token::Keyword(Keyword::Chr) => {
                self.advance();
                if self.peek() == Token::LeftParen {
                    self.advance();
                    let expr = self.expression();
                    if self.peek() == Token::RightParen {
                        self.advance();
                        Primary::Chr {
                            exp: Box::new(expr),
                        }
                    } else {
                        panic!("Expected ')'");
                    }
                } else {
                    panic!("Expected '('");
                }
            }
            Token::Keyword(Keyword::Ord) => {
                self.advance();
                if self.peek() == Token::LeftParen {
                    self.advance();
                    let expr = self.expression();
                    if self.peek() == Token::RightParen {
                        self.advance();
                        Primary::Ord {
                            exp: Box::new(expr),
                        }
                    } else {
                        panic!("Expected ')'");
                    }
                } else {
                    panic!("Expected '('");
                }
            }
            _ => panic!("Expected primary"),
        }
    }

    // Params -> Dcln list ';' => "params";
    fn params(&mut self) -> Params {
        let mut params = Vec::new();
        loop {
            let d = match self.dcln() {
                Some(d) => d,
                None => break,
            };
            params.push(d);
            if self.peek() == Token::Semicolon {
                self.advance();
            } else {
                break;
            }
        }
        Params { params }
    }

    // Body -> 'begin' Statement list ';' 'end' => "block";
    fn body(&mut self) -> Option<Body> {
        if self.peek() == Token::Keyword(Keyword::Begin) {
            self.advance();
            let mut statements = Vec::new();
            loop {
                let s = self.statement();
                statements.push(s);
                if self.peek() == Token::Semicolon {
                    self.advance();
                } else {
                    break;
                }
            }
            if self.peek() == Token::Keyword(Keyword::End) {
                self.advance();
                Some(Body { statements })
            } else {
                panic!("Expected 'end'");
            }
        } else {
            None
        }
    }

    // Statement -> 'output' '(' OutExp list ',' ')' => "output";
    // Statement -> 'if' Expression 'then' Statement ('else' Statement)? => "if";
    // Statement -> 'while' Expression 'do' Statement => "while";
    // Statement -> 'repeat' Statement list ';' 'until' Expression => "repeat";
    // Statement -> 'for' '(' ForStat ';' ForExp ';' ForStat ')' Statement => "for";
    // Statement -> 'loop' Statement list ';' 'pool' => "loop";
    // Statement -> 'case' Expression 'of' Caseclauses OtherwiseClause 'end' => "case";
    // Statement -> 'read' '(' Name list ',' ')' => "read";
    // Statement -> 'exit' => "exit";
    // Statement -> 'return' Expression => "return";
    // Statement -> Body;
    // Statement -> Assignment;
    // Statement -> => "<null>";
    fn statement(&mut self) -> Statement {
        match self.peek() {
            Token::Keyword(Keyword::Output) => {
                self.advance();
                if self.peek() == Token::LeftParen {
                    self.advance();
                    let outexps = self._statement_outexp_list();
                    if self.peek() == Token::RightParen {
                        self.advance();
                        Statement::Output {
                            expressions: outexps,
                        }
                    } else {
                        panic!("Expected ')'");
                    }
                } else {
                    panic!("Expected '('");
                }
            }
            Token::Keyword(Keyword::If) => {
                self.advance();
                let expr = self.expression();
                if self.peek() == Token::Keyword(Keyword::Then) {
                    self.advance();
                    let stmt = self.statement();
                    let else_stmt = if self.peek() == Token::Keyword(Keyword::Else) {
                        self.advance();
                        Some(self.statement())
                    } else {
                        None
                    };
                    Statement::If {
                        cond: expr,
                        then: Box::new(stmt),
                        else_stmt: else_stmt.map(Box::new),
                    }
                } else {
                    panic!("Expected 'then'");
                }
            }
            Token::Keyword(Keyword::While) => {
                self.advance();
                let expr = self.expression();
                if self.peek() == Token::Keyword(Keyword::Do) {
                    self.advance();
                    let stmt = self.statement();
                    Statement::While {
                        cond: expr,
                        stmt: Box::new(stmt),
                    }
                } else {
                    panic!("Expected 'do'");
                }
            }
            Token::Keyword(Keyword::Repeat) => {
                self.advance();
                let mut stmts = Vec::new();
                loop {
                    let stmt =  self.statement();
                    stmts.push(stmt);

                    if self.peek() == Token::Semicolon {
                        self.advance();
                    } else {
                        break;
                    }

                }
                if self.peek() == Token::Keyword(Keyword::Until) {
                    self.advance();
                    let expr = self.expression();
                    Statement::Repeat {
                        stmts,
                        cond: expr,
                    }
                } else {
                    panic!("Expected 'until'");
                }
            }
            Token::Keyword(Keyword::For) => {
                self.advance();
                if self.peek() != Token::LeftParen {
                    panic!("Expected '('");
                }
                self.advance();

                let init = match self.assignment() {
                    Some(a) => ForStat::Assignment(a),
                    None => ForStat::Null,
                };
                if self.peek() != Token::Semicolon {
                    panic!("Expected ';'");
                }
                self.advance();

                let cond  = if self.peek() == Token::Semicolon {
                    self.advance();
                    ForExp::True
                } else {
                    let cond = self.expression();
                    if self.peek() != Token::Semicolon {
                        panic!("Expected ';'");
                    }
                    self.advance();
                    ForExp::Expression(cond)
                };

                let update = match self.assignment() {
                    Some(a) => ForStat::Assignment(a),
                    None => ForStat::Null,
                };
                if self.peek() != Token::RightParen {
                    panic!("Expected ')'");
                }

                self.advance();
                let stmt = self.statement();
                Statement::For {
                    init,
                    cond,
                    update,
                    stmt: Box::new(stmt),
                }
            }
            Token::Keyword(Keyword::Loop) => {
                self.advance();
                let stmts = self._statement_list();
                if self.peek() == Token::Keyword(Keyword::Pool) {
                    self.advance();
                    Statement::Loop {
                        stmts,
                    }
                } else {
                    panic!("Expected 'pool'");
                }
            }
            Token::Keyword(Keyword::Case) => {
                // Statement -> 'case' Expression 'of' Caseclauses OtherwiseClause 'end' => "case";
                self.advance();
                let expr = self.expression();
                if self.peek() != Token::Keyword(Keyword::Of) {
                    panic!("Expected 'of'");
                }
                self.advance();

                // case clauses
                let mut case_clauses = Vec::new();
                loop {
                    let clause = self.case_clause();
                    case_clauses.push(clause);
                    if self.peek() == Token::Semicolon {
                        self.advance();
                    }
                    if self.peek() == Token::Keyword(Keyword::End) {
                        break;
                    }
                    if self.peek() == Token::Keyword(Keyword::Otherwise) {
                        break;
                    }
                }

                // otherwise clause
                let otherwise_clause = self.otherwise_clause();

                // end case
                if self.peek() != Token::Keyword(Keyword::End) {
                    panic!("Expected 'end'");
                }
                self.advance();


                Statement::Case {
                    expr,
                    cases: case_clauses,
                    otherwise: otherwise_clause.map(Box::new),
                }
            }
            Token::Keyword(Keyword::Read) => {
                self.advance();
                if self.peek() == Token::LeftParen {
                    self.advance();
                    let names = self._name_list();
                    if self.peek() == Token::RightParen {
                        self.advance();
                        Statement::Read {
                            names,
                        }
                    } else {
                        panic!("Expected ')'");
                    }
                } else {
                    panic!("Expected '('");
                }
            }
            Token::Keyword(Keyword::Exit) => {
                self.advance();
                Statement::Exit
            }
            Token::Keyword(Keyword::Return) => {
                self.advance();
                let expr = self.expression();
                Statement::Return {
                    exp: expr,
                }
            }
            Token::Keyword(Keyword::Begin) => {
                Statement::Body{
                    body: self.body().expect("Expected body"),
                }
            }
            Token::Identifier(_) => {
                Statement::Assign {
                    assignment: self.assignment().expect("Expected assignment"),
                }
            }
            _ => Statement::Null,
        }
    }

    fn _statement_outexp_list(&mut self) -> Vec<OutExp> {
        let mut outexps = Vec::new();
        loop {
            let outexp = self.outexp();
            outexps.push(outexp);
            if self.peek() == Token::Comma {
                self.advance();
            } else {
                break;
            }
        }
        outexps
    }

    fn _statement_list(&mut self) -> Vec<Statement> {
        let mut stmts = Vec::new();
        loop {
            let stmt = self.statement();
            stmts.push(stmt);
            if self.peek() == Token::Semicolon {
                self.advance();
            } else {
                break;
            }
        }
        stmts
    }

    fn _name_list(&mut self) -> Vec<Identifier> {
        let mut names = Vec::new();
        loop {
            let name = self.name().expect("Expected name");
            names.push(name);
            if self.peek() == Token::Comma {
                self.advance();
            } else {
                break;
            }
        }
        names
    }

    // OutExp -> Expression => "integer";
    // OutExp -> StringNode => "string";
    fn outexp(&mut self) -> OutExp {
        let expr = self.expression();
        match self.peek() {
            Token::String(s) => {
                self.advance();
                OutExp::String{
                    value: s.to_string(),
                }
            }
            _ => OutExp::Integer{
                exp: expr,
            }
        }
    }

    // Assignment -> Name ':=' Expression => "assign";
    // Assignment -> Name ':=:' Name => "swap";
    fn assignment(&mut self) -> Option<Assignment> {
        let name1 = match self.peek() {
            Token::Identifier(name) => Identifier { name: name.to_string()},
            _ => return None,
        };
        self.advance();
        match self.peek() {
            Token::Operator(Operator::Assignment) => {
                self.advance();
                let expr = self.expression();
                Some(Assignment::Assignment {
                    name: name1,
                    exp: expr,
                })
            }
            Token::Operator(Operator::Swap) => {
                self.advance();
                let name2 = match self.peek() {
                    Token::Identifier(name) => Identifier { name: name.to_string()},
                    _ => panic!("Expected identifier"),
                };
                self.advance();
                Some(Assignment::Swap {
                    name1,
                    name2,
                })
            }
            _ => panic!("Expected ':=' or ':=:'"),
        }
    }

    // Caseclause -> CaseExpression list ',' ':' Statement => "case_clause";
    fn case_clause(&mut self) -> CaseClause {
        let mut exprs = Vec::new();
        loop {
            let expr = self.case_expression();
            exprs.push(expr);
            if self.peek() == Token::Comma {
                self.advance();
            } else if self.peek() == Token::Colon {
                self.advance();
                break;
            } else {
                panic!("Expected ',' or ':'");
            }
        }
        let stmt = self.statement();
        CaseClause {
            expressions: exprs,
            statement: stmt,
        }
    }

    // CaseExpression -> ConstValue;
    // CaseExpression -> ConstValue '..' ConstValue => "..";
    fn case_expression(&mut self) -> CaseExpression {
        let expr1 = self.const_value();
        if self.peek() == Token::Dots {
            self.advance();
            let expr2 = self.const_value();
            CaseExpression::Range(expr1, expr2)
        } else {
            CaseExpression::Value(expr1)
        }
    }

    // OtherwiseClause -> 'otherwise' Statement => "otherwise";
    // OtherwiseClause -> ;
    fn otherwise_clause(&mut self) -> Option<OtherwiseClause> {
        if self.peek() == Token::Keyword(Keyword::Otherwise) {
            self.advance();
            let stmt = self.statement();
            Some(OtherwiseClause {
                stmt,
            })
        } else {
            None
        }
    }

    // SubProgs -> Fcn* => "subprogs";
    fn sub_progs(&mut self) -> SubProgs {
        let mut sub_progs = Vec::new();
        loop {
            match self.func() {
                Some(func) => sub_progs.push(func),
                None => break,
            }
        }
        SubProgs {
            sub_progs,
        }
    }

    // Fcn -> 'function' Name '(' Params ')' ':' Name ';' Consts Types Dclns Body Name ';' => "fcn";
    fn func(&mut self) -> Option<Func> {
        if self.peek() != Token::Keyword(Keyword::Function) {
            return None;
        }
        self.advance();
        let name = self.name().expect("Expected name");
        if self.peek() != Token::LeftParen {
            panic!("Expected '('");
        }
        self.advance();
        let params = self.params();
        if self.peek() != Token::RightParen {
            panic!("Expected ')'");
        }
        self.advance();
        if self.peek() != Token::Colon {
            panic!("Expected ':'");
        }
        self.advance();
        let return_type = self.name().expect("Expected name");
        if self.peek() != Token::Semicolon {
            panic!("Expected ';'");
        }
        self.advance();
        let consts = self.consts();
        let types = self.types();
        let dclns = self.dclns();
        let body = self.body().expect("Expected body");
        let end_name = self.name().expect("Expected name");
        if self.peek() != Token::Semicolon {
            panic!("Expected ';'");
        }
        self.advance();
        Some(Func {
            name,
            params,
            return_type,
            consts,
            types,
            dclns,
            body,
            end_name,
        })
    }
}

