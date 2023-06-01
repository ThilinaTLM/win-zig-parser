use crate::parser::*;

pub trait StringTree {
    fn get_string_tree(&self, level: usize) -> Vec<String>;
}

impl StringTree for Program {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();
        tree.push(format!("{}program(7)", ". ".repeat(level)));
        tree.append(&mut self.name.get_string_tree(level + 1));
        tree.append(&mut self.consts.get_string_tree(level + 1));
        tree.append(&mut self.types.get_string_tree(level + 1));
        tree.append(&mut self.dclns.get_string_tree(level + 1));
        tree.append(&mut self.sub_progs.get_string_tree(level + 1));
        tree.append(&mut self.body.get_string_tree(level + 1));
        tree.append(&mut self.end_name.get_string_tree(level + 1));
        return tree;
    }
}

impl StringTree for Identifier {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        return vec![
            format!("{}<identifier>(1)", ". ".repeat(level)),
            format!("{}{}(0)", ". ".repeat(level + 1), self.name),
        ];
    }
}

impl StringTree for Consts {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        let consts_len = self.consts.len();

        tree.push(format!("{}consts({})", ". ".repeat(level), consts_len));
        for const_ in &self.consts {
            // tree.append(&mut const_.get_string_tree(level + 1));
        }
        return tree;
    }
}

impl StringTree for Types {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        let types_len = self.types.len();

        tree.push(format!("{}types({})", ". ".repeat(level), types_len));
        for type_ in &self.types {
            tree.append(&mut type_.get_string_tree(level + 1));
        }
        return tree;
    }
}

impl StringTree for Type {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        let len = 2;

        tree.push(format!("{}type({})", ". ".repeat(level), len));
        tree.append(&mut self.name.get_string_tree(level + 1));
        tree.append(&mut self.lit_list.get_string_tree(level + 1));
        return tree;
    }
}

impl StringTree for LitList {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        let len = self.names.len();

        tree.push(format!("{}lit({})", ". ".repeat(level), len));
        for name in &self.names {
            tree.append(&mut name.get_string_tree(level + 1));
        }
        return tree;
    }
}

impl StringTree for Dclns {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        let dclns_len = self.vars.len();

        tree.push(format!("{}dclns({})", ". ".repeat(level), dclns_len));
        for var in &self.vars {
            tree.append(&mut var.get_string_tree(level + 1));
        }
        return tree;
    }
}

impl StringTree for Var {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        let len = self.names.len() + 1;

        tree.push(format!("{}var({})", ". ".repeat(level), len));
        for name in &self.names {
            tree.append(&mut name.get_string_tree(level + 1));
        }
        tree.append(&mut self.typename.get_string_tree(level + 1));
        return tree;
    }
}

impl StringTree for SubProgs {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        let len = self.sub_progs.len();

        tree.push(format!("{}subprogs({})", ". ".repeat(level), len));
        for sub_prog in &self.sub_progs {
            tree.append(&mut sub_prog.get_string_tree(level + 1));
        }
        return tree;
    }
}

impl StringTree for Func {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        let len = 8; // fixed length

        tree.push(format!("{}fcn({})", ". ".repeat(level), len));
        tree.append(&mut self.name.get_string_tree(level + 1));
        tree.append(&mut self.params.get_string_tree(level + 1));
        tree.append(&mut self.return_type.get_string_tree(level + 1));
        tree.append(&mut self.consts.get_string_tree(level + 1));
        tree.append(&mut self.types.get_string_tree(level + 1));
        tree.append(&mut self.dclns.get_string_tree(level + 1));
        tree.append(&mut self.body.get_string_tree(level + 1));
        tree.append(&mut self.end_name.get_string_tree(level + 1));
        return tree;
    }
}

impl StringTree for Params {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        let len = self.params.len();

        tree.push(format!("{}params({})", ". ".repeat(level), len));
        for param in &self.params {
            tree.append(&mut param.get_string_tree(level + 1));
        }
        return tree;
    }
}

impl StringTree for Body {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        let len = self.statements.len();

        tree.push(format!("{}block({})", ". ".repeat(level), len));
        for statement in &self.statements {
            tree.append(&mut statement.get_string_tree(level + 1));
        }
        return tree;
    }
}

impl StringTree for Statement {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        match self {
            Statement::Assign { assignment } => {
                return assignment.get_string_tree(level);
            }
            Statement::Output { expressions } => {
                let len = expressions.len();
                tree.push(format!("{}output({})", ". ".repeat(level), len));
                for expression in expressions {
                    tree.append(&mut expression.get_string_tree(level + 1));
                }
            }
            Statement::If { cond, then, else_stmt } => {
                let len = if else_stmt.is_some() { 3 } else { 2 };
                tree.push(format!("{}if({})", ". ".repeat(level), len));
                tree.append(&mut cond.get_string_tree(level + 1));
                tree.append(&mut then.get_string_tree(level + 1));
                if let Some(else_stmt) = else_stmt {
                    tree.append(&mut else_stmt.get_string_tree(level + 1));
                }
            }
            Statement::While { cond, stmt } => {
                let len = 2;
                tree.push(format!("{}while({})", ". ".repeat(level), len));
                tree.append(&mut cond.get_string_tree(level + 1));
                tree.append(&mut stmt.get_string_tree(level + 1));
            }
            Statement::Repeat { stmts, cond } => {
                let len = stmts.len() + 1;
                tree.push(format!("{}repeat({})", ". ".repeat(level), len));
                for stmt in stmts {
                    tree.append(&mut stmt.get_string_tree(level + 1));
                }
                tree.append(&mut cond.get_string_tree(level + 1));
            }
            Statement::For { init, cond, update, stmt } => {
                let len = 4;
                tree.push(format!("{}for({})", ". ".repeat(level), len));
                tree.append(&mut init.get_string_tree(level + 1));
                tree.append(&mut cond.get_string_tree(level + 1));
                tree.append(&mut update.get_string_tree(level + 1));
                tree.append(&mut stmt.get_string_tree(level + 1));
            }
            Statement::Loop {stmts} => {
                let len = stmts.len();
                tree.push(format!("{}loop({})", ". ".repeat(level), len));
                for stmt in stmts {
                    tree.append(&mut stmt.get_string_tree(level + 1));
                }
            }
            Statement::Case {expr, cases, otherwise} => {
                let len = cases.len() + if otherwise.is_some() { 1 } else { 0 } + 1;
                tree.push(format!("{}case({})", ". ".repeat(level), len));
                tree.append(&mut expr.get_string_tree(level + 1));
                for case in cases {
                    tree.append(&mut case.get_string_tree(level + 1));
                }
                if let Some(otherwise) = otherwise {
                    tree.append(&mut otherwise.get_string_tree(level + 1));
                }
            }
            Statement::Read {names} => {
                let len = names.len();
                tree.push(format!("{}read({})", ". ".repeat(level), len));
                for name in names {
                    tree.append(&mut name.get_string_tree(level + 1));
                }
            }
            Statement::Exit => {
                tree.push(format!("{}exit(0)", ". ".repeat(level)));
            }
            Statement::Return {exp} => {
                let len = 1;
                tree.push(format!("{}return({})", ". ".repeat(level), len));
                tree.append(&mut exp.get_string_tree(level + 1));
            }
            Statement::Body {body} => {
                return body.get_string_tree(level);
            }
            Statement::Null => {
                tree.push(format!("{}<null>(0)", ". ".repeat(level)));
            }
        }
        return tree;
    }
}

impl StringTree for CaseClause {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        let len = self.expressions.len() + 1;

        tree.push(format!("{}case_clause({})", ". ".repeat(level), len));
        for expression in &self.expressions {
            tree.append(&mut expression.get_string_tree(level + 1));
        }
        tree.append(&mut self.statement.get_string_tree(level + 1));
        return tree;
    }
}

impl StringTree for CaseExpression {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        match self {
            CaseExpression::Value(value) => {
                return value.get_string_tree(level);
            }
            CaseExpression::Range(c1, c2) => {
                let len = 2;
                tree.push(format!("{}..({})", ". ".repeat(level), len));
                tree.append(&mut c1.get_string_tree(level + 1));
                tree.append(&mut c2.get_string_tree(level + 1));
            }
        }
        return tree;
    }
}

impl StringTree for ConstValue {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        match self {
            ConstValue::Integer(i) => {
                let len = 1;
                tree.push(format!("{}<integer>({})", ". ".repeat(level), len));
                tree.push(format!("{}{}(0)", ". ".repeat(level + 1), i));
            }
            ConstValue::Char(c) => {
                let len = 1;
                tree.push(format!("{}<char>({})", ". ".repeat(level), len));
                tree.push(format!("{}{}(0)", ". ".repeat(level + 1), c));
            }
            ConstValue::Name(name) => {
                return name.get_string_tree(level);
            }
        }
        return tree;
    }
}

impl StringTree for OtherwiseClause {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();
        let len = 1;
        tree.push(format!("{}otherwise({})", ". ".repeat(level), len));
        tree.append(&mut self.stmt.get_string_tree(level + 1));
        return tree;
    }
}


impl StringTree for ForStat {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();
        match self {
            ForStat::Assignment(assign) => {
                return assign.get_string_tree(level);
            }
            ForStat::Null => {
                tree.push(format!("{}<null>(0)", ". ".repeat(level)));
            }
        }
        return tree;
    }
}

impl StringTree for ForExp {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();
        match self {
            ForExp::Expression(exp) => {
                return exp.get_string_tree(level);
            }
            ForExp::True => {
                tree.push(format!("{}true(0)", ". ".repeat(level)));
            }
        }
        return tree;
    }
}

impl StringTree for Assignment {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();
        let len = 2;
        match self {
            Assignment::Assignment { name, exp } => {
                tree.push(format!("{}assign({})", ". ".repeat(level), len));
                tree.append(&mut name.get_string_tree(level + 1));
                tree.append(&mut exp.get_string_tree(level + 1));
            }
            Assignment::Swap { name1, name2 } => {
                tree.push(format!("{}swap({})", ". ".repeat(level), len));
                tree.append(&mut name1.get_string_tree(level + 1));
                tree.append(&mut name2.get_string_tree(level + 1));
            }
        }
        return tree;
    }
}

impl StringTree for Expression {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        match self {
            Expression::Le { left, right } => {
                let len = 2;
                tree.push(format!("{}<=({})", ". ".repeat(level), len));
                tree.append(&mut left.get_string_tree(level + 1));
                tree.append(&mut right.get_string_tree(level + 1));
            }
            Expression::Lt { left, right } => {
                let len = 2;
                tree.push(format!("{}<({})", ". ".repeat(level), len));
                tree.append(&mut left.get_string_tree(level + 1));
                tree.append(&mut right.get_string_tree(level + 1));
            }
            Expression::Ge { left, right } => {
                let len = 2;
                tree.push(format!("{}>=({})", ". ".repeat(level), len));
                tree.append(&mut left.get_string_tree(level + 1));
                tree.append(&mut right.get_string_tree(level + 1));
            }
            Expression::Gt { left, right } => {
                let len = 2;
                tree.push(format!("{}>({})", ". ".repeat(level), len));
                tree.append(&mut left.get_string_tree(level + 1));
                tree.append(&mut right.get_string_tree(level + 1));
            }
            Expression::Eq { left, right } => {
                let len = 2;
                tree.push(format!("{}=({})", ". ".repeat(level), len));
                tree.append(&mut left.get_string_tree(level + 1));
                tree.append(&mut right.get_string_tree(level + 1));
            }
            Expression::Ne { left, right } => {
                let len = 2;
                tree.push(format!("{}<>({})", ". ".repeat(level), len));
                tree.append(&mut left.get_string_tree(level + 1));
                tree.append(&mut right.get_string_tree(level + 1));
            }
            Expression::Term(term) => {
                return term.get_string_tree(level);
            }
        }
        return tree;
    }
}

impl StringTree for Term {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        match self {
            Term::Add { left, right } => {
                let len = 2;
                tree.push(format!("{}+({})", ". ".repeat(level), len));
                tree.append(&mut left.get_string_tree(level + 1));
                tree.append(&mut right.get_string_tree(level + 1));
            }
            Term::Subtract { left, right } => {
                let len = 2;
                tree.push(format!("{}-({})", ". ".repeat(level), len));
                tree.append(&mut left.get_string_tree(level + 1));
                tree.append(&mut right.get_string_tree(level + 1));
            }
            Term::Or { left, right } => {
                let len = 2;
                tree.push(format!("{}or({})", ". ".repeat(level), len));
                tree.append(&mut left.get_string_tree(level + 1));
                tree.append(&mut right.get_string_tree(level + 1));
            }
            Term::Factor(factor) => {
                return factor.get_string_tree(level);
            }
        }
        return tree;
    }
}

impl StringTree for Factor {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        match self {
            Factor::Multiply { left, right } => {
                let len = 2;
                tree.push(format!("{}*({})", ". ".repeat(level), len));
                tree.append(&mut left.get_string_tree(level + 1));
                tree.append(&mut right.get_string_tree(level + 1));
            }
            Factor::Divide { left, right } => {
                let len = 2;
                tree.push(format!("/({})", len));
                tree.append(&mut left.get_string_tree(level + 1));
                tree.append(&mut right.get_string_tree(level + 1));
            }
            Factor::And { left, right } => {
                let len = 2;
                tree.push(format!("{}and({})", ". ".repeat(level), len));
                tree.append(&mut left.get_string_tree(level + 1));
                tree.append(&mut right.get_string_tree(level + 1));
            }
            Factor::Mod { left, right } => {
                let len = 2;
                tree.push(format!("{}mod({})", ". ".repeat(level), len));
                tree.append(&mut left.get_string_tree(level + 1));
                tree.append(&mut right.get_string_tree(level + 1));
            }
            Factor::Primary(primary) => {
                return primary.get_string_tree(level);
            }
        }
        return tree;
    }
}

impl StringTree for Primary {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        match self {
            Primary::Negate { primary } => {
                let len = 1;
                tree.push(format!("-({})", len));
                tree.append(&mut primary.get_string_tree(level + 1));
            }
            Primary::Not { primary } => {
                let len = 1;
                tree.push(format!("{}not({})", ". ".repeat(level), len));
                tree.append(&mut primary.get_string_tree(level + 1));
            }
            Primary::Eof => {
                let len = 0;
                tree.push(format!("{}eof({})", ". ".repeat(level), len));
            }
            Primary::Name(name) => {
                return name.get_string_tree(level);
            }
            Primary::Integer(number) => {
                let len = 1;
                tree.push(format!("{}<integer>({})", ". ".repeat(level), len));
                tree.push(format!("{}{}(0)", ". ".repeat(level + 1), number));
            }
            Primary::Char(ch) => {
                let len = 1;
                tree.push(format!("{}<char>({})", ". ".repeat(level), len));
                tree.push(format!("{}{}(0)", ". ".repeat(level + 1), ch));
            }
            Primary::Call { name, exps } => {
                let len = exps.len() + 1;
                tree.push(format!("{}call({})", ". ".repeat(level), len));
                tree.append(&mut name.get_string_tree(level + 1));
                for exp in exps {
                    tree.append(&mut exp.get_string_tree(level + 1));
                }
            }
            Primary::Expression(exp) => {
                return exp.get_string_tree(level);
            }
            Primary::Succ { exp } => {
                let len = 1;
                tree.push(format!("{}succ({})", ". ".repeat(level), len));
                tree.append(&mut exp.get_string_tree(level + 1));
            }
            Primary::Pred { exp } => {
                let len = 1;
                tree.push(format!("{}pred({})", ". ".repeat(level), len));
                tree.append(&mut exp.get_string_tree(level + 1));
            }
            Primary::Chr { exp } => {
                let len = 1;
                tree.push(format!("{}chr({})", ". ".repeat(level), len));
                tree.append(&mut exp.get_string_tree(level + 1));
            }
            Primary::Ord { exp } => {
                let len = 1;
                tree.push(format!("{}ord({})", ". ".repeat(level), len));
                tree.append(&mut exp.get_string_tree(level + 1));
            }
        }
        return tree;
    }
}

impl StringTree for OutExp {
    fn get_string_tree(&self, level: usize) -> Vec<String> {
        let mut tree = Vec::new();

        match self {
            OutExp::Integer { exp } => {
                let len = 1;
                tree.push(format!("{}integer({})", ". ".repeat(level), len));
                tree.append(&mut exp.get_string_tree(level + 1));
            }
            OutExp::String { value } => {
                let len = 1;
                tree.push(format!("{}<string>({})", ". ".repeat(level), len));
                tree.push(format!("{}{}(0)", ". ".repeat(level + 1), value));
            }
        }
        return tree;
    }
}