use crate::token::{build_keyword_map, BooleanT, Token};

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
    /// * `input` - A String that contains Yo source code
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
        if self.ch.is_none() {
            return Token::Eof;
        }

        self.skip_whitespace();
        let token = match self.ch {
            None => Token::Eof,
            Some('=') => self.read_binary_operator('=', Token::Assign, Token::Eq),
            Some(';') => Token::Semicolon,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some(',') => Token::Comma,
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('/') => Token::Slash,
            Some('*') => self.read_binary_operator('*', Token::Asterisk, Token::Power),
            Some('%') => Token::Modulo,
            Some('>') => self.read_binary_operator('=', Token::Gt, Token::GtEq),
            Some('<') => self.read_binary_operator('=', Token::Lt, Token::LtEq),
            Some('!') => self.read_binary_operator('=', Token::Bang, Token::NotEq),
            _ => {
                if !self.is_allowed_char() {
                    println!("Illegal character found '{:?}'", self.ch);
                    return Token::Illegal;
                }

                return match self.ch {
                    None => Token::Illegal,
                    Some(ch) => {
                        return if ch.is_alphabetic() {
                            let keywords = build_keyword_map();
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
            }
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

    fn peek(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            None
        } else {
            self.input.chars().nth(self.read_position)
        }
    }

    /// Read a string from the input. The function reads from the input, while the specified callback returns true.
    /// This function can be reused to read identifiers, integers, numbers, etc, whenever we need to read n characters as a whole
    ///
    /// # Arguments
    ///
    /// * `f` - A callback function that accepts `Lexer` struct and returns a `bool`
    fn read_string(&mut self, f: impl Fn(&mut Lexer) -> bool) -> String {
        let position = self.position;

        while f(self) {
            self.read_char();
        }

        String::from(&self.input.as_str()[position..self.position])
    }

    /// Read identifier
    fn read_identifier(&mut self) -> String {
        self.read_string(|l| l.is_allowed_char())
    }

    fn read_number(&mut self) -> i32 {
        let num_string = self.read_string(|l| l.ch.is_some() && l.ch.unwrap().is_numeric());

        match num_string.parse::<i32>() {
            Err(e) => panic!(
                "Failed to parse string to i32 - '{:?}', {:?}",
                num_string, e
            ),
            Ok(number) => number,
        }
    }

    fn read_binary_operator(
        &mut self,
        char_to_match: char,
        original_token: Token,
        expected_token: Token,
    ) -> Token {
        let next_ch = self.peek();
        return if next_ch.is_some() && next_ch.unwrap() == char_to_match {
            self.read_char();
            expected_token
        } else {
            original_token
        };
    }

    fn is_allowed_char(&self) -> bool {
        match self.ch {
            None => false,
            Some(ch) => ch.is_alphanumeric(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if ch.is_whitespace() || ch == '\t' || ch == '\n' || ch == '\r' {
                self.read_char();
            } else {
                break;
            }
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
    fn next_token_should_tokenize_arithmetic_operators() {
        let mut lexer = Lexer::new(String::from("+-*/%**"));
        let tokens: Vec<Token> = vec![
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::Slash,
            Token::Modulo,
            Token::Power,
        ];

        for token in tokens {
            assert_eq!(lexer.next_token(), token);
        }
    }

    #[test]
    fn next_token_should_tokenize_boolean_operators() {
        let mut lexer = Lexer::new(String::from("! != > < >= <= =="));
        let tokens: Vec<Token> = vec![
            Token::Bang,
            Token::NotEq,
            Token::Gt,
            Token::Lt,
            Token::GtEq,
            Token::LtEq,
            Token::Eq,
        ];

        for token in tokens {
            assert_eq!(lexer.next_token(), token);
        }
    }

    #[test]
    fn next_token_should_tokenize_true_false() {
        let mut lexer = Lexer::new(String::from("true false"));
        let tokens: Vec<Token> = vec![
            Token::BooleanT(BooleanT::True),
            Token::BooleanT(BooleanT::False),
        ];

        for token in tokens {
            assert_eq!(lexer.next_token(), token);
        }
    }

    #[test]
    fn next_token_should_tokenize_conditionals() {
        let mut lexer = Lexer::new(String::from("if else while elif"));
        let tokens: Vec<Token> = vec![Token::If, Token::Else, Token::While, Token::ElseIf];

        for token in tokens {
            assert_eq!(lexer.next_token(), token);
        }
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
