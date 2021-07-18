use std::fmt;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Illegal,
    Eof,
    Id(String),
    IntT(i32),
    Assign,
    Plus,
    Minus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Funcion,
    Let,
    Return,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Illegal => panic!("Illegal character"),
            Token::Id(id) => write!(f, "{}", id),
            Token::IntT(i) => write!(f, "{}", i),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Funcion => write!(f, "fun"),
            Token::Let => write!(f, "let"),
            Token::Return => write!(f, "return"),
            _ => panic!("Unknown character"),
        }
    }
}

pub fn build_keyword_map() -> HashMap<String, Token> {
    let mut keywords = HashMap::new();

    keywords.insert(String::from("fun"), Token::Funcion);
    keywords.insert(String::from("let"), Token::Let);
    keywords.insert(String::from("return"), Token::Return);

    keywords
}
