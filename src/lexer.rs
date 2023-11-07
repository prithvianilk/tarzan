use crate::token::Token;

pub struct Lexer {
    source_code: String,
    current_index: usize,
}

impl Lexer {
    pub fn next_token(&mut self) -> Result<Token, &str> {
        self.skip_whitespace();

        let len = self.source_code.len();

        if self.current_index > len {
            return Err("source code already tokenized");
        }

        if self.current_index == len {
            self.current_index = self.current_index + 1;
            return Ok(Token::Eof);
        }

        let character = self.get_char(self.current_index);

        let token = match character {
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '+' => Token::Plus,
            '-' => Token::Minus,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '{' => Token::LeftBracket,
            '}' => Token::RightBracket,
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            '=' => {
                if self.next_char() == '=' {
                    self.current_index += 1;
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            '!' => {
                if self.next_char() == '=' {
                    self.current_index += 1;
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            c if c.is_alphanumeric() => self.read_alphanumeric(),
            _ => Token::Illegal
        };

        self.current_index += 1;
        Ok(token)
    }

    fn skip_whitespace(&mut self) {
        let len = self.source_code.len();
        while self.current_index < len && self.current_char().is_ascii_whitespace() {
            self.current_index += 1
        }
    }

    fn get_char(&self, index: usize) -> char {
        let character_as_u32 = self.source_code.as_bytes()[index] as u32;
        char::from_u32(character_as_u32).unwrap()
    }

    fn current_char(&self) -> char {
        self.get_char(self.current_index)
    }

    fn next_char(&self) -> char {
        self.get_char(self.current_index + 1)
    }

    fn read_alphanumeric(&mut self) -> Token {
        let mut literal = String::from(self.current_char());
        while self.next_char().is_alphanumeric() {
            self.current_index += 1;
            literal.push(self.current_char());
        }

        match literal.as_str() {
            "if" => Token::If,
            "else" => Token::Else,
            "fn" => Token::Function,
            "let" => Token::Let,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            _ if is_numeric(&literal) => Token::Int { literal },
            _ => Token::Identifier { literal },
        }
    }
}

fn is_numeric(literal: &String) -> bool {
    literal.chars()
        .nth(0)
        .filter(|x| x.is_numeric())
        .is_some()
}

pub fn new(source_code: String) -> Lexer {
    Lexer { source_code, current_index: 0 }
}
