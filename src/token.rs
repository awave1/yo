use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum BooleanT {
    True,
    False,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Illegal,
    Eof,
    Id(String),
    IntT(i32),
    BooleanT(BooleanT),
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Modulo,
    Power,
    Bang,
    Gt,
    Lt,
    GtEq,
    LtEq,
    Eq,
    NotEq,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Funcion,
    Let,
    Return,
    If,
    Else,
    ElseIf,
    While,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Illegal => panic!("Illegal character"),
            Token::Id(id) => write!(f, "{}", id),
            Token::IntT(i) => write!(f, "{}", i),
            Token::BooleanT(b) => write!(f, "{}", b),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Slash => write!(f, "/"),
            Token::Asterisk => write!(f, "*"),
            Token::Modulo => write!(f, "%"),
            Token::Power => write!(f, "**"),
            Token::Gt => write!(f, ">"),
            Token::Lt => write!(f, "<"),
            Token::GtEq => write!(f, ">="),
            Token::LtEq => write!(f, "<="),
            Token::Eq => write!(f, "=="),
            Token::Bang => write!(f, "!"),
            Token::NotEq => write!(f, "!="),
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

impl fmt::Display for BooleanT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BooleanT::True => write!(f, "true"),
            BooleanT::False => write!(f, "false"),
        }
    }
}

pub fn build_keyword_map() -> HashMap<String, Token> {
    let mut keywords = HashMap::new();

    keywords.insert(String::from("fun"), Token::Funcion);
    keywords.insert(String::from("let"), Token::Let);
    keywords.insert(String::from("return"), Token::Return);
    keywords.insert(String::from("true"), Token::BooleanT(BooleanT::True));
    keywords.insert(String::from("false"), Token::BooleanT(BooleanT::False));
    keywords.insert(String::from("if"), Token::If);
    keywords.insert(String::from("else"), Token::Else);
    keywords.insert(String::from("elif"), Token::ElseIf);
    keywords.insert(String::from("while"), Token::While);

    keywords
}
