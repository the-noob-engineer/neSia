pub fn ns_format(message: String) {
    println!("{message}")
}

pub fn nesia_interpret(source_code: String) -> Result<(), ns_lexer::lexer::LexerError> {
    let _tokens = ns_lexer::lexerize(source_code)?;
    Ok(())
}
