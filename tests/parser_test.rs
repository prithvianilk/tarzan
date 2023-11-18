use tarzan::{lexer, parser};
use tarzan::ast::Statement;
use tarzan::parser::Parser;

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
        }
    };
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