#![allow(dead_code)]

use std::cell::Cell;

use crate::lexer::{Token};


mod test;
mod parser;
pub mod tree;

pub struct Parser {
    tokens: Vec<Token>,
    current: Cell<usize>,
    verbose: bool,
}

pub struct Program {
    name: Identifier,
    consts: Consts,
    types: Types,
    dclns: Dclns,
    sub_progs: SubProgs,
    body: Body,
    end_name: Identifier,
}

pub struct Identifier {
    name: String,
}

pub struct Consts {
    consts: Vec<Const>,
}

pub struct Const {
    name: String,
    value: ConstValue,
}

pub struct Dclns {
    vars: Vec<Var>,
}

pub struct Types {
    types: Vec<Type>,
}

pub enum ConstValue {
    Integer(i64),
    Char(char),
    Name(Identifier),
}

pub struct Type {
    name: Identifier,
    lit_list: LitList,
}

pub struct LitList {
    names: Vec<Identifier>,
}

pub enum Fcn {
    Fcn { name: String, params: Vec<Var>, consts: Vec<Const>, types: Vec<Type>, dclns: Vec<Var>, body: Body },
}

pub struct Var {
    names: Vec<Identifier>,
    typename: Identifier,
}

pub struct Body {
    statements: Vec<Statement>,
}

pub enum Statement {
    Assign { assignment: Assignment },
    Output { expressions: Vec<OutExp> },
    If { cond: Expression, then: Box<Statement>, else_stmt: Option<Box<Statement>> },
    While { cond: Expression, stmt: Box<Statement> },
    Repeat { stmts: Vec<Statement>, cond: Expression },
    For {
        init: ForStat,
        cond: ForExp,
        update: ForStat,
        stmt: Box<Statement>,
    },
    Loop { stmts: Vec<Statement> },
    Case {
        expr: Expression,
        cases: Vec<CaseClause>,
        otherwise: Option<Box<OtherwiseClause>>,
    },
    Read { names: Vec<Identifier> },
    Exit,
    Return { exp: Expression },
    Body { body: Body },
    Null,
}

pub enum ForStat {
    Assignment(Assignment),
    Null,
}

pub enum ForExp {
    Expression(Expression),
    True,
}

pub enum Assignment {
    Assignment { name: Identifier, exp: Expression },
    Swap { name1: Identifier, name2: Identifier },
}

pub enum OutExp {
    Integer { exp: Expression },
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

pub struct OtherwiseClause {
    stmt: Statement,
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
    Name(Identifier),
    Integer(i64),
    Char(char),
    Call { name: Identifier, exps: Vec<Expression> },
    Expression(Expression),
    Succ { exp: Box<Expression> },
    Pred { exp: Box<Expression> },
    Chr { exp: Box<Expression> },
    Ord { exp: Box<Expression> },
}

pub struct Params {
    params: Vec<Var>,
}

pub struct Func {
    name: Identifier,
    params: Params,
    return_type: Identifier,
    consts: Consts,
    types: Types,
    dclns: Dclns,
    body: Body,
    end_name: Identifier,
}

pub struct SubProgs {
    sub_progs: Vec<Func>,
}
