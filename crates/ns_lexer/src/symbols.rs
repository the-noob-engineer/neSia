use crate::kind::TokenKind;

pub const SYMBOLS: &[(&str, TokenKind)] = &[
    ("+=", TokenKind::PlusEqual),
    ("-=", TokenKind::MinusEqual),
    ("*=", TokenKind::StarEqual),
    ("/=", TokenKind::SlashEqual),
    ("==", TokenKind::EqualEqual),
    ("!=", TokenKind::BangEqual),
    ("<=", TokenKind::LessEqual),
    (">=", TokenKind::GreaterEqual),
    ("&&", TokenKind::AndAnd),
    ("||", TokenKind::OrOr),
    ("->", TokenKind::Arrow),
    ("=>", TokenKind::FatArrow),
    ("::", TokenKind::DoubleColon),
    ("+", TokenKind::Plus),
    ("-", TokenKind::Minus),
    ("*", TokenKind::Star),
    ("/", TokenKind::Slash),
    ("%", TokenKind::Percent),
    ("=", TokenKind::Equal),
    ("<", TokenKind::Less),
    (">", TokenKind::Greater),
    ("!", TokenKind::Bang),
    ("(", TokenKind::LeftParen),
    (")", TokenKind::RightParen),
    ("{", TokenKind::LeftBrace),
    ("}", TokenKind::RightBrace),
    ("[", TokenKind::LeftBracket),
    ("]", TokenKind::RightBracket),
    (",", TokenKind::Comma),
    (".", TokenKind::Dot),
    (":", TokenKind::Colon),
    (";", TokenKind::Semicolon),
];

pub fn lookup_symbol(symbol: &str) -> Option<TokenKind> {
    SYMBOLS
        .iter()
        .find(|(lexeme, _)| *lexeme == symbol)
        .map(|(_, kind)| kind.clone())
}
