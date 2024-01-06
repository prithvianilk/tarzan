use tarzan::{lexer, parser};
use tarzan::ast::{Statement, Expression, Program};
use tarzan::parser::Parser;
use tarzan::token::Token;

#[test]
fn test_let_statements() {
    let source_code = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
    ".into();

    let program = parse(source_code);
    assert_eq!(3, program.statements.len());

    let expected_literals = vec!["x", "y", "foobar"];

    for i in 0..3 {
        let expected_literal = expected_literals[i];
        let statement = program.statements.get(i).unwrap();

        match statement {
            Statement::Let(let_statement) => {
                assert_eq!(expected_literal, let_statement.identifier_name);
            }
            _ => panic!("statement is not a let statement, got: {}", statement)
        }
    };
}

fn parse(source_code: String) -> Program {
    let lexer = lexer::new(source_code);
    let mut parser = parser::new(lexer);
    let program = parser.parse().unwrap();
    assert_zero_parser_errors(&parser);
    return program;
}

#[test]
fn test_return_statements() {
    let source_code = "
        return 5;
        return 10;
        return 993322;
    ".into();

    let program = parse(source_code);
    assert_eq!(3, program.statements.len());

    let expected_literals = vec!["5", "10", "993322"];

    for i in 0..3 {
        let expected_literal = expected_literals[i];
        let statement = program.statements.get(i).unwrap();

        match statement {
            Statement::Return(_) => {}
            _ => panic!("statement is not a return statement, got: {}", statement)
        }
    };
}

#[test]
fn test_identifier_expression() {
    let source_code = "foobar;".into();
    let program = parse(source_code);
    assert_eq!(1, program.statements.len());

    let first_statement = program.statements.get(0).unwrap();
    if let Statement::Expression(Expression::Identifier(token)) = first_statement {
        assert_eq!(token, &Token::Identifier { literal: "foobar".into() })
    } else {
        panic!("statement is not an expression containing an identifier, got: {}", first_statement)
    }
}

#[test]
fn test_integer_literal_expression() {
    let source_code = "5;".into();
    let program = parse(source_code);
    assert_eq!(1, program.statements.len());

    let first_statement = program.statements.get(0).unwrap();
    if let Statement::Expression(expression) = first_statement {
        assert_is_integer_expression("5".into(), expression);
    } else {
        panic!("statement is not an expression, got: {}", first_statement)
    }
}

fn assert_is_integer_expression(expected_literal: String, expression: &Expression) {
    let expected_value = expected_literal.parse::<i64>().unwrap();

    if let Expression::IntegerLiteral { token, value } = expression {
        assert_eq!(&Token::Int { literal: expected_literal }, token);
        assert_eq!(&expected_value, value);
    } else {
        panic!("expression is not of type integer literal, got: {:?}", expression)
    }
}

#[test]
fn test_prefix_expressions() {
    struct PrefixExpressionTestCase {
        source_code: String,
        operator: String,
        expected_literal: String,
    }

    let test_cases = vec![
        PrefixExpressionTestCase {
            source_code: "!5;".into(),
            operator: "!".into(),
            expected_literal: "5".into(),
        },
        PrefixExpressionTestCase {
            source_code: "-15;".into(),
            operator: "-".into(),
            expected_literal: "15".into(),
        },
    ];

    for test_case in test_cases {
        let program = parse(test_case.source_code);
        assert_eq!(1, program.statements.len());

        let first_statement = program.statements.get(0).unwrap();
        if let Statement::Expression(Expression::PrefixExpression { right: expression, operator }) = first_statement {
            assert_eq!(&test_case.operator, operator);
            assert_is_integer_expression(test_case.expected_literal, expression);
        } else {
            panic!("statement is not an expression containing a prefix expression, got: {}", first_statement);
        }
    }
}

#[test]
fn test_prefix_boolean_expressions() {
    struct PrefixExpressionTestCase {
        source_code: String,
        operator: String,
        expected_token: Token,
    }

    let test_cases = vec![
        PrefixExpressionTestCase {
            source_code: "!false;".into(),
            operator: "!".into(),
            expected_token: Token::False,
        },
        PrefixExpressionTestCase {
            source_code: "!true;".into(),
            operator: "!".into(),
            expected_token: Token::True,
        },
    ];

    for test_case in test_cases {
        let program = parse(test_case.source_code);
        assert_eq!(1, program.statements.len());

        let first_statement = program.statements.get(0).unwrap();
        if let Statement::Expression(Expression::PrefixExpression { right: expression, operator }) = first_statement {
            assert_eq!(&test_case.operator, operator);
            assert_is_boolean_expression(test_case.expected_token, expression);
        } else {
            panic!("statement is not an expression containing a prefix expression, got: {}", first_statement);
        }
    }
}

fn assert_is_boolean_expression(expected_token: Token, expression: &Expression) {
    match expression {
        Expression::Boolean { token, .. } => {
            assert_eq!(expected_token, *token)
        }
        _ => panic!("expression is not of boolean type, got: {:?}", expression)
    }
}

#[test]
fn test_infix_expressions() {
    #[derive(Debug)]
    struct InfixExpressionTestCase {
        source_code: String,
        expected_expression_string: String,
    }

    let test_cases = vec![
        InfixExpressionTestCase {
            source_code: "5 + 5;".into(),
            expected_expression_string: "InfixExpression { \
            operator: \"+\", \
            left: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 }, \
            right: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 } }".into(),
        },
        InfixExpressionTestCase {
            source_code: "5 - 5;".into(),
            expected_expression_string: "InfixExpression { \
            operator: \"-\", \
            left: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 }, \
            right: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 } }".into(),
        },
        InfixExpressionTestCase {
            source_code: "5 * 5;".into(),
            expected_expression_string: "InfixExpression { \
            operator: \"*\", \
            left: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 }, \
            right: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 } }".into(),
        },
        InfixExpressionTestCase {
            source_code: "5 / 5;".into(),
            expected_expression_string: "InfixExpression { \
            operator: \"/\", left: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 }, \
            right: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 } }".into(),
        },
        InfixExpressionTestCase {
            source_code: "5 > 5;".into(),
            expected_expression_string: "InfixExpression { \
            operator: \">\", \
            left: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 }, \
            right: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 } }".into(),
        },
        InfixExpressionTestCase {
            source_code: "5 < 5;".into(),
            expected_expression_string: "InfixExpression { \
            operator: \"<\", \
            left: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 }, \
            right: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 } }".into(),
        },
        InfixExpressionTestCase {
            source_code: "5 == 5;".into(),
            expected_expression_string: "InfixExpression { \
            operator: \"==\", \
            left: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 }, \
            right: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 } }".into(),
        },
        InfixExpressionTestCase {
            source_code: "5 != 5;".into(),
            expected_expression_string: "InfixExpression { \
            operator: \"!=\", \
            left: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 }, \
            right: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 } }".into(),
        },
    ];

    for test_case in test_cases {
        let program = parse(test_case.source_code);
        assert_eq!(1, program.statements.len());

        let first_statement = program.statements.get(0).unwrap();
        assert_eq!(test_case.expected_expression_string, first_statement.to_string());
    }
}

#[test]
fn test_infix_expressions_with_3_operands() {
    #[derive(Debug)]
    struct InfixExpressionTestCase {
        source_code: String,
        expected_expression_string: String,
    }

    let test_cases = vec![
        InfixExpressionTestCase {
            source_code: "1 + 2 * 3;".to_string(),
            expected_expression_string: "InfixExpression { \
            operator: \"+\", \
            left: IntegerLiteral { token: Int { literal: \"1\" }, value: 1 }, \
            right: InfixExpression { operator: \"*\", left: IntegerLiteral { token: Int { literal: \"2\" }, value: 2 }, right: IntegerLiteral { token: Int { literal: \"3\" }, value: 3 } } }".into(),
        },
        InfixExpressionTestCase {
            source_code: "1 * 2 + 3;".to_string(),
            expected_expression_string: "InfixExpression { \
            operator: \"+\", \
            left: InfixExpression { operator: \"*\", left: IntegerLiteral { token: Int { literal: \"1\" }, value: 1 }, right: IntegerLiteral { token: Int { literal: \"2\" }, value: 2 } }, \
            right: IntegerLiteral { token: Int { literal: \"3\" }, value: 3 } }".into(),
        },
        InfixExpressionTestCase {
            source_code: "1 + 2 + 3;".to_string(),
            expected_expression_string: "InfixExpression { \
            operator: \"+\", \
            left: InfixExpression { operator: \"+\", left: IntegerLiteral { token: Int { literal: \"1\" }, value: 1 }, right: IntegerLiteral { token: Int { literal: \"2\" }, value: 2 } }, \
            right: IntegerLiteral { token: Int { literal: \"3\" }, value: 3 } }".into(),
        },
    ];


    for test_case in test_cases {
        let program = parse(test_case.source_code);
        assert_eq!(1, program.statements.len());

        let first_statement = program.statements.get(0).unwrap();
        assert_eq!(test_case.expected_expression_string, first_statement.to_string());
    }
}

#[test]
fn test_boolean_parsing() {
    struct BooleanStatementTestCase {
        source_code: String,
        expected_expression_string: String,
    }

    let test_cases = vec![
        BooleanStatementTestCase {
            source_code: "true;".into(),
            expected_expression_string: "Boolean { token: True, value: true }".into(),
        },
        BooleanStatementTestCase {
            source_code: "false;".into(),
            expected_expression_string: "Boolean { token: False, value: false }".into(),
        },
        BooleanStatementTestCase {
            source_code: "3 > 5 == false;".into(),
            expected_expression_string: "InfixExpression { \
            operator: \"==\", \
            left: InfixExpression { operator: \">\", left: IntegerLiteral { token: Int { literal: \"3\" }, value: 3 }, right: IntegerLiteral { token: Int { literal: \"5\" }, value: 5 } }, \
            right: Boolean { token: False, value: false } }".into(),
        },
    ];

    for test_case in test_cases {
        let program = parse(test_case.source_code);
        assert_eq!(1, program.statements.len());

        let first_statement = program.statements.get(0).unwrap();
        assert_eq!(test_case.expected_expression_string, first_statement.to_string());
    }
}

fn assert_zero_parser_errors(parser: &Parser) {
    if parser.errors.is_empty() {
        return;
    }
    for err in parser.errors.iter() {
        println!("found error: {}", err)
    }
    panic!()
}