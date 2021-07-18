use crate::token::{Token, build_keyword_map};

/// Lexer
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    /// Creates a Lexer with specified input string
    ///
    /// # Arguments
    ///
    /// * `input` - A String that contains Monkey source code
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();

        lexer
    }

    /// Read the current character, advance to the next character and return a token
    pub fn next_token(&mut self) -> Token {
        let keywords = build_keyword_map();

        self.skip_whitespace();
        let token = match self.ch {
            None => Token::Eof,
            Some('=') => Token::Assign,
            Some(';') => Token::Semicolon,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some(',') => Token::Comma,
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            _ => {
                if !self.is_allowed_char() {
                    println!("Illegal character found '{:?}'", self.ch);
                    return Token::Illegal;
                }

                return match self.ch {
                    None => Token::Illegal,
                    Some(ch) => {
                        return if ch.is_alphabetic() {
                            let identifier = self.read_identifier();

                            match keywords.get(&identifier) {
                                None => Token::Id(identifier),
                                Some(keyword_token) => keyword_token.to_owned(),
                            }
                        } else if ch.is_numeric() {
                            Token::IntT(self.read_number())
                        } else {
                            Token::Illegal
                        }
                    }
                };

            },
        };

        self.read_char();

        token
    }

    /// Get next character from the input and advance to the next position.
    ///
    /// The function checks if we reached the end of the input, if that's the case,
    /// set `self.ch` to `None`, aka `NUL`, meaning we reached the end or there's no input at all
    /// Otherwise, get the character at the `self.read_position`, assign the `self.position` to `self.read_position` and advance the `self.read_position`.
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    /// Read identifier
    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while self.is_allowed_char() {
            self.read_char();
        }

        let identifier = &self.input.as_str()[position..self.position];
        String::from(identifier)
    }

    fn read_number(&mut self) -> i32 {
        let position = self.position;

        while self.ch.is_some() && self.ch.unwrap().is_numeric() {
            self.read_char();
        }

        let num_string = String::from(&self.input.as_str()[position..self.position]);

        match num_string.parse::<i32>() {
            Err(e) => panic!("Failed to parse string to i32 - '{:?}', {:?}", num_string, e),
            Ok(number) => number
        }
    }

    fn is_allowed_char(&self) -> bool {
        match self.ch {
            None => false,
            Some(ch) => ch.is_alphanumeric(),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_some() && self.ch.unwrap().is_whitespace() || self.ch.unwrap() == '\t' || self.ch.unwrap() == '\n' || self.ch.unwrap() == '\r' {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token_should_return_eof_token() {
        let mut lexer = Lexer::new(String::from(""));
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn next_token_should_return_illegal_token() {
        let mut lexer = Lexer::new(String::from("#"));
        assert_eq!(lexer.next_token(), Token::Illegal);
    }

    #[test]
    fn next_token_should_return_valid_token() {
        let mut lexer = Lexer::new(String::from("="));
        assert_eq!(lexer.next_token(), Token::Assign);
    }

    #[test]
    fn next_token_should_return_valid_tokens() {
        let mut lexer = Lexer::new(String::from("={}()+"));
        let expected_tokens: Vec<Token> = vec![
            Token::Assign,
            Token::LBrace,
            Token::RBrace,
            Token::LParen,
            Token::RParen,
            Token::Plus,
        ];

        for token in expected_tokens {
            assert_eq!(lexer.next_token(), token);
        }
    }

    #[test]
    fn next_token_should_tokenize_ids() {
        let mut lexer = Lexer::new(String::from("abc"));
        assert_eq!(lexer.next_token(), Token::Id(String::from("abc")));
    }

    #[test]
    fn next_token_should_tokenize_let() {
        let mut lexer = Lexer::new(String::from("let"));
        assert_eq!(lexer.next_token(), Token::Let);
    }

    #[test]
    fn next_token_should_tokenize_fun() {
        let mut lexer = Lexer::new(String::from("fun"));
        assert_eq!(lexer.next_token(), Token::Funcion);
    }

    #[test]
    fn next_token_should_tokenize_return() {
        let mut lexer = Lexer::new(String::from("return"));
        assert_eq!(lexer.next_token(), Token::Return);
    }

    #[test]
    fn next_token_should_tokenize_valid_code() {
        let code = String::from(
            "
            let five = 5;
            let ten = 10;

            let add = fun(x, y) {
                return x + y;
            };

            fun sub(x, y) {
                return x - y;
            }

            let addResult = add(five, ten);
            let subResult = sub(addResult, ten);
        ",
        );

        let expected_tokens: Vec<Token> = vec![
            Token::Let,
            Token::Id(String::from("five")),
            Token::Assign,
            Token::IntT(5),
            Token::Semicolon,

            Token::Let,
            Token::Id(String::from("ten")),
            Token::Assign,
            Token::IntT(10),
            Token::Semicolon,

            Token::Let,
            Token::Id(String::from("add")),
            Token::Assign,
            Token::Funcion,
            Token::LParen,
            Token::Id(String::from("x")),
            Token::Comma,
            Token::Id(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Id(String::from("x")),
            Token::Plus,
            Token::Id(String::from("y")),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,

            Token::Funcion,
            Token::Id(String::from("sub")),
            Token::LParen,
            Token::Id(String::from("x")),
            Token::Comma,
            Token::Id(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Id(String::from("x")),
            Token::Minus,
            Token::Id(String::from("y")),
            Token::Semicolon,
            Token::RBrace,

            Token::Let,
            Token::Id(String::from("addResult")),
            Token::Assign,
            Token::Id(String::from("add")),
            Token::LParen,
            Token::Id(String::from("five")),
            Token::Comma,
            Token::Id(String::from("ten")),
            Token::RParen,
            Token::Semicolon,

            Token::Let,
            Token::Id(String::from("subResult")),
            Token::Assign,
            Token::Id(String::from("sub")),
            Token::LParen,
            Token::Id(String::from("addResult")),
            Token::Comma,
            Token::Id(String::from("ten")),
            Token::RParen,
            Token::Semicolon,
        ];

        let mut lexer = Lexer::new(code);

        for token in expected_tokens {
            let t = lexer.next_token();
            assert_eq!(t, token);
        }
    }
}
