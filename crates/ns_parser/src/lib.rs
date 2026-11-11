use ns_ast::{
    CallExpression, Expr, FunctionStatement, LetStatement, Literal, Parameter, PrintStatement,
    Program, ReturnStatement, Statement,
};
use ns_lexer::{kind::TokenKind, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    UnexpectedToken(String),
    UnexpectedEof,
    UnsupportedSyntax(String),
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if self.check_kind(&TokenKind::Eof) {
                break;
            }
            statements.push(self.parse_statement()?);
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.peek().kind {
            TokenKind::Let => self.parse_let_statement(),
            TokenKind::Fn => self.parse_function_statement(),
            TokenKind::Return => self.parse_return_statement(),
            TokenKind::Print => self.parse_print_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_kind(&TokenKind::Let)?;
        let name = self.expect_identifier()?;

        let type_name = if self.match_kind(&[TokenKind::Colon]) {
            Some(self.expect_identifier()?)
        } else {
            None
        };

        let initializer = if self.match_kind(&[TokenKind::Equal]) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume_optional_semicolon();

        Ok(Statement::Let(LetStatement {
            name,
            type_name,
            initializer,
        }))
    }

    fn parse_function_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_kind(&TokenKind::Fn)?;
        let name = self.expect_identifier()?;
        self.expect_kind(&TokenKind::LeftParen)?;

        let mut parameters = Vec::new();
        if !self.check_kind(&TokenKind::RightParen) {
            loop {
                let param_name = self.expect_identifier()?;
                let mut type_name = None;
                if self.match_kind(&[TokenKind::Colon]) {
                    type_name = Some(self.expect_identifier()?);
                }

                let default_value = if self.match_kind(&[TokenKind::Equal]) {
                    Some(self.parse_expression()?)
                } else {
                    None
                };

                parameters.push(Parameter {
                    name: param_name,
                    type_name,
                    default_value,
                });

                if !self.match_kind(&[TokenKind::Comma]) {
                    break;
                }
            }
        }

        self.expect_kind(&TokenKind::RightParen)?;

        let return_type = if self.match_kind(&[TokenKind::Arrow]) {
            Some(self.expect_identifier()?)
        } else {
            None
        };

        self.expect_kind(&TokenKind::LeftBrace)?;
        let mut body = Vec::new();
        while !self.check_kind(&TokenKind::RightBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }
        self.expect_kind(&TokenKind::RightBrace)?;

        Ok(Statement::Function(FunctionStatement {
            name,
            parameters,
            return_type,
            body,
        }))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_kind(&TokenKind::Return)?;
        let value = if !self.check_kind(&TokenKind::Semicolon)
            && !self.check_kind(&TokenKind::RightBrace)
        {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.consume_optional_semicolon();

        Ok(Statement::Return(ReturnStatement { value }))
    }

    fn parse_print_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_kind(&TokenKind::Print)?;
        self.expect_kind(&TokenKind::LeftParen)?;
        let expression = self.parse_expression()?;
        self.expect_kind(&TokenKind::RightParen)?;
        self.consume_optional_semicolon();

        Ok(Statement::Print(PrintStatement { expression }))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.parse_expression()?;
        self.consume_optional_semicolon();
        Ok(Statement::Expression(ns_ast::ExpressionStatement {
            expression,
        }))
    }

    fn parse_expression(&mut self) -> Result<Expr, ParserError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParserError> {
        let expr = self.parse_call()?;

        if self.match_kind(&[TokenKind::Equal]) {
            let value = self.parse_assignment()?;
            match expr {
                Expr::Identifier(name) => Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                }),
                _ => Err(ParserError::UnsupportedSyntax(
                    "assignment target must be an identifier".into(),
                )),
            }
        } else {
            Ok(expr)
        }
    }

    fn parse_call(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_kind(&[TokenKind::LeftParen]) {
                let mut arguments = Vec::new();
                if !self.check_kind(&TokenKind::RightParen) {
                    loop {
                        arguments.push(self.parse_expression()?);
                        if !self.match_kind(&[TokenKind::Comma]) {
                            break;
                        }
                    }
                }
                self.expect_kind(&TokenKind::RightParen)?;
                expr = Expr::Call(CallExpression {
                    callee: Box::new(expr),
                    arguments,
                });
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParserError> {
        let token = self.advance();

        match token.kind {
            TokenKind::Identifier => Ok(Expr::Identifier(token.lexeme)),
            TokenKind::Integer => Ok(Expr::Literal(Literal::Integer(
                token
                    .lexeme
                    .parse::<i64>()
                    .map_err(|_| ParserError::UnexpectedToken(token.lexeme))?,
            ))),
            TokenKind::Float => Ok(Expr::Literal(Literal::Float(
                token
                    .lexeme
                    .parse::<f64>()
                    .map_err(|_| ParserError::UnexpectedToken(token.lexeme))?,
            ))),
            TokenKind::String => Ok(Expr::Literal(Literal::String(
                token.lexeme.trim_matches('"').to_string(),
            ))),
            TokenKind::Boolean => Ok(Expr::Literal(Literal::Boolean(token.lexeme == "true"))),
            TokenKind::LeftParen => {
                let expr = self.parse_expression()?;
                self.expect_kind(&TokenKind::RightParen)?;
                Ok(expr)
            }
            _ => Err(ParserError::UnexpectedToken(token.lexeme)),
        }
    }

    fn expect_identifier(&mut self) -> Result<String, ParserError> {
        let token = self.peek();
        match token.kind {
            TokenKind::Identifier => {
                self.position += 1;
                Ok(token.lexeme)
            }
            _ => Err(ParserError::UnexpectedToken(token.lexeme)),
        }
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.position).cloned().unwrap_or(Token {
            kind: TokenKind::Eof,
            lexeme: String::new(),
            line: 0,
            column: 0,
        })
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens.get(self.position).cloned().unwrap_or(Token {
            kind: TokenKind::Eof,
            lexeme: String::new(),
            line: 0,
            column: 0,
        });
        self.position += 1;
        token
    }

    fn check_kind(&self, kind: &TokenKind) -> bool {
        self.peek().kind == *kind
    }

    fn match_kind(&mut self, kinds: &[TokenKind]) -> bool {
        if kinds.iter().any(|kind| self.check_kind(kind)) {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn expect_kind(&mut self, kind: &TokenKind) -> Result<(), ParserError> {
        if self.check_kind(kind) {
            self.position += 1;
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken(format!(
                "expected {:?}, found {:?}",
                kind,
                self.peek().kind
            )))
        }
    }

    fn consume_optional_semicolon(&mut self) {
        self.match_kind(&[TokenKind::Semicolon]);
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }
}

pub fn parse_tokens(tokens: Vec<Token>) -> Result<Program, ParserError> {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}

pub fn parse_source(source: &str) -> Result<Program, ParserError> {
    let tokens = ns_lexer::lexerize(source.to_owned());
    parse_tokens(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_let_and_function_calls() {
        let program = parse_source(
            "let name = \"Dawda\"\nfn display_name(arg1: string) {\n  print(\"hi\")\n}\ndisplay_name(name)",
        )
        .unwrap();

        assert_eq!(program.statements.len(), 3);
    }

    #[test]
    fn parses_documented_rust_like_syntax() {
        let program = parse_source(
            "let name: String = \"Dawda\";\n\nfn display_name(arg1: String = \"example\") -> String {\n  return \"hello\"\n}",
        )
        .unwrap();

        assert_eq!(program.statements.len(), 2);
        assert!(matches!(program.statements[0], Statement::Let(_)));
        assert!(matches!(program.statements[1], Statement::Function(_)));
    }
}
