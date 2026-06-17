use std::{iter::Peekable, str::Chars};

use crate::tokens::{NesiaDataType, Token};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

static KEYWORDS: Lazy<HashMap<&'static str, Token>> = Lazy::new(|| {
    HashMap::from([
        ("function", Token::Function),
        ("let", Token::VariableIdentifier),
        ("return", Token::Return),
        ("print", Token::Print),
        ("string", Token::DataType(NesiaDataType::StringDataType)),
        ("number", Token::DataType(NesiaDataType::NumberDataType)),
    ])
});

fn keyword_or_identifier(ident: String) -> Token {
    KEYWORDS
        .get(ident.as_str())
        .cloned()
        .unwrap_or(Token::Identifier(ident))
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(&c) = self.chars.peek() {
            match c {
                c if c.is_whitespace() => {
                    self.chars.next();
                }

                c if c.is_ascii_digit() => {
                    tokens.push(self.read_number());
                }

                // Operators
                '=' => {
                    tokens.push(Token::Assign);
                    self.chars.next();
                }
                '>' => {
                    tokens.push(Token::GreaterThan);
                    self.chars.next();
                }
                '<' => {
                    tokens.push(Token::LessThan);
                    self.chars.next();
                }
                '!' => {
                    tokens.push(Token::NotOperator);
                    self.chars.next();
                }

                '(' => {
                    tokens.push(Token::OpenParen);
                    self.chars.next();
                }
                ')' => {
                    tokens.push(Token::CloseParen);
                    self.chars.next();
                }

                '{' => {
                    tokens.push(Token::OpenBrace);
                    self.chars.next();
                }
                '}' => {
                    tokens.push(Token::CloseBrace);
                    self.chars.next();
                }

                '[' => {
                    tokens.push(Token::OpenBracket);
                    self.chars.next();
                }
                ']' => {
                    tokens.push(Token::CloseBracket);
                    self.chars.next();
                }

                '"' => {
                    self.chars.next();
                    tokens.push(self.read_string());
                    self.chars.next();
                }

                c if c.is_alphabetic() || c == '_' => {
                    tokens.push(self.read_identifier());
                }

                _ => {
                    self.chars.next();
                }
            }
        }

        tokens
    }

    fn read_string(&mut self) -> Token {
        let mut string = String::new();

        while let Some(&c) = self.chars.peek() {
            if c == '"' {
                self.chars.next(); // consume closing quote
                break;
            }

            string.push(c);
            self.chars.next();
        }

        Token::String(string)
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        keyword_or_identifier(ident)
    }

    fn read_number(&mut self) -> Token {
        let mut num = String::new();

        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                num.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        Token::Number(num.parse().expect("expected a number"))
    }
}
