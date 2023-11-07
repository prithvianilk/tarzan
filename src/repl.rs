use std::io;
use crate::lexer;

pub fn start() {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut lexer = lexer::new(buffer);
        while let Ok(token) = lexer.next_token() {
            println!("{:?}", token);
        }
    }
}