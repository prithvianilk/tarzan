use std::fmt::{Display, Formatter, Pointer};
use crate::token;

pub enum Statement {
    Let(LetStatement),
    Return(Expression),
    Expression(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let(let_statement) => let_statement.fmt(f),
            Statement::Return(expression) => write!(f, "return {:?};", expression),
            Statement::Expression(expression) => write!(f, "{:?}", expression)
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(token::Token),
    Boolean { token: token::Token, value: bool },
    IntegerLiteral { token: token::Token, value: i64 },
    PrefixExpression { operator: String, right: Box<Expression> },
    InfixExpression { operator: String, left: Box<Expression>, right: Box<Expression> },
}


pub struct Program {
    pub statements: Vec<Statement>,
}

pub struct LetStatement {
    pub identifier_name: String,
    pub value: Expression,
}

impl Display for LetStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {} = {:?};", self.identifier_name, self.value)
    }
}