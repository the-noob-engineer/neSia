use nesia_lexer::tokens::Token;

use crate::{ast::Node, errors::ParseError};

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) -> Result<Node, ParseError> {}
}
