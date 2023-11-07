use tarzan::lexer;
use tarzan::token::Token;

#[test]
fn test_lexer_complete() {
    let source_code = "
    let five = 5;
    let ten = 10;

    let add = fn (x, y)
    {
        x + y;
    };

    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;

    if (5 < 10) {
        return true;
    } else {
        return false;
    }

    10 == 10;
    10 != 9;".into();

    let expected_tokens = [
        Token::Let,
        Token::Identifier { literal: "five".into() },
        Token::Assign,
        Token::Int { literal: "5".into() },
        Token::Semicolon,
        Token::Let,
        Token::Identifier { literal: "ten".into() },
        Token::Assign,
        Token::Int { literal: "10".into() },
        Token::Semicolon,
        Token::Let,
        Token::Identifier { literal: "add".into() },
        Token::Assign,
        Token::Function,
        Token::LeftParenthesis,
        Token::Identifier { literal: "x".into() },
        Token::Comma,
        Token::Identifier { literal: "y".into() },
        Token::RightParenthesis,
        Token::LeftBracket,
        Token::Identifier { literal: "x".into() },
        Token::Plus,
        Token::Identifier { literal: "y".into() },
        Token::Semicolon,
        Token::RightBracket,
        Token::Semicolon,
        Token::Let,
        Token::Identifier { literal: "result".into() },
        Token::Assign,
        Token::Identifier { literal: "add".into() },
        Token::LeftParenthesis,
        Token::Identifier { literal: "five".into() },
        Token::Comma,
        Token::Identifier { literal: "ten".into() },
        Token::RightParenthesis,
        Token::Semicolon,
        Token::Bang,
        Token::Minus,
        Token::Slash,
        Token::Asterisk,
        Token::Int { literal: "5".into() },
        Token::Semicolon,
        Token::Int { literal: "5".into() },
        Token::LessThan,
        Token::Int { literal: "10".into() },
        Token::GreaterThan,
        Token::Int { literal: "5".into() },
        Token::Semicolon,
        Token::If,
        Token::LeftParenthesis,
        Token::Int { literal: "5".into() },
        Token::LessThan,
        Token::Int { literal: "10".into() },
        Token::RightParenthesis,
        Token::LeftBracket,
        Token::Return,
        Token::True,
        Token::Semicolon,
        Token::RightBracket,
        Token::Else,
        Token::LeftBracket,
        Token::Return,
        Token::False,
        Token::Semicolon,
        Token::RightBracket,
        Token::Int { literal: "10".into() },
        Token::Equal,
        Token::Int { literal: "10".into() },
        Token::Semicolon,
        Token::Int { literal: "10".into() },
        Token::NotEqual,
        Token::Int { literal: "9".into() },
        Token::Semicolon,
        Token::Eof
    ];

    let mut lexer = lexer::new(source_code);
    for expected_token in expected_tokens {
        let token = lexer.next_token().unwrap();
        assert_eq!(expected_token, token)
    }
}