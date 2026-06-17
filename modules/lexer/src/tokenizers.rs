use std::{iter::Peekable, str::Chars};

use crate::tokens::{NesiaDataType, Token};

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
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

        match ident.as_str() {
            "function" => Token::Function,
            "return" => Token::Return,
            "print" => Token::Print,
            "string" => Token::DataType(NesiaDataType::StringDataType),
            "number" => Token::DataType(NesiaDataType::NumberDataType),
            _ => Token::Identifier(ident),
        }
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

        Token::DataType(num.parse().unwrap())
    }
}
