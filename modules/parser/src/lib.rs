use nesia_lexer::tokens::Token;

use crate::{ast::Node, errors::ParseError};

pub mod ast;
pub mod errors;
pub mod parser;

pub fn parse_tokens(tokens: Vec<Token>) -> Result<Node, ParseError> {
    let parser = parser::Parser::new(tokens);
    parser.parse()
}
