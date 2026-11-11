use crate::{
    kind::TokenKind,
    symbols::{SYMBOLS, lookup_symbol},
    token::Token,
};

#[derive(Debug, PartialEq, Eq)]
pub enum LexerError {
    UnknownToken(String),
    UnterminatedString,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownToken(lexeme) => write!(formatter, "unknown token `{lexeme}`"),
            Self::UnterminatedString => {
                write!(formatter, "unterminated string or character literal")
            }
        }
    }
}

impl std::error::Error for LexerError {}

pub fn tokenize(lexeme: String) -> Result<Token, LexerError> {
    let kind = match lexeme.as_str() {
        "fn" => TokenKind::Fn,
        "let" => TokenKind::Let,
        "mut" => TokenKind::Mut,
        "struct" => TokenKind::Struct,
        "class" => TokenKind::Class,
        "trait" => TokenKind::Trait,
        "impl" => TokenKind::Impl,
        "enum" => TokenKind::Enum,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "while" => TokenKind::While,
        "for" => TokenKind::For,
        "return" => TokenKind::Return,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "import" => TokenKind::Import,
        "as" => TokenKind::As,
        "print" => TokenKind::Print,
        "true" | "false" => TokenKind::Boolean,

        _ if lexeme.parse::<i64>().is_ok() => TokenKind::Integer,
        _ if lexeme.parse::<f64>().is_ok() => TokenKind::Float,

        _ if lexeme.starts_with('"') && lexeme.ends_with('"') => TokenKind::String,
        _ if lexeme.starts_with('\'') && lexeme.ends_with('\'') => TokenKind::Char,

        _ if !lexeme.is_empty() && lexeme.chars().all(|c| c.is_alphanumeric() || c == '_') => {
            TokenKind::Identifier
        }

        _ => lookup_symbol(&lexeme).ok_or_else(|| LexerError::UnknownToken(lexeme.clone()))?,
    };

    Ok(Token {
        kind,
        line: 1,
        column: 1,
        lexeme,
    })
}

pub fn lex(source: &str) -> Result<Vec<Token>, LexerError> {
    let characters: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0;
    let mut line = 1;
    let mut column = 1;

    while index < characters.len() {
        let character = characters[index];

        if character.is_whitespace() {
            advance(character, &mut index, &mut line, &mut column);
            continue;
        }

        if character == '/' && characters.get(index + 1) == Some(&'/') {
            while index < characters.len() && characters[index] != '\n' {
                advance(characters[index], &mut index, &mut line, &mut column);
            }
            continue;
        }

        let start = index;
        let token_line = line;
        let token_column = column;

        if character == '"' || character == '\'' {
            let quote = character;
            advance(character, &mut index, &mut line, &mut column);
            let mut closed = false;

            while index < characters.len() {
                let current = characters[index];
                if current == '\\' {
                    advance(current, &mut index, &mut line, &mut column);
                    if index < characters.len() {
                        advance(characters[index], &mut index, &mut line, &mut column);
                    }
                } else if current == quote {
                    advance(current, &mut index, &mut line, &mut column);
                    closed = true;
                    break;
                } else {
                    advance(current, &mut index, &mut line, &mut column);
                }
            }

            if !closed {
                return Err(LexerError::UnterminatedString);
            }
        } else if character.is_alphabetic() || character == '_' {
            while index < characters.len()
                && (characters[index].is_alphanumeric() || characters[index] == '_')
            {
                advance(characters[index], &mut index, &mut line, &mut column);
            }
        } else if character.is_ascii_digit() {
            while index < characters.len() && characters[index].is_ascii_digit() {
                advance(characters[index], &mut index, &mut line, &mut column);
            }

            if characters.get(index) == Some(&'.')
                && characters.get(index + 1).is_some_and(char::is_ascii_digit)
            {
                advance('.', &mut index, &mut line, &mut column);
                while index < characters.len() && characters[index].is_ascii_digit() {
                    advance(characters[index], &mut index, &mut line, &mut column);
                }
            }

            if matches!(characters.get(index), Some('e' | 'E')) {
                let exponent_start = index;
                let mut exponent_end = index + 1;
                if matches!(characters.get(exponent_end), Some('+' | '-')) {
                    exponent_end += 1;
                }
                if characters
                    .get(exponent_end)
                    .is_some_and(char::is_ascii_digit)
                {
                    while index <= exponent_end {
                        advance(characters[index], &mut index, &mut line, &mut column);
                    }
                    while index < characters.len() && characters[index].is_ascii_digit() {
                        advance(characters[index], &mut index, &mut line, &mut column);
                    }
                } else {
                    index = exponent_start;
                }
            }
        } else if let Some((symbol, _)) = SYMBOLS.iter().find(|(symbol, _)| {
            let symbol: Vec<char> = symbol.chars().collect();
            characters[index..].starts_with(&symbol)
        }) {
            for symbol_character in symbol.chars() {
                advance(symbol_character, &mut index, &mut line, &mut column);
            }
        } else {
            return Err(LexerError::UnknownToken(character.to_string()));
        }

        let lexeme: String = characters[start..index].iter().collect();
        let mut token = tokenize(lexeme)?;
        token.line = token_line;
        token.column = token_column;
        tokens.push(token);
    }

    Ok(tokens)
}

fn advance(character: char, index: &mut usize, line: &mut usize, column: &mut usize) {
    *index += 1;
    if character == '\n' {
        *line += 1;
        *column = 1;
    } else {
        *column += 1;
    }
}
