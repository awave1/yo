use crate::lexer::Lexer;
use crate::token::Token;
use std::io;

pub fn repl() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.ends_with("\n") || input.ends_with("\r\n") {
                    let mut lexer = Lexer::new(input);
                    let mut token = lexer.next_token();
                    while token != Token::Eof {
                        println!("{:?}", token);
                        token = lexer.next_token();
                    }
                }
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}
