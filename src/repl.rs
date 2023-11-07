use std::io;
use crate::lexer;

pub fn start() {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut lexer = lexer::new(buffer);
        loop {
            let result = lexer.next_token();
            if result.is_err() {
                break;
            }
            println!("{:?}", result.unwrap());
        }
    }
}