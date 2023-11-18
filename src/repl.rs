use std::io;
use crate::lexer;

pub fn start() {
    let stdin = io::stdin();
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let mut lexer = lexer::new(buffer);
        while let Ok(token) = lexer.next_token() {
            println!("{:?}", token);
        }
    }
}