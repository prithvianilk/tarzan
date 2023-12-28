use crate::parser::Precedence;
use crate::token::token_value::BOOL;

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum Token {
    Illegal,
    Eof,
    Identifier { literal: String },
    Int { literal: String },
    Assign,
    Plus,
    Minus,
    Comma,
    Semicolon,
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    Return,
    Function,
    Let,
    False,
    True,
    Slash,
    Asterisk,
    Bang,
    If,
    Else,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,
}

pub mod token_value {
    pub const IDENTIFIER: i8 = 0;
    pub const INT: i8 = 1;
    pub const PLUS: i8 = 2;
    pub const MINUS: i8 = 3;
    pub const ASTERISK: i8 = 4;
    pub const SLASH: i8 = 5;
    pub const BANG: i8 = 6;
    pub const EQUAL: i8 = 7;
    pub const NOT_EQUAL: i8 = 8;
    pub const LESS_THAN: i8 = 9;
    pub const GREATER_THAN: i8 = 10;
    pub const BOOL: i8 = 11;
    pub const ILLEGAL: i8 = -1;
}

pub mod token_name {
    pub const IDENTIFIER: &str = "Identifier";

    pub const ASSIGN: &str = "Assign";
}

impl Token {
    pub fn value(&mut self) -> i8 {
        match self {
            Token::Identifier { .. } => token_value::IDENTIFIER,
            Token::Int { .. } => token_value::INT,
            Token::Plus => token_value::PLUS,
            Token::Minus => token_value::MINUS,
            Token::Asterisk => token_value::ASTERISK,
            Token::Slash => token_value::SLASH,
            Token::Bang => token_value::BANG,
            Token::Equal => token_value::EQUAL,
            Token::NotEqual => token_value::NOT_EQUAL,
            Token::LessThan => token_value::LESS_THAN,
            Token::GreaterThan => token_value::GREATER_THAN,
            Token::True | Token::False => BOOL,
            _ => token_value::ILLEGAL,
        }
    }

    pub fn literal(&mut self) -> Option<String> {
        match self {
            Token::Bang => Some("!".into()),
            Token::Minus => Some("-".into()),
            Token::Plus => Some("+".into()),
            Token::Asterisk => Some("*".into()),
            Token::Slash => Some("/".into()),
            Token::LessThan => Some("<".into()),
            Token::GreaterThan => Some(">".into()),
            Token::Assign => Some("=".into()),
            Token::Equal => Some("==".into()),
            Token::NotEqual => Some("!=".into()),
            _ => None
        }
    }

    pub fn precedence(&mut self) -> Option<Precedence> {
        return match self {
            Token::Equal | Token::NotEqual => Some(Precedence::Equals),
            Token::LessThan | Token::GreaterThan => Some(Precedence::LessGreater),
            Token::Plus | Token::Minus => Some(Precedence::Sum),
            Token::Slash | Token::Asterisk => Some(Precedence::Prefix),
            _ => None
        };
    }
}
