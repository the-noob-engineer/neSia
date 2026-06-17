pub struct ParseError {
    pub message: String,
    pub line: u32,
    pub column: u32,
}
