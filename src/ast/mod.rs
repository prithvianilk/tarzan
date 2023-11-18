use crate::token;

pub enum Statement {
    Let(LetStatement)
}

pub enum Expression {
    Identifier(token::Token)
}

pub struct Program {
    pub statements: Vec<Statement>,
}

pub struct LetStatement {
    pub identifier_name: String,
    pub value: Expression,
}
