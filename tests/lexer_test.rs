use tarzan::lexer;
use tarzan::token::TokenType;

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
        TokenType::Let,
        TokenType::Identifier { literal: "five".into() },
        TokenType::Assign,
        TokenType::Int { literal: "5".into() },
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Identifier { literal: "ten".into() },
        TokenType::Assign,
        TokenType::Int { literal: "10".into() },
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Identifier { literal: "add".into() },
        TokenType::Assign,
        TokenType::Function,
        TokenType::LeftParenthesis,
        TokenType::Identifier { literal: "x".into() },
        TokenType::Comma,
        TokenType::Identifier { literal: "y".into() },
        TokenType::RightParenthesis,
        TokenType::LeftBracket,
        TokenType::Identifier { literal: "x".into() },
        TokenType::Plus,
        TokenType::Identifier { literal: "y".into() },
        TokenType::Semicolon,
        TokenType::RightBracket,
        TokenType::Semicolon,
        TokenType::Let,
        TokenType::Identifier { literal: "result".into() },
        TokenType::Assign,
        TokenType::Identifier { literal: "add".into() },
        TokenType::LeftParenthesis,
        TokenType::Identifier { literal: "five".into() },
        TokenType::Comma,
        TokenType::Identifier { literal: "ten".into() },
        TokenType::RightParenthesis,
        TokenType::Semicolon,
        TokenType::Bang,
        TokenType::Minus,
        TokenType::Slash,
        TokenType::Asterisk,
        TokenType::Int { literal: "5".into() },
        TokenType::Semicolon,
        TokenType::Int { literal: "5".into() },
        TokenType::LessThan,
        TokenType::Int { literal: "10".into() },
        TokenType::GreaterThan,
        TokenType::Int { literal: "5".into() },
        TokenType::Semicolon,
        TokenType::If,
        TokenType::LeftParenthesis,
        TokenType::Int { literal: "5".into() },
        TokenType::LessThan,
        TokenType::Int { literal: "10".into() },
        TokenType::RightParenthesis,
        TokenType::LeftBracket,
        TokenType::Return,
        TokenType::True,
        TokenType::Semicolon,
        TokenType::RightBracket,
        TokenType::Else,
        TokenType::LeftBracket,
        TokenType::Return,
        TokenType::False,
        TokenType::Semicolon,
        TokenType::RightBracket,
        TokenType::Int { literal: "10".into() },
        TokenType::Equal,
        TokenType::Int { literal: "10".into() },
        TokenType::Semicolon,
        TokenType::Int { literal: "10".into() },
        TokenType::NotEqual,
        TokenType::Int { literal: "9".into() },
        TokenType::Semicolon,
        TokenType::Eof
    ];

    let mut lexer = lexer::new(source_code);
    for expected_token in expected_tokens {
        let token = lexer.next_token().unwrap();
        assert_eq!(expected_token, token)
    }
}