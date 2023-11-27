use std::collections::HashMap;
use crate::lexer::Lexer;
use crate::ast::{Expression, LetStatement, Program, Statement};
use crate::token::Token;

type PrefixParseFunction = fn(&Parser) -> Expression;

type InfixParseFunction = fn(&Parser, Expression) -> Expression;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
    token_to_prefix_parse_functions_map: HashMap<u8, PrefixParseFunction>,
    token_to_infix_parse_functions_map: HashMap<u8, InfixParseFunction>,
}

enum Precedence {
    Lowest,
    Equals,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    fn value(&self) -> u8 {
        match self {
            Precedence::Lowest => 0,
            Precedence::Equals => 1,
            Precedence::Sum => 2,
            Precedence::Product => 3,
            Precedence::Prefix => 4,
            Precedence::Call => 5
        }
    }
}

pub fn new(lexer: Lexer) -> Parser {
    let mut parser = Parser {
        lexer,
        current_token: Token::Illegal,
        peek_token: Token::Illegal,
        errors: Vec::new(),
        token_to_prefix_parse_functions_map: HashMap::new(),
        token_to_infix_parse_functions_map: HashMap::new(),
    };

    parser.token_to_prefix_parse_functions_map.insert(
        2, |parser| { Expression::Identifier(parser.current_token.clone()) },
    );

    parser.next_token_n_times(2);

    return parser;
}

impl Parser {
    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = match self.current_token {
            Token::Eof => Token::Eof,
            _ => self.lexer.next_token().unwrap()
        }
    }

    pub fn next_token_n_times(&mut self, n: u32) {
        for _ in 0..n {
            self.next_token()
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while self.current_token != Token::Eof {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement)
            }
            self.next_token();
        }

        return Ok(Program { statements });
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        return match self.current_token.clone() {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        };
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if let Token::Identifier { literal } = self.peek_token.clone() {
            self.next_token();

            if self.peek_token.clone() != Token::Assign {
                self.add_err("Assign", self.peek_token.clone());
                return None;
            }
            self.next_token();

            let expression = self.parse_expression()?;

            return Some(Statement::Let(
                LetStatement {
                    identifier_name: literal,
                    value: expression,
                }
            ));
        }
        self.add_err("Identifier", self.peek_token.clone());
        return None;
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();
        let expression = self.parse_expression()?;
        return Some(Statement::Return(expression));
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        while self.current_token != Token::Semicolon {
            self.next_token()
        }
        let literal = String::from("dummy");
        return Some(Expression::Identifier(
            Token::Identifier { literal }
        ));
    }

    fn parse_expression_precedence(&mut self, precedence: Precedence) -> Option<Expression> {
        let prefix_parse_function =
            self.token_to_prefix_parse_functions_map.get(&self.current_token.clone().value())?;

        return Some(prefix_parse_function(self));
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression_precedence(Precedence::Lowest)?;
        if self.peek_token == Token::Semicolon {
            self.next_token();
        }
        return Some(Statement::Expression(expression));
    }

    fn add_err(&mut self, expected: &str, value: Token) {
        let message = format!("Parsing error, expected: {:?}, found: {:?}", expected, value);
        self.errors.push(message)
    }
}
