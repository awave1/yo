use crate::lexer;
use crate::lexer::Lexer;
use crate::token::Token;
use std::io;
use std::io::Write;

const PROMPT: &str = "> ";

pub fn repl() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        if input.ends_with("\n") || input.ends_with("\r\n") {
            let mut lexer = Lexer::new(input);
            let mut token = lexer.next_token();
            while token != Token::Eof {
                println!("{:?}", token);
                token = lexer.next_token();
                io::stdout().flush();
            }
        }
    }
}
