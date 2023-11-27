use tarzan::{lexer, parser};
use tarzan::ast::{Statement, Expression};
use tarzan::parser::Parser;
use tarzan::token::Token;

#[test]
fn test_let_statements() {
    let source_code = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
    ".into();

    let lexer = lexer::new(source_code);
    let mut parser = parser::new(lexer);
    let program = parser.parse().unwrap();

    assert_zero_parser_errors(&parser);
    assert_eq!(3, program.statements.len());

    let expected_literals = vec!["x", "y", "foobar"];

    for i in 0..3 {
        let expected_literal = expected_literals[i];
        let statement = program.statements.get(i).unwrap();

        match statement {
            Statement::Let(let_statement) => {
                assert_eq!(expected_literal, let_statement.identifier_name);
            }
            _ => panic!("statement is not a let statement")
        }
    };
}

#[test]
fn test_return_statements() {
    let source_code = "
        return 5;
        return 10;
        return 993322;
    ".into();

    let lexer = lexer::new(source_code);
    let mut parser = parser::new(lexer);
    let program = parser.parse().unwrap();

    assert_zero_parser_errors(&parser);
    assert_eq!(3, program.statements.len());

    let expected_literals = vec!["5", "10", "993322"];

    for i in 0..3 {
        let expected_literal = expected_literals[i];
        let statement = program.statements.get(i).unwrap();

        match statement {
            Statement::Return(_) => {}
            _ => panic!("statement is not a return statement")
        }
    };
}

#[test]
fn test_identifier_expression() {
    let source_code = "foobar;".into();
    let lexer = lexer::new(source_code);
    let mut parser = parser::new(lexer);
    let program = parser.parse().unwrap();

    assert_zero_parser_errors(&parser);
    assert_eq!(1, program.statements.len());

    if let Statement::Expression(Expression::Identifier(token)) = program.statements.get(0).unwrap() {
        assert_eq!(token, &Token::Identifier { literal: "foobar".into() })
    } else {
        panic!("statement is not an expression")
    }
}

fn assert_zero_parser_errors(parser: &Parser) {
    if parser.errors.is_empty() {
        return;
    }
    for err in parser.errors.iter() {
        println!("{}", err)
    }
    panic!()
}