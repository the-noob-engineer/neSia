pub mod tokenizers;
pub mod tokens;

use crate::tokenizers::Tokenizer;
use crate::tokens::Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    Tokenizer::new(input).tokenize()
}
