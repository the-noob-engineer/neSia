pub mod kind;
pub mod lexer;
pub mod symbols;
pub mod token;

pub fn lexerize(source_code: String) -> Vec<token::Token> {
    match lexer::lex(&source_code) {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("Lexer error: {error}");
            Vec::new()
        }
    }
}

pub fn lexerize_result(source_code: String) -> Result<Vec<token::Token>, lexer::LexerError> {
    lexer::lex(&source_code)
}

#[cfg(test)]
mod tests {
    use super::{kind::TokenKind, lexerize};

    #[test]
    fn tokenizes_basic_program_with_adjacent_punctuation() {
        let source = r#"let name = "Dawda"

fn display_name(arg1: string) {
    print("Your name is {arg1}")
}

display_name(name)"#;
        let tokens = lexerize(source.to_owned());
        let lexemes: Vec<&str> = tokens.iter().map(|token| token.lexeme.as_str()).collect();

        assert_eq!(
            lexemes,
            [
                "let",
                "name",
                "=",
                "\"Dawda\"",
                "fn",
                "display_name",
                "(",
                "arg1",
                ":",
                "string",
                ")",
                "{",
                "print",
                "(",
                "\"Your name is {arg1}\"",
                ")",
                "}",
                "display_name",
                "(",
                "name",
                ")"
            ]
        );
        assert_eq!(tokens[4].kind, TokenKind::Fn);
        assert_eq!(tokens[5].kind, TokenKind::Identifier);
        assert_eq!(tokens[14].kind, TokenKind::String);
        assert_eq!((tokens[4].line, tokens[4].column), (3, 1));
    }

    #[test]
    fn recognizes_longest_operators_and_skips_line_comments() {
        let tokens = lexerize("a==b // ignored\na+=2.5e-1".to_owned());
        let lexemes: Vec<&str> = tokens.iter().map(|token| token.lexeme.as_str()).collect();

        assert_eq!(lexemes, ["a", "==", "b", "a", "+=", "2.5e-1"]);
        assert_eq!(tokens[5].kind, TokenKind::Float);
        assert_eq!(tokens[3].line, 2);
    }

    #[test]
    fn tokenizes_documented_rust_like_syntax() {
        let source = r#"let name: String = "Dawda borje Kujabi";

fn display_name(arg1: String = "example") -> String {
    return "hello"
}"#;
        let tokens = lexerize(source.to_owned());
        let lexemes: Vec<&str> = tokens.iter().map(|token| token.lexeme.as_str()).collect();

        assert_eq!(
            lexemes,
            [
                "let",
                "name",
                ":",
                "String",
                "=",
                "\"Dawda borje Kujabi\"",
                ";",
                "fn",
                "display_name",
                "(",
                "arg1",
                ":",
                "String",
                "=",
                "\"example\"",
                ")",
                "->",
                "String",
                "{",
                "return",
                "\"hello\"",
                "}"
            ]
        );
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[5].kind, TokenKind::String);
        assert_eq!(tokens[16].kind, TokenKind::Arrow);
    }
}
