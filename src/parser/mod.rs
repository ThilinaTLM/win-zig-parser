#![allow(dead_code)]

use std::cell::Cell;
use crate::lexer::{Lexer, Token};

mod test;
mod parser;

pub struct Parser {
    tokens: Vec<Token>,
    start: Cell<usize>,
    current: Cell<usize>,
}

pub struct Program {
    name: Name,
    consts: Option<Vec<Const>>,
    types: Option<Vec<Type>>,
    dclns: Option<Dclns>,
    sub_progs: SubProgs,
    body: Body,
}

pub struct Dclns {
    dclns: Vec<Var>,
}

pub struct Name {
    name: String,
}

pub struct Const {
    name: String,
    value: ConstValue,
}

pub enum ConstValue {
    Integer(i64),
    Char(char),
    Name(String),
}

pub struct Type {
    name: String,
    lit_list: LitList,
}

pub struct LitList {
    names: Vec<String>,
}

pub enum Fcn {
    Fcn { name: String, params: Vec<Var>, consts: Vec<Const>, types: Vec<Type>, dclns: Vec<Var>, body: Body },
}

pub struct Var {
    names: Vec<String>,
    typename: String,
}

pub struct Body {
    statements: Vec<Statement>
}

pub enum Statement {
    Assign { assignment: Assignment },
    Swap { name1: String, name2: String },
    Output { expressions: Vec<OutExp> },
    If { condition: Expression, then_stmt: Box<Statement>, else_stmt: Option<Box<Statement>> },
    While { condition: Expression, stmt: Box<Statement> },
    Repeat { stmts: Vec<Statement>, condition: Expression },
    For { for_stat: Option<Assignment>, for_exp: Expression, stmt: Box<Statement> },
    Loop { stmts: Vec<Statement> },
    Case { condition: Expression, case_clauses: Vec<CaseClause>, otherwise_clause: Box<OtherwiseClause> },
    Read { names: Vec<String> },
    Exit,
    Return { expression: Expression },
    Body { body: Body },
    Null,
}

pub enum Assignment {
    Assignment { name: String, expression: Expression },
    Swap { name1: String, name2: String },
}

pub enum OutExp {
    Integer { expression: Expression },
    String { value: String },
}

pub struct CaseClause {
    expressions: Vec<CaseExpression>,
    statement: Statement,
}

pub enum CaseExpression {
    Value(ConstValue),
    Range(ConstValue, ConstValue),
}

pub enum OtherwiseClause {
    Statement(Statement),
}



pub enum Expression {
    Le { left: Box<Term>, right: Box<Term> },
    Lt { left: Box<Term>, right: Box<Term> },
    Ge { left: Box<Term>, right: Box<Term> },
    Gt { left: Box<Term>, right: Box<Term> },
    Eq { left: Box<Term>, right: Box<Term> },
    Ne { left: Box<Term>, right: Box<Term> },
    Term(Box<Term>),
}

pub enum Term {
    Add { left: Box<Term>, right: Box<Factor> },
    Subtract { left: Box<Term>, right: Box<Factor> },
    Or { left: Box<Term>, right: Box<Factor> },
    Factor(Factor),
}

pub enum Factor {
    Multiply { left: Box<Factor>, right: Box<Primary> },
    Divide { left: Box<Factor>, right: Box<Primary> },
    And { left: Box<Factor>, right: Box<Primary> },
    Mod { left: Box<Factor>, right: Box<Primary> },
    Primary(Primary),
}


pub enum Primary {
    Negate { primary: Box<Primary> },
    Not { primary: Box<Primary> },
    Eof,
    Name(String),
    Integer(i64),
    Char(char),
    Call { name: String, expressions: Vec<Expression> },
    Expression(Expression),
    Succ { expression: Box<Expression> },
    Pred { expression: Box<Expression> },
    Chr { expression: Box<Expression> },
    Ord { expression: Box<Expression> },
}

pub struct Params {
    params: Vec<Var>,
}

pub struct Func {
    name: String,
    params: Params,
    return_type: String,
    consts: Option<Vec<Const>>,
    types: Option<Vec<Type>>,
    dclns: Option<Dclns>,
    body: Body,
    end_name: String,
}

pub struct SubProgs {
    sub_progs: Vec<Func>,
}
