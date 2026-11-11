use crate::kind::TokenKind;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}
