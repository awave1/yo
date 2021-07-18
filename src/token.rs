use std::fmt;

#[derive(Eq, PartialEq, Debug)]
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
            _ => panic!("Unknown character"),
        }
    }
}

// impl PartialEq for Token {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             ()
//         }
//     }
//
//     fn ne(&self, other: &Self) -> bool {
//         todo!()
//     }
// }
