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

    pub const MINUS: i8 = 2;

    pub const BANG: i8 = 3;

    pub const ILLEGAL: i8 = -1;
}

pub mod token_name {
    pub const IDENTIFIER: &str = "Identifier";

    pub const ASSIGN: &str = "Assign";
}

impl Token {
    pub fn value(self) -> i8 {
        match self {
            Token::Identifier { .. } => token_value::IDENTIFIER,
            Token::Int { .. } => token_value::INT,
            Token::Minus => token_value::MINUS,
            Token::Bang => token_value::BANG,
            _ => token_value::ILLEGAL
        }
    }

    pub fn literal(self) -> Option<String> {
        match self {
            Token::Bang => Some("!".into()),
            Token::Minus => Some("-".into()),
            _ => None
        }
    }
}
