use crate::token::TokenType;

pub struct Lexer {
    source_code: &'static str,
    current_index: usize,
}

impl Lexer {
    pub fn next_token(&mut self) -> Result<TokenType, &str> {
        let len = self.source_code.len();

        if self.current_index > len {
            return Err("source code already tokenized");
        }

        if self.current_index == len {
            self.current_index = self.current_index + 1;
            return Ok(TokenType::Eof);
        }

        let character = self.get_char(self.current_index);

        let token = match character {
            ' ' | '\n' => {
                self.current_index = self.current_index + 1;
                return self.next_token();
            }
            '(' => TokenType::LeftParenthesis,
            ')' => TokenType::RightParenthesis,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            ',' => TokenType::Comma,
            ';' => TokenType::Semicolon,
            '{' => TokenType::LeftBracket,
            '}' => TokenType::RightBracket,
            '/' => TokenType::Slash,
            '*' => TokenType::Asterisk,
            '<' => TokenType::LessThan,
            '>' => TokenType::GreaterThan,
            '=' => {
                if self.get_char(self.current_index + 1) == '=' {
                    self.current_index += 1;
                    TokenType::Equal
                } else {
                    TokenType::Assign
                }
            }
            '!' => {
                if self.get_char(self.current_index + 1) == '=' {
                    self.current_index += 1;
                    TokenType::NotEqual
                } else {
                    TokenType::Bang
                }
            }
            c if c.is_alphanumeric() => self.read_alphanumeric(),
            _ => TokenType::Illegal
        };

        self.current_index += 1;
        Ok(token)
    }

    fn get_char(&self, index: usize) -> char {
        let character_as_u32 = self.source_code.as_bytes()[index] as u32;
        char::from_u32(character_as_u32).unwrap()
    }

    fn current_char(&self) -> char {
        self.get_char(self.current_index)
    }

    fn read_alphanumeric(&mut self) -> TokenType {
        let mut literal = String::from(self.current_char());
        while self.get_char(self.current_index + 1).is_alphanumeric() {
            self.current_index += 1;
            literal.push(self.current_char());
        }

        match literal.as_str() {
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "return" => TokenType::Return,
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ if is_numeric(&literal) => TokenType::Int { literal },
            _ => TokenType::Identifier { literal },
        }
    }
}

fn is_numeric(literal: &String) -> bool {
    match literal.is_empty() {
        true => false,
        false => literal.chars().nth(0).unwrap().is_numeric()
    }
}

pub fn new(source_code: &'static str) -> Lexer {
    Lexer { source_code, current_index: 0 }
}
