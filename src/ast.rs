use crate::token::Token;

trait Node {
	fn token_literal(&self) -> String;
}

trait Expression {
	fn expression_node(&self) -> dyn Node;
}

/// Root node of every AST that parser produces
struct Program {
	statements: Vec<Box<dyn Node>>,
}

impl Node for Program {
	fn token_literal(&self) -> String {
		return if self.statements.is_empty() {
			String::from("")
		} else {
			self.statements.get(0).unwrap().token_literal()
		};
	}
}

struct Identifier {
	token: Token,
	value: String,
}

struct LetStatement {
	token: Token,
	name: Box<Identifier>,
	value: Box<dyn Expression>,
}

// struct Node {
// 	token: String,
// }

// impl Node {
// 	pub fn new(token: &str) -> Node {
// 		Node {
// 			token: String::from(token),
// 		}
// 	}
// }

// trait Expression {
// 	fn expression_node(&self) -> Node;
// }

// trait Statement {
// 	fn statement_node(&self) -> Node;
// }

// trait Token {
// 	fn token_literal(&self) -> Node;
// }

// struct Program {
// 	statements: Vec<Node>,
// }

// impl Token for Program {
// 	fn token_literal(&self) -> Node {
// 		return if self.statements.is_empty() {
// 			Node::new("")
// 		} else {
// 			self.statements.get(0).unwrap()
// 		};
// 	}
// }

pub struct AST {}
