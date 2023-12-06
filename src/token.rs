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

    pub const ILLEGAL: i8 = -1;
}

impl Token {
    pub fn value(self) -> i8 {
        match self {
            Token::Identifier { .. } => token_value::IDENTIFIER,
            Token::Int { .. } => token_value::INT,
            _ => token_value::ILLEGAL
        }
    }
}
