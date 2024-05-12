use std::array::from_fn;
use std::collections::HashMap;
use crate::lexer::Lexer;
use crate::ast::{BlockStatement, Expression, LetStatement, Program, Statement};
use crate::token::{Token, token_name, token_value};

type PrefixParseFunction = fn(&mut Parser) -> Option<Expression>;

type InfixParseFunction = fn(&mut Parser, Expression) -> Option<Expression>;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
    token_to_prefix_parse_functions_map: HashMap<i8, PrefixParseFunction>,
    token_to_infix_parse_functions_map: HashMap<i8, InfixParseFunction>,
}

pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    pub fn value(&self) -> u8 {
        match self {
            Precedence::Lowest => 0,
            Precedence::Equals => 1,
            Precedence::LessGreater => 2,
            Precedence::Sum => 3,
            Precedence::Product => 4,
            Precedence::Prefix => 5,
            Precedence::Call => 6,
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

    register_prefix_parse_functions(&mut parser);
    register_infix_parse_functions(&mut parser);

    parser.next_token_n_times(2);

    return parser;
}

fn register_prefix_parse_functions(parser: &mut Parser) {
    parser.token_to_prefix_parse_functions_map.insert(
        token_value::IDENTIFIER,
        |parser| { Some(Expression::Identifier(parser.current_token.clone())) },
    );

    parser.token_to_prefix_parse_functions_map.insert(
        token_value::BOOL,
        |parser| { parser.parse_boolean_expression() },
    );

    parser.token_to_prefix_parse_functions_map.insert(
        token_value::INT,
        |parser| { parser.parse_integer_literal_expression() },
    );

    parser.token_to_prefix_parse_functions_map.insert(
        token_value::PLUS,
        |parser| { parser.parse_prefix_expression() },
    );

    parser.token_to_prefix_parse_functions_map.insert(
        token_value::MINUS,
        |parser| { parser.parse_prefix_expression() },
    );

    parser.token_to_prefix_parse_functions_map.insert(
        token_value::BANG,
        |parser| { parser.parse_prefix_expression() },
    );

    parser.token_to_prefix_parse_functions_map.insert(
        token_value::LESS_THAN,
        |parser| { parser.parse_prefix_expression() },
    );

    parser.token_to_prefix_parse_functions_map.insert(
        token_value::LEFT_PARENTHESIS,
        |parser| { parser.parse_grouped_expression() },
    );

    parser.token_to_prefix_parse_functions_map.insert(
        token_value::IF,
        |parser| { parser.parse_if_expression() },
    );

    parser.token_to_prefix_parse_functions_map.insert(
        token_value::FUNCTION,
        |parser| { parser.parse_function_literal() },
    );
}

fn register_infix_parse_functions(parser: &mut Parser) {
    parser.token_to_infix_parse_functions_map.insert(
        token_value::PLUS,
        |parser, left| { parser.parse_infix_expression(left) },
    );

    parser.token_to_infix_parse_functions_map.insert(
        token_value::MINUS,
        |parser, left| { parser.parse_infix_expression(left) },
    );

    parser.token_to_infix_parse_functions_map.insert(
        token_value::SLASH,
        |parser, left| { parser.parse_infix_expression(left) },
    );

    parser.token_to_infix_parse_functions_map.insert(
        token_value::ASTERISK,
        |parser, left| { parser.parse_infix_expression(left) },
    );

    parser.token_to_infix_parse_functions_map.insert(
        token_value::EQUAL,
        |parser, left| { parser.parse_infix_expression(left) },
    );

    parser.token_to_infix_parse_functions_map.insert(
        token_value::NOT_EQUAL,
        |parser, left| { parser.parse_infix_expression(left) },
    );

    parser.token_to_infix_parse_functions_map.insert(
        token_value::LESS_THAN,
        |parser, left| { parser.parse_infix_expression(left) },
    );

    parser.token_to_infix_parse_functions_map.insert(
        token_value::GREATER_THAN,
        |parser, left| { parser.parse_infix_expression(left) },
    );
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
                self.add_err(token_name::ASSIGN, self.peek_token.clone());
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
        self.add_err(token_name::IDENTIFIER, self.peek_token.clone());
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

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression_precedence(Precedence::Lowest)?;
        if self.peek_token == Token::Semicolon {
            self.next_token();
        }
        return Some(Statement::Expression(expression));
    }

    fn parse_expression_precedence(&mut self, precedence: Precedence) -> Option<Expression> {
        let prefix_parse_function = self.token_to_prefix_parse_functions_map.get(&self.current_token.value());
        if None == prefix_parse_function {
            let message = format!("Parsing error, no prefix parsing function defined for {:?}", self.current_token.clone());
            self.errors.push(message);
            return None;
        }

        let mut left_expression = prefix_parse_function?(self);
        while !(self.peek_token == Token::Semicolon) && precedence.value() < self.get_peek_token_precedence().value() {
            let token_to_infix_parse_fn_map = self.token_to_infix_parse_functions_map.clone();
            let infix = token_to_infix_parse_fn_map.get(&self.peek_token.value());
            if infix == None {
                return left_expression;
            }
            self.next_token();
            left_expression = infix?(self, left_expression?);
        }

        return left_expression;
    }

    fn parse_integer_literal_expression(&mut self) -> Option<Expression> {
        if let Token::Int { literal } = self.current_token.clone() {
            let parsed_result = literal.parse::<i64>();

            if let Err(err) = parsed_result.clone() {
                let message = format!("Parsing error, could not parse: {}; {}", literal, err);
                self.errors.push(message);
            }

            return Some(Expression::IntegerLiteral {
                token: self.current_token.clone(),
                value: parsed_result.unwrap(),
            });
        }

        return None;
    }

    fn parse_boolean_expression(&mut self) -> Option<Expression> {
        let value = match self.current_token {
            Token::False => Some(false),
            Token::True => Some(true),
            _ => None
        }?;
        Some(Expression::Boolean {
            token: self.current_token.clone(),
            value,
        })
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = self.current_token.clone().literal()?;

        self.next_token();

        let right = self.parse_expression_precedence(Precedence::Prefix)?;

        return Some(Expression::PrefixExpression {
            operator,
            right: Box::new(right),
        });
    }


    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let expression = self.parse_expression_precedence(Precedence::Lowest);

        match self.peek_token == Token::RightParenthesis {
            true => {
                self.next_token();
                return expression;
            }
            false => None
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.current_token.literal()?;
        let precedence = self.get_current_token_precedence();
        self.next_token();
        let right = self.parse_expression_precedence(precedence)?;

        return Some(Expression::InfixExpression {
            operator,
            left: Box::from(left),
            right: Box::new(right),
        });
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        let current_token = self.current_token.clone();

        if self.peek_token != Token::LeftParenthesis {
            return None;
        }
        self.next_token_n_times(2);

        let condition = self.parse_expression_precedence(Precedence::Lowest)?;

        if self.peek_token != Token::RightParenthesis {
            return None;
        }
        self.next_token();

        if self.peek_token != Token::LeftBracket {
            return None;
        }
        self.next_token();

        let consequence = self.parse_block_statement();

        if self.peek_token != Token::Else {
            return Some(Expression::IfExpression {
                token: current_token,
                condition: Box::new(condition),
                consequence: Some(consequence),
                alternative: None,
            });
        }
        self.next_token();

        if self.peek_token != Token::LeftBracket {
            return None;
        }
        self.next_token();

        let alternative = self.parse_block_statement();

        return Some(Expression::IfExpression {
            token: current_token,
            condition: Box::new(condition),
            consequence: Some(consequence),
            alternative: Some(alternative),
        });
    }

    fn parse_function_literal(&mut self) -> Option<Expression> {
        let fn_token = self.current_token.clone();

        if self.peek_token != Token::LeftParenthesis {
            return None;
        }
        self.next_token();

        let parameters = self.parse_function_parameters();

        if self.peek_token != Token::LeftBracket {
            return None;
        }
        self.next_token();

        let body = self.parse_block_statement();

        return Some(Expression::Function {
            token: fn_token,
            parameters,
            body,
        });
    }

    fn parse_function_parameters(&mut self) -> Vec<Expression> {
        let mut parameters = vec![];

        if self.peek_token == Token::RightParenthesis {
            self.next_token();
            return parameters;
        }

        self.next_token();
        self.add_parameter(&mut parameters);

        while self.peek_token == Token::Comma {
            self.next_token_n_times(2);
            self.add_parameter(&mut parameters);
        }

        if self.peek_token != Token::RightParenthesis {
            let message = format!("Expected right parenthesis, got: {:?}", self.peek_token);
            self.errors.push(message);
        }
        self.next_token();

        return parameters;
    }

    fn add_parameter(&mut self, parameters: &mut Vec<Expression>) {
        let parameter = Expression::Identifier(self.current_token.clone());
        parameters.push(parameter);
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        let mut statements = Vec::new();
        self.next_token();

        while self.current_token != Token::RightBracket && self.current_token != Token::Eof {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
        }

        return BlockStatement {
            statements
        };
    }

    fn get_peek_token_precedence(&mut self) -> Precedence {
        self.peek_token.precedence().unwrap_or(Precedence::Lowest)
    }

    fn get_current_token_precedence(&mut self) -> Precedence {
        self.current_token.precedence().unwrap_or(Precedence::Lowest)
    }

    fn add_err(&mut self, expected: &str, value: Token) {
        let message = format!("Parsing error, expected: {:?}, found: {:?}", expected, value);
        self.errors.push(message)
    }
}
