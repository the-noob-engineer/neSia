#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Keywords
    Fn,
    Let,
    Mut,
    Struct,
    Class,
    Trait,
    Impl,
    Enum,
    If,
    Else,
    While,
    For,
    Return,
    Break,
    Continue,
    Import,
    As,
    Print,

    // Identifiers and literals
    Identifier,
    Integer,
    Float,
    String,
    Char,
    Boolean,

    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    // Assignment operators
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,

    // Comparison operators
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical operators
    AndAnd,
    OrOr,
    Bang,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // Punctuation
    Comma,
    Dot,
    Colon,
    Semicolon,
    Arrow,
    FatArrow,
    DoubleColon,

    // Special
    Eof,
}
