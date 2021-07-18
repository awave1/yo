use crate::token::Token;

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

    /// Read the current character, advance to the next character and return a token
    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            None => Token::Eof,
            Some('=') => Token::Assign,
            Some(';') => Token::Semicolon,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some(',') => Token::Semicolon,
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            _ => Token::Illegal,
        };

        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn next_token_should_return_eof_token() {
        let mut  lexer = Lexer::new(String::from(""));
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
        let tokens: Vec<Token> = vec![
            Token::Assign,
            Token::LBrace,
            Token::RBrace,
            Token::LParen,
            Token::RParen,
            Token::Plus
        ];

        for token in tokens {
            assert_eq!(lexer.next_token(), token);
        }
    }
}
