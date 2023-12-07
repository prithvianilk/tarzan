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
        assert_is_integer_expression(expression, "5".into());
    } else {
        panic!("statement is not an expression, got: {}", first_statement)
    }
}

fn assert_is_integer_expression(expression: &Expression, expected_literal: String) {
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
            assert_is_integer_expression(expression, test_case.expected_literal)
        } else {
            panic!("statement is not an expression containing a prefix expression, got: {}", first_statement)
        }
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